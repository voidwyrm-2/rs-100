use crate::common::*;

#[derive(Clone)]
pub struct Emitter {
    instructions: Vec<u8>,
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter {
            instructions: Vec::new(),
        }
    }

    pub fn emit(&mut self, bytes: &[u8]) {
        self.instructions.extend_from_slice(bytes)
    }

    pub fn emit_instruction(&mut self, opcode: Opcode, src: Dest, dst: Dest) {
        self.emit(&[0, 0, src.into(), dst.into()])
    }

    pub fn emit_literal_instruction(&mut self, opcode: Opcode, lit: i16, dst: Dest) {
        let (a, b) = Num::from(lit).to_u8();
        self.emit(&[0, a, b, dst.into()])
    }
}
