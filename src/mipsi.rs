use crate::{Decode, DecodeError, Register, Immediate, Jump};

#[derive(Debug, PartialEq)]
pub enum MipsI {
    Add(Register),
    Addi(Immediate),
    Addiu(Immediate),
    Addu(Register),
    And(Register),
    Andi(Immediate),
    Bc0f(Immediate),
    Bc0t(Immediate),
    Bc1f(Immediate),
    Bc1t(Immediate),
    Bc2f(Immediate),
    Bc2t(Immediate),
    Bc3f(Immediate),
    Bc3t(Immediate),
    Beq(Immediate),
    Bgez(Immediate),
    Bgezal(Immediate),
    Bgtz(Immediate),
    Blez(Immediate),
    Bltz(Immediate),
    Bltzal(Immediate),
    Bne(Immediate),
    Break(Register),
    Cfc1(Register),
    Cfc2(Register),
    Cfc3(Register),
    Cop0(Register),
    Cop1(Register),
    Cop2(Register),
    Cop3(Register),
    Ctc1(Register),
    Ctc2(Register),
    Ctc3(Register),
    Div(Register),
    Divu(Register),
    J(Jump),
    Jal(Jump),
    Jalr(Register),
    Jr(Register),
    Lb(Immediate),
    Lbu(Immediate),
    Lh(Immediate),
    Lhu(Immediate),
    Lui(Immediate),
    Lw(Immediate),
    Lwc1(Immediate),
    Lwc2(Immediate),
    Lwc3(Immediate),
    Lwl(Immediate),
    Lwr(Immediate),
    Mfc0(Register),
    Mfc1(Register),
    Mfc2(Register),
    Mfc3(Register),
    Mfhi(Register),
    Mflo(Register),
    Mtc0(Register),
    Mtc1(Register),
    Mtc2(Register),
    Mtc3(Register),
    Mthi(Register),
    Mtlo(Register),
    Mult(Register),
    Multu(Register),
    Nor(Register),
    Or(Register),
    Ori(Immediate),
    Rfe(Register),
    Sb(Immediate),
    Sh(Immediate),
    Sll(Register),
    Sllv(Register),
    Slt(Register),
    Slti(Immediate),
    Sltiu(Immediate),
    Sltu(Register),
    Sra(Register),
    Srav(Register),
    Srl(Register),
    Srlv(Register),
    Sub(Register),
    Subu(Register),
    Sw(Immediate),
    Swc1(Immediate),
    Swc2(Immediate),
    Swc3(Immediate),
    Swl(Immediate),
    Swr(Immediate),
    Syscall(Register),
    Tlbp(Register),
    Tlbr(Register),
    Tlbwi(Register),
    Tlbwr(Register),
    Xor(Register),
    Xori(Immediate),
}

impl Decode<MipsI> for u32 {
    type Error = DecodeError;

