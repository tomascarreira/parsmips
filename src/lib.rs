// TODO: Improve error (maybe use thiserror crate)
// TODO: Implement DEBUG or Display
// TODO: FP(CP1) specific opcode

const MIPSI_FUNCT_MNEMONIC: [Option<&'static str>; 64] = [
    Some("sll"),
    None,
    Some("srl"),
    Some("sra"),
    Some("sllv"),
    None,
    Some("srlv"),
    Some("srav"),
    Some("jr"),
    Some("jalr"),
    None,
    None,
    Some("syscall"),
    Some("break"),
    None,
    None,
    Some("mfhi"),
    Some("mthi"),
    Some("mflo"),
    Some("mtlo"),
    None,
    None,
    None,
    None,
    Some("mult"),
    Some("multu"),
    Some("div"),
    Some("divu"),
    None,
    None,
    None,
    None,
    Some("add"),
    Some("addu"),
    Some("sub"),
    Some("subu"),
    Some("and"),
    Some("or"),
    Some("xor"),
    Some("nor"),
    None,
    None,
    Some("slt"),
    Some("sltu"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

const MIPSI_OP_MNEMONIC: [Option<&str>; 64] = [
    Some("this should not happend"),
    Some("this should not happend"),
    Some("j"),
    Some("jal"),
    Some("beq"),
    Some("bne"),
    Some("blez"),
    Some("bgtz"),
    Some("addi"),
    Some("addiu"),
    Some("subi"),
    Some("subiu"),
    Some("andi"),
    Some("ori"),
    Some("xori"),
    Some("lui"),
    Some("this should not happend"),
    Some("this should not happend"),
    Some("this should not happend"),
    Some("this should not happend"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("lb"),
    Some("lh"),
    Some("lwl"),
    Some("lw"),
    Some("lbu"),
    Some("lhu"),
    Some("lwr"),
    None,
    Some("sb"),
    Some("sh"),
    Some("swl"),
    Some("sw"),
    None,
    None,
    Some("swr"),
    None,
    Some("this should not happend"),
    Some("this should not happend"),
    Some("this should not happend"),
    Some("this should not happend"),
    None,
    None,
    None,
    None,
    Some("this should not happend"),
    Some("this should not happend"),
    Some("this should not happend"),
    Some("this should not happend"),
    None,
    None,
    None,
    None,
];

pub enum MipsIInstr<'a> {
    Immediate {
        mnemonic: &'a str,
        op: u8,
        rs: u8,
        rt: u8,
        immediate: u16,
    },

    Jump {
        mnemonic: &'a str,
        op: u8,
        target: u32,
    },

    Register {
        mnemonic: &'a str,
        op: u8,
        rs: u8,
        rt: u8,
        rd: u8,
        shamt: u8,
        funct: u8,
    },
}

trait Instruction {}
impl Instruction for MipsIInstr<'_> {}

pub trait Decode<Instruction> {
    fn decode(&self) -> Result<Instruction, ()>;
}

impl<'b> Decode<MipsIInstr<'b>> for u32 {
    fn decode<'a>(&'a self) -> Result<MipsIInstr<'b>, ()> {
        let op = (self >> 26).try_into().unwrap();
        let rs = ((self >> 21) & 0x0000001f).try_into().unwrap();
        let rt = ((self >> 16) & 0x0000001f).try_into().unwrap();
        let rd = ((self >> 11) & 0x0000001f).try_into().unwrap();
        let shamt = ((self >> 6) & 0x0000001f).try_into().unwrap();
        let funct = (self & 0x0000003f).try_into().unwrap();
        let immediate = (self & 0x0000ffff).try_into().unwrap();
        let target = self & 0x03ffffff;

        let mnemonic = if let Some(mnemonic) = get_mnemonic(op, funct, rs, rd) {
            mnemonic
        } else {
            return Err(());
        };

        match (op, funct, rs) {
            (0x00, _, _) => Ok(MipsIInstr::Register {
                mnemonic,
                op,
                rs,
                rt,
                rd,
                shamt,
                funct,
            }),

            (0x10 | 0x11 | 0x12 | 0x13, _, 0x00 | 0x04 | 0x10) => Ok(MipsIInstr::Register {
                mnemonic,
                op,
                rs,
                rt,
                rd,
                shamt,
                funct,
            }),

            (0x02 | 0x03, _, _) => Ok(MipsIInstr::Jump {
                mnemonic,
                op,
                target,
            }),

            _ => Ok(MipsIInstr::Immediate {
                mnemonic,
                op,
                rs,
                rt,
                immediate,
            }),
        }
    }
}

