use crate::ir::derive::Derivation;
use crate::ir::error::DeriveError::{MetaKeyNotFound, OptionNone};
use crate::ir::hlo_ast::*;
use itertools::Itertools;
use log::debug;
use petgraph::dot::{Config, Dot};
use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use rayon::prelude::*;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::error::Error;

pub type NodeType<'a> = (&'a str, i8);
pub type EdgeColor<'a> = &'a HashMap<&'a str, i8>;
pub type EdgeTypeSingle<'a> = (&'a Instruction, EdgeColor<'a>);
pub type EdgeType<'a> = Vec<EdgeTypeSingle<'a>>;

pub struct VarGraph3D<'a> {
    pub graph: UnGraph<NodeType<'a>, EdgeType<'a>>,
    pub node_id: HashMap<(&'a str, i8), NodeIndex>,
    pub node_edge_cache: HashMap<
        &'a Instruction,
        Vec<(
            NodeType<'a>,
            NodeType<'a>,
            &'a Instruction,
            &'a HashMap<&'a str, i8>,
        )>,
    >,
    // pub edge_id: HashMap<(NodeIndex, NodeIndex), EdgeIndex>,
    pub ast: &'a HLORoot,
    pub d: &'a Derivation<'a>,

    pub fusion_inst: Vec<&'a Instruction>,
    pub fusion_map: HashMap<&'a Instruction, Vec<HashMap<&'a str, i8>>>,
}

