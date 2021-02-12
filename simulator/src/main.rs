use std::path::PathBuf;
use capstone::prelude::*;

fn example(code: &[u8]) -> CsResult<()> {
    let cs = Capstone::new()
        .arm()
        .mode(arch::arm::ArchMode::Thumb)
        .endian(capstone::Endian::Little)
        .detail(true)
        .build()?;

    let insns = cs.disasm_all(code, 0x100b8)?;
    println!("Found {} instructions", insns.len());
    for i in insns.iter() {
        println!("{}", i);
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");

    let path = PathBuf::from("../programs/basic/basic.elf");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let text_scn = match file.get_section(".text") {
        Some(s) => s,
        None => panic!("Failed to look up .text section"),
    };

    println!("{:02X?}", text_scn.data);

    if let Err(err) = example(&text_scn.data) {
        println!("Error: {}", err);
    }
}