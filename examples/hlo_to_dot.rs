use std::error::Error;
use HPGO::input::*;
// use HPGO::ir::propagate::propagate::Propagate;
use HPGO::ir::propagate::vargraph::VarGraph3D;
use HPGO::ir::*;

fn main() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    let mut d = derive::Derivation::new_with_ast(&ast);
    // d.cache_all_derive(&ast)?;
    let mut g = VarGraph3D::new(&d);
    // g.build_from_function("%cluster_0__XlaCompiledKernel_true__XlaNumConstantArgs_8315__XlaNumResourceArgs_2186_.94957.ComputeTask")?;
    // g.build_from_function("%fused_computation.3484.clone")?;

    // g.build_from_hlo()?;
    // g.construct_fusion_map()?;
    // g.update_graph_for_fusion()?;

    // print!("{}", g.export_to_dot()?);
    // print!("Matrix: {:#?}", g.g.adjacency_matrix());
    Ok(())
    // as long as unwrap succeeds
    // println!("{:#?}", result);
}