impl<'a> VarGraph3D<'a> {
    pub fn new(d: &'a Derivation) -> VarGraph3D<'a> {
        VarGraph3D {
            graph: UnGraph::<NodeType, EdgeType>::new_undirected(),
            node_id: HashMap::new(),
            node_edge_cache: HashMap::new(),
            ast: d.ast.unwrap(),
            d: d,

            fusion_inst: vec![],
            fusion_map: HashMap::new(),
        }
    }

    /// return the node_id, create one if not exist
    pub fn node_id(&mut self, name: &'a str, dim: i8) -> NodeIndex {
        if !self.node_id.contains_key(&(name, dim)) {
            self.node_id
                .insert((name, dim), self.graph.add_node((name, dim)));
        }
        return self.node_id[&(name, dim)];
    }

    pub fn get_node_id(&self, name: &'a str, dim: i8) -> Option<NodeIndex> {
        if self.node_id.contains_key(&(name, dim)) {
            return Some(self.node_id[&(name, dim)]);
        } else {
            if !name.contains("%") {
                return self.get_node_id(format!("%{}", name).as_str(), dim);
            // return self.get_node_id(, dim);
            } else {
                return None;
            }
        }
    }

    pub fn update_node_edge_cache(
        &mut self,
        inst: &'a Instruction,
        res: &'a Vec<HashMap<&'a str, i8>>,
    ) {
        self.node_edge_cache.insert(
            inst,
            res.iter()
                .flat_map(|m| {
                    // m.keys()
                    m.keys()
                        .tuple_combinations()
                        .map(move |(a, b)| ((*a, m[a]), (*b, m[b]), inst, m))
                })
                .collect::<Vec<(
                    NodeType<'a>,
                    NodeType<'a>,
                    &'a Instruction,
                    &'a HashMap<&'a str, i8>,
                )>>(),
        );
    }

    /// given an instruction, cache every edges produced by the instruction.
    fn inst_to_edges(&mut self, inst: &'a Instruction) -> Result<(), Box<dyn Error>> {
        // defer fusion handling
        if inst.function.name == "fusion" {
            self.fusion_inst.push(inst);
            // return Ok(());
        }

        let res = self.d.derive_infer(inst)?;

        self.update_node_edge_cache(inst, res);

        Ok(())
    }

    /// take the result from inst_to_edges and update the global graph
    pub fn update_graph_from_inst(&mut self, index: usize, i: &'a Instruction) -> bool {
        debug!("Processing inst {}", index);
        if !self.node_edge_cache.contains_key(i) {
            self.inst_to_edges(i).unwrap();
        }
        let node_edge_result: Vec<(
            NodeType<'a>,
            NodeType<'a>,
            &'a Instruction,
            &'a HashMap<&'a str, i8>,
        )> = self.node_edge_cache[i].iter().map(|x| x.clone()).collect();
        // TODO: the above code made a copy of the resulting vec for no good fkn reason

        for (ta, tb, tc, td) in node_edge_result {
            // println!("[debug] edge {},{} - {},{}", ta.0, ta.1, tb.0, tb.1);
            let a = self.node_id(ta.0, ta.1);
            let b = self.node_id(tb.0, tb.1);
            let e = self.graph.find_edge(a, b);
            if e.is_none() {
                self.graph.add_edge(a, b, vec![(tc, td)]);
            } else {
                let ew = self.graph.edge_weight_mut(e.unwrap()).unwrap();
                ew.push((tc, td));
            }
        }

        true
    }

    pub fn construct_fusion_map(&mut self) -> Result<(), Box<dyn Error>> {
        let fis = self.fusion_inst.clone();
        for fi in fis {
            fi.assert_key_in_meta("calls");
            let fn_name: &'a str = fi
                .meta
                .as_ref()
                .ok_or(OptionNone("inst.meta".into()))?
                .iter()
                .find(|x| x.key == "calls")
                .ok_or(MetaKeyNotFound("calls".into()))?
                .str_value
                .as_ref()
                .unwrap();
            let F = self
                .ast
                .functions
                .iter()
                .find(|x| &x.name == fn_name)
                .unwrap();
            let return_var = &F.body[F.body.len() - 1].var_name;
            let result = self.propagate(F)?;
            let mut flattened_result: Vec<HashMap<&'a str, i8>> = vec![];
            for m in result {
                let mut flattened_map: HashMap<&'a str, i8> = HashMap::new();
                for (k, v) in m {
                    if k == return_var {
                        flattened_map.insert(&fi.var_name, v.iter().cloned().next().unwrap());
                    } else {
                        for (i, p) in F.params.iter().enumerate() {
                            if &p.name == k {
                                flattened_map.insert(
                                    &fi.function.params.as_ref().unwrap()[i].name,
                                    v.iter().cloned().next().unwrap(),
                                );
                            }
                        }
                    }
                }
                flattened_result.push(flattened_map);
            }
            self.fusion_map.insert(fi, flattened_result);

            // self.d.derive_cache.insert(fi, flattened_result);
            // self.d.derive_cache.insert(fi, flattened_result);
            // let ref_result = &self.fusion_map[fi];
            // self.update_node_edge_cache(fi, ref_result);
            // self.update_graph_from_inst(0, fi);
        }

        // fusion_map.iter().for_each(|(k, v)| {
        //     self.update_node_edge_cache(k, v);
        //     self.update_graph_from_inst(0, k);
        // });
        Ok(())
    }

    pub fn update_graph_for_fusion(&mut self) -> Result<(), Box<dyn Error>> {
        self.construct_fusion_map()?;
        // self.fusion_map.iter_mut().for_each() {
        //
        // }

        println!("Fusion Map:");
        self.fusion_map.iter().for_each(|(k, v)| {
            println!("{:?} -> {:?}", k, v);
        });

        // unimplemented!()
        Ok(())
    }

    // do graph update for every instruction in the function
    fn func_to_edges(&mut self, f: &'a HLOFunction) -> bool {
        debug!("Processing fn {}", f.name);
        f.body
            .iter()
            .enumerate()
            .map(|(index, i)| self.update_graph_from_inst(index, i))
            .all(|x| x == true)
    }

    pub fn build_from_hlo(&mut self) -> Result<&UnGraph<NodeType, EdgeType>, Box<dyn Error>> {
        self.graph.clear();
        let ok = self
            .ast
            .functions
            .iter()
            .map(|f| self.func_to_edges(f))
            .all(|x| x == true);

        match ok {
            true => Ok(&self.graph),
            false => Err("Graph Construction Error".into()),
        }
    }

    pub fn build_from_function(
        &mut self,
        fn_name: &str,
    ) -> Result<&UnGraph<NodeType, EdgeType>, Box<dyn Error>> {
        self.graph.clear();
        let ok = self
            .ast
            .functions
            .iter()
            .filter(|f| f.name == fn_name)
            .map(|f| self.func_to_edges(f))
            .all(|x| x == true);
        match ok {
            true => Ok(&self.graph),
            false => Err("Graph Construction Error".into()),
        }
    }

    pub fn export_to_dot(&self) -> Result<String, Box<dyn Error>> {
        let dot = Dot::with_config(&self.graph, &[Config::EdgeIndexLabel]); // Config::EdgeNoLabel
        Ok(format!("{:?}", dot))
    }
}