    fn decode(&self) -> Result<MipsI, Self::Error> {
        let op: u8 = (self >> 26).try_into().unwrap();
        let rs: u8 = ((self >> 21) & 0x0000001f).try_into().unwrap();
        let rt: u8 = ((self >> 16) & 0x0000001f).try_into().unwrap();
        let rd: u8 = ((self >> 11) & 0x0000001f).try_into().unwrap();
        let sa: u8 = ((self >> 6) & 0x0000001f).try_into().unwrap();
        let funct: u8 = (self & 0x0000003f).try_into().unwrap();
        let immediate: u16 = (self & 0x0000ffff).try_into().unwrap();
        let target: u32 = self & 0x03ffffff;

        match (op, rs, rd, rt, funct) {
            (0x00, _, _, _, 0x00) => Ok(MipsI::Sll(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x02) => Ok(MipsI::Srl(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x03) => Ok(MipsI::Sra(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x04) => Ok(MipsI::Sllv(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x06) => Ok(MipsI::Srlv(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x07) => Ok(MipsI::Srav(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x08) => Ok(MipsI::Jr(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x09) => Ok(MipsI::Jalr(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x0c) => Ok(MipsI::Syscall(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x0d) => Ok(MipsI::Break(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x10) => Ok(MipsI::Mfhi(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x11) => Ok(MipsI::Mthi(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x12) => Ok(MipsI::Mflo(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x13) => Ok(MipsI::Mtlo(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x18) => Ok(MipsI::Mult(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x19) => Ok(MipsI::Multu(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x1a) => Ok(MipsI::Div(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x1b) => Ok(MipsI::Divu(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x20) => Ok(MipsI::Add(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x21) => Ok(MipsI::Addu(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x22) => Ok(MipsI::Sub(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x23) => Ok(MipsI::Subu(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x24) => Ok(MipsI::And(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x25) => Ok(MipsI::Or(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x26) => Ok(MipsI::Xor(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x27) => Ok(MipsI::Nor(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x2a) => Ok(MipsI::Slt(Register { rs, rt, rd, sa })),

            (0x00, _, _, _, 0x2b) => Ok(MipsI::Sltu(Register { rs, rt, rd, sa })),

            (0x01, _, 0x00, _, _) => Ok(MipsI::Bltz(Immediate { rs, rt, immediate })),

            (0x01, _, 0x01, _, _) => Ok(MipsI::Bgez(Immediate { rs, rt, immediate })),

            (0x01, _, 0x10, _, _) => Ok(MipsI::Bltzal(Immediate { rs, rt, immediate })),

            (0x01, _, 0x11, _, _) => Ok(MipsI::Bgezal(Immediate { rs, rt, immediate })),

            (0x02, _, _, _, _) => Ok(MipsI::J(Jump { target })),

            (0x03, _, _, _, _) => Ok(MipsI::Jal(Jump { target })),

            (0x04, _, _, _, _) => Ok(MipsI::Beq(Immediate { rs, rt, immediate })),

            (0x05, _, _, _, _) => Ok(MipsI::Bne(Immediate { rs, rt, immediate })),

            (0x06, _, _, _, _) => Ok(MipsI::Blez(Immediate { rs, rt, immediate })),

            (0x07, _, _, _, _) => Ok(MipsI::Bgtz(Immediate { rs, rt, immediate })),

            (0x08, _, _, _, _) => Ok(MipsI::Addi(Immediate { rs, rt, immediate })),

            (0x09, _, _, _, _) => Ok(MipsI::Addiu(Immediate { rs, rt, immediate })),

            (0x0a, _, _, _, _) => Ok(MipsI::Slti(Immediate { rs, rt, immediate })),

            (0x0b, _, _, _, _) => Ok(MipsI::Sltiu(Immediate { rs, rt, immediate })),

            (0x0c, _, _, _, _) => Ok(MipsI::Andi(Immediate { rs, rt, immediate })),

            (0x0d, _, _, _, _) => Ok(MipsI::Ori(Immediate { rs, rt, immediate })),

            (0x0e, _, _, _, _) => Ok(MipsI::Xori(Immediate { rs, rt, immediate })),

            (0x0f, _, _, _, _) => Ok(MipsI::Lui(Immediate { rs, rt, immediate })),

            (0x10, 0x00, _, _, _) => Ok(MipsI::Mfc0(Register { rs, rt, rd, sa })),

            (0x10, 0x04, _, _, _) => Ok(MipsI::Mtc0(Register { rs, rt, rd, sa })),

            (0x10, 0x08, 0x00, _, _) => Ok(MipsI::Bc0f(Immediate { rs, rt, immediate })),

            (0x10, 0x08, 0x01, _, _) => Ok(MipsI::Bc0t(Immediate { rs, rt, immediate })),

            (0x10, 0x10, _, _, 0x01) => Ok(MipsI::Tlbr(Register { rs, rt, rd, sa })),

            (0x10, 0x10, _, _, 0x02) => Ok(MipsI::Tlbwi(Register { rs, rt, rd, sa })),

            (0x10, 0x10, _, _, 0x06) => Ok(MipsI::Tlbwr(Register { rs, rt, rd, sa })),

            (0x10, 0x10, _, _, 0x08) => Ok(MipsI::Tlbp(Register { rs, rt, rd, sa })),

            (0x10, 0x10, _, _, 0x10) => Ok(MipsI::Rfe(Register { rs, rt, rd, sa })),

            (0x10, 0x10..=0x1f, _, _, _) => Ok(MipsI::Cop0(Register { rs, rt, rd, sa })),

            (0x11, 0x00, _, _, _) => Ok(MipsI::Mfc1(Register { rs, rt, rd, sa })),

            (0x11, 0x02, _, _, _) => Ok(MipsI::Cfc1(Register { rs, rt, rd, sa })),

            (0x11, 0x04, _, _, _) => Ok(MipsI::Mtc1(Register { rs, rt, rd, sa })),

            (0x11, 0x06, _, _, _) => Ok(MipsI::Ctc1(Register { rs, rt, rd, sa })),

            (0x11, 0x08, 0x00, _, _) => Ok(MipsI::Bc1f(Immediate { rs, rt, immediate })),

            (0x11, 0x08, 0x01, _, _) => Ok(MipsI::Bc1t(Immediate { rs, rt, immediate })),

            (0x11, 0x10..=0x1f, _, _, _) => Ok(MipsI::Cop1(Register { rs, rt, rd, sa })),

            (0x12, 0x00, _, _, _) => Ok(MipsI::Mfc2(Register { rs, rt, rd, sa })),

            (0x12, 0x02, _, _, _) => Ok(MipsI::Cfc2(Register { rs, rt, rd, sa })),

            (0x12, 0x04, _, _, _) => Ok(MipsI::Mtc2(Register { rs, rt, rd, sa })),

            (0x12, 0x06, _, _, _) => Ok(MipsI::Ctc2(Register { rs, rt, rd, sa })),

            (0x12, 0x08, 0x00, _, _) => Ok(MipsI::Bc2f(Immediate { rs, rt, immediate })),

            (0x12, 0x08, 0x01, _, _) => Ok(MipsI::Bc2t(Immediate { rs, rt, immediate })),

            (0x12, 0x10..=0x1f, _, _, _) => Ok(MipsI::Cop2(Register { rs, rt, rd, sa })),

            (0x13, 0x00, _, _, _) => Ok(MipsI::Mfc3(Register { rs, rt, rd, sa })),

            (0x13, 0x02, _, _, _) => Ok(MipsI::Cfc3(Register { rs, rt, rd, sa })),

            (0x13, 0x04, _, _, _) => Ok(MipsI::Mtc3(Register { rs, rt, rd, sa })),

            (0x13, 0x06, _, _, _) => Ok(MipsI::Ctc3(Register { rs, rt, rd, sa })),

            (0x13, 0x08, 0x00, _, _) => Ok(MipsI::Bc3f(Immediate { rs, rt, immediate })),

            (0x13, 0x08, 0x01, _, _) => Ok(MipsI::Bc3t(Immediate { rs, rt, immediate })),

            (0x13, 0x10..=0x1f, _, _, _) => Ok(MipsI::Cop3(Register { rs, rt, rd, sa })),

            (0x20, _, _, _, _) => Ok(MipsI::Lb(Immediate { rs, rt, immediate })),

            (0x21, _, _, _, _) => Ok(MipsI::Lh(Immediate { rs, rt, immediate })),

            (0x22, _, _, _, _) => Ok(MipsI::Lwl(Immediate { rs, rt, immediate })),

            (0x23, _, _, _, _) => Ok(MipsI::Lw(Immediate { rs, rt, immediate })),

            (0x24, _, _, _, _) => Ok(MipsI::Lbu(Immediate { rs, rt, immediate })),

            (0x25, _, _, _, _) => Ok(MipsI::Lhu(Immediate { rs, rt, immediate })),

            (0x26, _, _, _, _) => Ok(MipsI::Lwr(Immediate { rs, rt, immediate })),

            (0x28, _, _, _, _) => Ok(MipsI::Sb(Immediate { rs, rt, immediate })),

            (0x29, _, _, _, _) => Ok(MipsI::Sh(Immediate { rs, rt, immediate })),

            (0x2a, _, _, _, _) => Ok(MipsI::Swl(Immediate { rs, rt, immediate })),

            (0x2b, _, _, _, _) => Ok(MipsI::Sw(Immediate { rs, rt, immediate })),

            (0x2e, _, _, _, _) => Ok(MipsI::Swr(Immediate { rs, rt, immediate })),

            (0x31, _, _, _, _) => Ok(MipsI::Swc1(Immediate { rs, rt, immediate })),

            (0x32, _, _, _, _) => Ok(MipsI::Swc2(Immediate { rs, rt, immediate })),

            (0x33, _, _, _, _) => Ok(MipsI::Swc3(Immediate { rs, rt, immediate })),

            (0x39, _, _, _, _) => Ok(MipsI::Lwc1(Immediate { rs, rt, immediate })),

            (0x3a, _, _, _, _) => Ok(MipsI::Lwc2(Immediate { rs, rt, immediate })),

            (0x3b, _, _, _, _) => Ok(MipsI::Lwc3(Immediate { rs, rt, immediate })),

            _ => Err(DecodeError::CannotDecodeU8 { u32: *self }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_instruction() {
        assert_eq!(
            0x3c048000.decode(),
            Ok(MipsI::Lui(Immediate {
                rs: 0,
                rt: 4,
                immediate: 0x8000
            }))
        )
    }

    #[test]
    fn decode_invalid_instruction() {
        assert_eq!(
            0x00000001.decode(),
            Err(DecodeError::CannotDecodeU8 { u32: 0x00000001 })
        );
    }
}
