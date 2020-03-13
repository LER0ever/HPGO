use std::collections::{HashMap, HashSet};
use std::error::Error;

use itertools::Itertools;
use log::debug;
use petgraph::dot::{Config, Dot};
use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use rayon::prelude::*;

use crate::ir::derive::Derivation;
use crate::ir::error::DeriveError::OptionNone;
use crate::ir::hlo_ast::*;
use petgraph::visit::GetAdjacencyMatrix;

pub type node_type<'a> = (&'a str, i8);
pub type edge_type<'a> = Vec<(&'a Instruction, &'a HashMap<&'a str, i8>)>;

pub struct VarGraph2D<'a> {
    pub g: UnGraph<node_type<'a>, edge_type<'a>>,
    pub node_id: HashMap<(&'a str, i8), NodeIndex>,
    pub node_edge_cache: HashMap<
        &'a Instruction,
        Vec<(
            node_type<'a>,
            node_type<'a>,
            &'a Instruction,
            &'a HashMap<&'a str, i8>,
        )>,
    >,
    // pub edge_id: HashMap<(NodeIndex, NodeIndex), EdgeIndex>,
    pub ast: &'a HLORoot,
    pub d: &'a Derivation<'a>,
}

impl<'a> VarGraph2D<'a> {
    pub fn new(d: &'a Derivation) -> VarGraph2D<'a> {
        VarGraph2D {
            g: UnGraph::<node_type, edge_type>::new_undirected(),
            node_id: HashMap::new(),
            node_edge_cache: HashMap::new(),
            ast: d.ast.unwrap(),
            d: d,
        }
    }

    fn node_id(&mut self, name: &'a str, dim: i8) -> NodeIndex {
        if !self.node_id.contains_key(&(name, dim)) {
            self.node_id
                .insert((name, dim), self.g.add_node((name, dim)));
        }
        return self.node_id[&(name, dim)];
    }

    fn inst_to_edges(&mut self, inst: &'a Instruction) -> Result<(), Box<dyn Error>> {
        let res = self.d.derive_infer(inst)?;
        // let mut all_vars: Vec<&'a str> = inst
        //     .function
        //     .params
        //     .as_ref()
        //     .ok_or(OptionNone("inst.fn.params".into()))?
        //     .iter()
        //     .map(|x| x.name.as_str())
        //     .collect();
        // all_vars.push(inst.var_name.as_str());
        // std::hash::Hash(&res[0]);

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
                    node_type<'a>,
                    node_type<'a>,
                    &'a Instruction,
                    &'a HashMap<&'a str, i8>,
                )>>(),
        );

        Ok(())
    }

    fn update_graph_from_inst(&mut self, index: usize, i: &'a Instruction) -> bool {
        debug!("Processing inst {}", index);
        if !self.node_edge_cache.contains_key(i) {
            self.inst_to_edges(i).unwrap();
        }
        let node_edge_result: Vec<(
            node_type<'a>,
            node_type<'a>,
            &'a Instruction,
            &'a HashMap<&'a str, i8>,
        )> = self.node_edge_cache[i].iter().map(|x| x.clone()).collect();
        // TODO: the above code made a copy of the resulting vec for no good fkn reason
        for (ta, tb, tc, td) in node_edge_result {
            let a = self.node_id(ta.0, ta.1);
            let b = self.node_id(tb.0, tb.1);
            let e = self.g.find_edge(a, b);
            if e.is_none() {
                self.g.add_edge(a, b, vec![(tc, td)]);
            } else {
                let ew = self.g.edge_weight_mut(e.unwrap()).unwrap();
                ew.push((tc, td));
            }
        }

        true
    }

    fn func_to_edges(&mut self, f: &'a HLOFunction) -> bool {
        debug!("Processing fn {}", f.name);
        f.body
            .iter()
            .enumerate()
            .map(|(index, i)| self.update_graph_from_inst(index, i))
            .all(|x| x == true)
    }

    pub fn build_from_hlo(&mut self) -> Result<&UnGraph<node_type, edge_type>, Box<dyn Error>> {
        self.g.clear();
        let ok = self
            .ast
            .functions
            .iter()
            .map(|f| self.func_to_edges(f))
            .all(|x| x == true);
        match ok {
            true => Ok(&self.g),
            false => Err("Graph Construction Error".into()),
        }
    }

    pub fn build_from_function(
        &mut self,
        fn_name: &str,
    ) -> Result<&UnGraph<node_type, edge_type>, Box<dyn Error>> {
        self.g.clear();
        let ok = self
            .ast
            .functions
            .iter()
            .filter(|f| f.name == fn_name)
            .map(|f| self.func_to_edges(f))
            .all(|x| x == true);
        match ok {
            true => Ok(&self.g),
            false => Err("Graph Construction Error".into()),
        }
    }

    pub fn export_to_dot(&self) -> Result<String, Box<dyn Error>> {
        let dot = Dot::with_config(&self.g, &[Config::EdgeIndexLabel]); // Config::EdgeNoLabel
        Ok(format!("{:?}", dot))
    }
}
