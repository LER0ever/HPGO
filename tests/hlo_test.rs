use std::error::Error;
use HPGO::input::*;
use HPGO::ir::ungraph::VarGraph2D;
use HPGO::ir::*;

#[test]
fn test_hlo_export_dot() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    let mut g = VarGraph2D::new(&ast);
    g.build_from_hlo()?;
    print!("{}", g.export_to_dot()?);
    Ok(())
    // as long as unwrap succeeds
    // println!("{:#?}", result);
}
