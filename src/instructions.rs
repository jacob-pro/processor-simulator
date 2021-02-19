pub type Register = String;

pub fn decode_instruction(mnemonic: &str) -> i32 {
    let upper = mnemonic.to_ascii_uppercase();
    if upper.starts_with("ADC") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("ADD") {
        return 0
    }
    if upper.starts_with("ADR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("AND") {
        return 0
    }
    if upper.starts_with("ASR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("BIC") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("BKPT") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("BLX") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("BL") {
        return 0
    }
    if upper.starts_with("BX") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("B") {
        return 0
    }
    if upper.starts_with("CMN") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("CMP") {
        return 0
    }
    if upper.starts_with("CPS") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("DMB") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("DSB") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("EOR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("ISB") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("LDM") {
        return 0
    }
    if upper.starts_with("LDR") {
        return 0
    }
    if upper.starts_with("LSL") {
        return 0
    }
    if upper.starts_with("LSR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("MOV") {
        return 0
    }
    if upper.starts_with("MRS") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("MSR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("MUL") {
        return 0
    }
    if upper.starts_with("MVN") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("NOP") {
        return 0
    }
    if upper.starts_with("ORR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("POP") {
        return 0
    }
    if upper.starts_with("PUSH") {
        return 0
    }
    if upper.starts_with("REV") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("ROR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("RSB") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("SBC") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("SEV") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("ST") {
        return 0
    }
    if upper.starts_with("SUB") {
        return 0
    }
    if upper.starts_with("SVC") {
        return 0
    }
    if upper.starts_with("SXT") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("TST") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("UXT") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("WF") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("YIELD") {
        panic!("{} not yet implemented", upper);
    }
    panic!("Unrecognised Cortex-M0 mnemonic: {}", upper);
}

pub trait Instruction {
    fn execute(&self);
}

pub struct ADD {

}

pub struct AND {

}
