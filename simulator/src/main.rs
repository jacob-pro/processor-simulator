use std::path::PathBuf;
use binutils::utils::disassemble_buffer;
use binutils::opcodes::DisassembleInfo;

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

    println!("{:?}", text_scn.data);

    let mut info = disassemble_buffer("armv6", &text_scn.data, 0)
        .unwrap_or(DisassembleInfo::empty());

    loop {
        match info.disassemble()
            .ok_or(2807)
            .map(|i| Some(i.unwrap()))
            .unwrap_or(None)
        {
            Some(instruction) => println!("{}", instruction),
            None => break,
        }
    }

}
