type Addr = usize;
use crate::memory::registers::Register as Reg;

pub enum OpCodes {
    /// 0nnn
    Sys(Addr),
    /// 00E0
    Cls,
    /// 00EE
    Ret,
    /// 1nnn
    Jump(Addr),
    /// 2nnn
    Call(Addr),
    /// 3xkk
    SkipIfEq(Reg, u8),
    /// 4xkk
    SkipNotEq(Reg, u8),
    /// 5xy0
    SkipIfEqReg(Reg, Reg),
    /// 9xy0
    SkipNotEqReg(Reg, Reg),
    /// 6xkk
    LoadToReg(Reg, u8),
    /// 7xkk
    AddConstant(Reg, u8),
    /// 8xy0
    LdReg { dest: Reg, source: Reg },
    /// 8xy1
    Or(Reg, Reg),
    /// 8xy2
    And(Reg, Reg),
    /// 8xy3
    Xor(Reg, Reg),
    /// 8xy4
    Add(Reg, Reg),
    /// 8xy5
    Sub(Reg, Reg),
    /// 8xy6
    Shr(Reg, Reg),
    /// 8xy7
    SubN(Reg, Reg),
    /// 8xyE
    Shl(Reg, Reg),
    /// Annn
    LdImage(Addr),
    /// Bnnn
    JumpOffset(Addr),
    /// Cxkk
    Rnd(Reg, u8),
    /// Dxyn
    Draw(Reg, Reg, u8),
    /// Ex9E
    SkipIfKeyPressed(Reg),
    /// ExA1
    SkipIfKeyNotPressed(Reg),
    /// Fx07
    LoadDelay(Reg),
    /// Fx0A
    LoadKey(Reg),
    /// Fx15
    SetDelay(Reg),
    /// Fx18
    SetSound(Reg),
    /// Fx1E
    AddToI(Reg),
    /// Fx29
    LoadLetter(Reg),
    // missing
    // Fx33
    // Fx55
    // Fx65
}
