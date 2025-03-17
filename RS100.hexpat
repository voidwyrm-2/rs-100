#pragma author Nuclear Pasta
#pragma description RS-100 executable

import std.mem;
import std.core;
import std.io;

// I use bid endian because it feels more intuitive
std::core::set_endian(std::mem::Endian::Big);

enum OpcodeEnum : u8 {
    NOP,
    MOV,
    SWP,
    SAV,
    ADD,
    SUB,
    NEG,
    JMP,
    JEZ,
    JNZ,
    JGZ,
    JLZ,
    JRO
};

bitfield Opcode {
    bool a : 1;
    bool b : 1;
    bool c : 1;
    bool literal : 1;
    OpcodeEnum opcode : 4;
};

bitfield JumpArg {
    u32 address : 24;
};

struct Instruction {
    Opcode opcode [[color("00CC66")]];
    if (opcode.opcode == OpcodeEnum::NOP) {
        u8 nopArg[3] [[hidden]];
    } else if (opcode.opcode >= OpcodeEnum::JMP) {
        JumpArg arg [[color("80B3FF")]];
    } else {
        u16 argA [[color("80B3FF")]];
        u8 argB [[color("C299DD")]];
    }
};

struct Instructions {
    Instruction instructions[while(!std::mem::eof())];
};

Instructions ins @ 0;