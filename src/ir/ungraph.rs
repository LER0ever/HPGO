use std::collections::HashMap;
use std::error::Error;

use log::debug;
use petgraph::dot::{Config, Dot};
use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use rayon::prelude::*;

use crate::ir::hlo_ast::*;

pub struct VarGraph2D<'a> {
    pub g: UnGraph<(&'a str, i8), &'a Instruction>,
    pub node_id: HashMap<(&'a str, i8), NodeIndex>,
    pub ast: &'a HLORoot,
}

impl<'a> VarGraph2D<'a> {
    pub fn new(ast_root: &'a HLORoot) -> VarGraph2D<'a> {
        VarGraph2D {
            g: UnGraph::<(&'a str, i8), &'a Instruction>::new_undirected(),
            node_id: HashMap::new(),
            ast: ast_root,
        }
    }

    fn node_id(&mut self, name: &'a str) -> NodeIndex {
        if !self.node_id.contains_key(&(name, -1i8)) {
            self.node_id
                .insert((name, -1i8), self.g.add_node((name, -1i8)));
        }
        return self.node_id[&(name, -1i8)];
    }

    fn inst_to_edges(
        inst: &'a Instruction,
    ) -> Result<Vec<(&'a str, &'a str, &'a Instruction, &'a HashMap<&'a str, i8>)>, Box<dyn Error>>
    {
        unimplemented!()
    }

    fn func_to_edges(
        f: &'a HLOFunction,
    ) -> Result<Vec<(&'a str, &'a str, &'a Instruction)>, Box<dyn Error>> {
        // f.body
        //     .iter()
        //     .enumerate()
        //     .map(|(index, i)| {
        //         debug!("Processing inst {}", index);
        //         i.function
        //             .params
        //             .as_ref()
        //             .unwrap()
        //             .iter()
        //             .map(|p| (i.var_name.as_ref(), p.name.as_ref(), i))
        //             .collect()
        //     })
        //     .collect();
        unimplemented!()
    }

    pub fn build_from_hlo(
        &mut self,
    ) -> Result<&UnGraph<(&'a str, i8), &'a Instruction>, Box<dyn Error>> {
        self.g.clear();
        let ok = self
            .ast
            .functions
            .iter()
            .map(|f| {
                debug!("Processing fn {}", f.name);
                f.body
                    .iter()
                    .enumerate()
                    .map(|(index, i)| {
                        debug!("Processing inst {}", index);
                        if i.function.params.is_some() {
                            i.function.params.as_ref().unwrap().iter().for_each(|p| {
                                let a = self.node_id(i.var_name.as_ref());
                                let b = self.node_id(p.name.as_ref());
                                self.g.add_edge(a, b, i);
                            });
                        }

                        true
                    })
                    .all(|x| x == true)
            })
            .all(|x| x == true);
        match ok {
            true => Ok(&self.g),
            false => Err("Graph Construction Error".into()),
        }
    }

    pub fn build_from_function(
        &mut self,
        fn_name: &str,
    ) -> Result<&UnGraph<(&'a str, i8), &'a Instruction>, Box<dyn Error>> {
        self.g.clear();
        let ok = self
            .ast
            .functions
            .iter()
            .filter(|f| f.name == fn_name)
            .map(|f| {
                debug!("Processing fn {}", f.name);
                if f.name == fn_name {
                    f.body
                        .iter()
                        .enumerate()
                        .map(|(index, i)| {
                            debug!("Processing inst {}", index);
                            if i.function.params.is_some() {
                                i.function.params.as_ref().unwrap().iter().for_each(|p| {
                                    let a = self.node_id(i.var_name.as_ref());
                                    let b = self.node_id(p.name.as_ref());
                                    self.g.add_edge(a, b, i);
                                });
                            }

                            true
                        })
                        .all(|x| x == true);
                }
                true
            })
            .all(|x| x == true);
        match ok {
            true => Ok(&self.g),
            false => Err("Graph Construction Error".into()),
        }
    }

    pub fn export_to_dot(&self) -> Result<String, Box<dyn Error>> {
        let dot = Dot::with_config(&self.g, &[Config::EdgeNoLabel]);
        Ok(format!("{:?}", dot))
    }
}
