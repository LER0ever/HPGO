use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::time::{Duration, Instant};
use HPGO::input::*;
use HPGO::ir::derive::Derivation;
use HPGO::ir::propagate::vargraph::VarGraph3D;
use HPGO::ir::*;

#[test]
fn test_ast_positional_cache() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let mut ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    ast.cache_all_positional()?;
    let now = Instant::now();
    // checks func_id and inst_local_id
    for i in 0..ast.functions.len() {
        assert_eq!(i, ast.func_id[&ast.functions[i].name]);
        for j in 0..ast.functions[i].body.len() {
            assert_eq!(j, ast.inst_local_id[&ast.functions[i].body[j]]);
        }
    }
    println!("[test]\t variable cache contains {} elements", ast.var_pos.len());
    println!(
        "[test]\t AST Positional Check {}ms",
        now.elapsed().as_millis()
    );
    Ok(())
}

#[test]
fn test_hlo_export_dot() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    let mut d = Derivation::new_with_ast(&ast);
    d.cache_all_derive(&ast);
    let mut g = VarGraph3D::new(&d);

    g.build_from_function("%fused_computation.9.clone")?;
    print!("{}", g.export_to_dot()?);
    Ok(())
    // as long as unwrap succeeds
    // println!("{:#?}", result);
}

#[test]
fn test_hlo_derive_matmul() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/matmul.json")?;
    let result = Derivation::d(&ast.functions[0].body[0])?;
    for x in &result {
        println!("a: {}, b: {}, x: {}", x["%a"], x["%b"], x["%x"]);
    }

    // println!("{:?}", result);
    assert_eq!(result.len(), 6);
    Ok(())
}

#[test]
fn test_hlo_derive_elem() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/elemwise.json")?;
    for i in &ast.functions[0].body {
        let result = Derivation::d(i)?;
        for x in &result {
            println!("{:?}", x);
        }
        println!();
    }

    Ok(())
}

#[test]
fn test_hlo_derive_reshape() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/reshape.json")?;
    let target_inst = &ast.functions[0].body[0];
    let result = Derivation::d(target_inst)?;
    for x in &result {
        println!("{:?}", x);
    }

    Ok(())
}

#[test]
fn test_hlo_derive_transpose() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/transpose.json")?;
    let target_inst = &ast.functions[0].body[0];
    let result = Derivation::d(target_inst)?;
    for x in &result {
        println!("{:?}", x);
    }

    Ok(())
}

#[test]
fn test_hlo_derive_gather() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/gather.json")?;
    let target_inst = &ast.functions[0].body[0];
    let result = Derivation::d(target_inst)?;
    for x in &result {
        println!("{:?}", x);
    }

    Ok(())
}

#[test]
fn test_hlo_derive_scatter() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/scatter.json")?;
    let target_inst = &ast.functions[0].body[0];
    let result = Derivation::d(target_inst)?;
    for x in &result {
        println!("{:?}", x);
    }

    Ok(())
}

#[test]
fn test_hlo_derive_concat() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/concat.json")?;
    let mut target_inst = &ast.functions[0].body[0];
    let result = Derivation::d(target_inst)?;
    for x in &result {
        println!("{:?}", x);
    }
    println!();

    target_inst = &ast.functions[0].body[1];
    let result = Derivation::d(target_inst)?;
    for x in &result {
        println!("{:?}", x);
    }

    Ok(())
}

#[test]
fn test_hlo_derive_broadcast() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/broadcast.json")?;
    let mut target_inst = &ast.functions[0].body[0];
    let result = Derivation::d(target_inst)?;
    for x in &result {
        println!("{:?}", x);
    }

    Ok(())
}

#[test]
fn test_hlo_derive_cache() -> Result<(), Box<dyn Error>> {
    let mut d = Derivation::new();

    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    d.cache_all_derive(&ast)?;
    print!("cache has {} entries", d.derive_cache.len());
    Ok(())
}

#[test]
fn test_hlo_propagation_fn() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    let mut d = Derivation::new_with_ast(&ast);
    d.cache_all_derive(&ast)?;
    let mut g = VarGraph3D::new(&d);
    // let fn_name = "%fused_computation.2280";
    // let fn_name = "%fused_computation.2737.clone";
    // let fn_name = "%fused_computation.2271.clone";
    let fn_name = "%fused_computation.3484.clone";
    g.build_from_function(fn_name)?;
    // let fn_name = "%cluster_0__XlaCompiledKernel_true__XlaNumConstantArgs_8315__XlaNumResourceArgs_2186_.94957.ComputeTask";
    let f = g
        .ast
        .functions
        .iter()
        .filter(|f| f.name == fn_name)
        .next()
        .unwrap();
    let result = g.propagate(f)?;
    println!("returns {} results", result.len());
    for (i, r) in result.iter().enumerate() {
        println!("{} :: {:?}", i, r);
    }

    Ok(())
}

#[test]
fn test_hlo_compute_task_dfs() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    let mut d = Derivation::new_with_ast(&ast);
    d.cache_all_derive(&ast)?;
    let mut g = VarGraph3D::new(&d);
    // let fn_name = "%fused_computation.2280";
    // let fn_name = "%fused_computation.2737.clone";
    // let fn_name = "%fused_computation.2271.clone";
    g.build_from_hlo()?;
    g.construct_fusion_map()?;
    let fn_name = "%cluster_0__XlaCompiledKernel_true__XlaNumConstantArgs_8315__XlaNumResourceArgs_2186_.94957.ComputeTask";
    let node_id = g.get_node_id("%arg3456.0", 1).unwrap();
    let result = g.propagate_bfs(
        node_id,
        &BTreeMap::new(),
        true, // true if
    )?;

    println!("returns {} results", result.len());
    for (i, r) in result.iter().enumerate() {
        println!("{} :: {:?}", i, r);
    }

    Ok(())
}