fn get_mnemonic<'a>(op: u8, funct: u8, rs: u8, rd: u8) -> Option<&'a str> {
    match (op, rs, rd, funct) {
        (0x00, _, _, _) => MIPSI_FUNCT_MNEMONIC[funct as usize],
        (0x01, _, 0x00, _) => Some("bltz"),
        (0x01, _, 0x01, _) => Some("bgez"),
        (0x01, _, 0x10, _) => Some("bltzal"),
        (0x01, _, 0x11, _) => Some("bgezal"),

        (0x10, 0x00, _, _) => Some("mfc0"),
        (0x10, 0x04, _, _) => Some("mtc0"),
        (0x10, 0x80, 0x00, _) => Some("bcf0"),
        (0x10, 0x80, 0x01, _) => Some("bct0"),
        (0x10, 0x10, _, 0x01) => Some("tlbr"),
        (0x10, 0x10, _, 0x02) => Some("tlbwi"),
        (0x10, 0x10, _, 0x06) => Some("tlbwr"),
        (0x10, 0x10, _, 0x08) => Some("tlbp"),
        (0x10, 0x10, _, 0x10) => Some("rfe"),

        (0x11, 0x00, _, _) => Some("mfc1"),
        (0x11, 0x04, _, _) => Some("mtc1"),
        (0x11, 0x80, 0x00, _) => Some("bcf1"),
        (0x11, 0x80, 0x01, _) => Some("bct1"),
        (0x11, _, _, _) => Some("cop1"),

        (0x12, 0x00, _, _) => Some("mfc2"),
        (0x12, 0x04, _, _) => Some("mtc2"),
        (0x12, 0x80, 0x00, _) => Some("bcf2"),
        (0x12, 0x80, 0x01, _) => Some("bct2"),
        (0x12, _, _, _) => Some("cop2"),

        (0x13, 0x00, _, _) => Some("mfc3"),
        (0x13, 0x04, _, _) => Some("mtc3"),
        (0x13, 0x80, 0x00, _) => Some("bcf3"),
        (0x13, 0x80, 0x01, _) => Some("bct3"),
        (0x13, _, _, _) => Some("cop3"),

        (0x30, _, _, _) => Some("lwc0"),
        (0x38, _, _, _) => Some("swc0"),

        (0x31, _, _, _) => Some("lwc1"),
        (0x39, _, _, _) => Some("swc1"),

        (0x32, _, _, _) => Some("lwc2"),
        (0x3a, _, _, _) => Some("swc2"),

        (0x33, _, _, _) => Some("lwc3"),
        (0x3b, _, _, _) => Some("swc3"),

        _ => MIPSI_OP_MNEMONIC[op as usize],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn register_instruction() {
        if let MipsIInstr::Register {
            mnemonic,
            op,
            rs,
            rt,
            rd,
            shamt,
            funct,
        } = 0x00011020.decode().unwrap()
        {
            assert_eq!(
                ("add", 0x00, 0x00, 0x01, 0x02, 0x00, 0x20),
                (mnemonic, op, rs, rt, rd, shamt, funct)
            );
        } else {
            assert!(false);
        }
    }

    #[test]
    fn immediate_instruction() {
        if let MipsIInstr::Immediate { mnemonic, op, rs, rt, immediate } = 0x2246743b.decode().unwrap() {
            assert_eq!(("addi", 0x08, 0x12, 0x06, 0x743b), (mnemonic, op, rs, rt, immediate));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn jump_instruction() {
        if let MipsIInstr::Jump { mnemonic, op, target } = 0x0e0ab251.decode().unwrap() {
            assert_eq!(("jal", 0x03, 0x20ab251), (mnemonic, op, target));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn life_times() {
        let instr;
        {
            let word = 0x00011020;
            instr = word.decode().unwrap();
        }

        if let MipsIInstr::Register {
            mnemonic,
            op,
            rs,
            rt,
            rd,
            shamt,
            funct,
        } = instr
        {
            assert_eq!(
                ("add", 0x00, 0x00, 0x01, 0x02, 0x00, 0x20),
                (mnemonic, op, rs, rt, rd, shamt, funct)
            );
        } else {
            assert!(false);
        }
    }
}
