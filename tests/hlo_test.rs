use std::error::Error;
use HPGO::input::*;
use HPGO::ir::derive::Derivation;
use HPGO::ir::ungraph::VarGraph2D;
use HPGO::ir::*;

#[test]
fn test_hlo_export_dot() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    let mut g = VarGraph2D::new(&ast);
    g.build_from_function("%fused_computation.9.clone")?;
    print!("{}", g.export_to_dot()?);
    Ok(())
    // as long as unwrap succeeds
    // println!("{:#?}", result);
}

#[test]
fn test_hlo_derive_matmul() -> Result<(), Box<dyn Error>> {
    let d = Derivation::new();

    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/matmul.json")?;
    let result = d.d(&ast.functions[0].body[0])?;
    for x in &result {
        println!("a: {}, b: {}, x: {}", x["%a"], x["%b"], x["%x"]);
    }

    // println!("{:?}", result);
    assert_eq!(result.len(), 6);
    Ok(())
}

#[test]
fn test_hlo_derive_elem() -> Result<(), Box<dyn Error>> {
    let d = Derivation::new();

    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/elemwise.json")?;
    for i in &ast.functions[0].body {
        let result = d.d(i);
        for x in &result {
            println!("{:?}", result);
        }
    }

    Ok(())
}
