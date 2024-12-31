//spec of the lc2k assembly language can be found at https://eecs370.github.io/project_1_spec/

use std::fmt;

#[derive(Debug, Clone)]
pub enum Reg {
    R0, // not enforced, but no one should change register 0 value
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match &self {
            Reg::R0 => 0,
            Reg::R1 => 1,
            Reg::R2 => 2,
            Reg::R3 => 3,
            Reg::R4 => 4,
            Reg::R5 => 5,
            Reg::R6 => 6,
            Reg::R7 => 7,
        };
        write!(f, "{:<8}", val)
    }
}

#[derive(Debug, Clone)]
pub enum Offset {
    Symbolic(String),
    Numeric(i8),
}

#[derive(Debug, Clone)]
pub struct RArgs {
    pub reg_a: Reg,
    pub reg_b: Reg,
    pub dest_reg: Reg,
}

#[derive(Debug, Clone)]
pub struct LArgs {
    pub reg_a: Reg,
    pub reg_b: Reg,
    pub addr: Offset,
}

#[derive(Debug, Clone)]
pub struct JArgs {
    pub reg_a: Reg,
    pub reg_b: Reg,
}

#[derive(Debug, Clone)]
pub enum Instr {
    Add(RArgs),
    Nor(RArgs),
    Lw(LArgs),
    Sw(LArgs),
    Beq(LArgs),
    Jalr(JArgs),
    Fill(Offset),
    Noop,
    Halt,
}

#[derive(Debug, Clone)]
pub struct AsmLine {
    pub label: String,
    pub instr: Instr,
}

pub fn addr_to_string(addr: &Offset) -> String {
    let s = match addr {
        Offset::Symbolic(s) => s.to_string(),
        Offset::Numeric(n) => n.to_string(),
    };
    return format!("{:<8}", s);
}

pub fn r_arg_to_string(arg: &RArgs) -> String {
    return format!("{:<8}{:<8}{:<8}", arg.reg_a, arg.reg_b, arg.dest_reg);
}

pub fn l_arg_to_string(arg: &LArgs) -> String {
    return format!(
        "{:<8}{:<8}{:<8}",
        arg.reg_a,
        arg.reg_b,
        addr_to_string(&arg.addr)
    );
}

pub fn j_arg_to_string(arg: &JArgs) -> String {
    return format!("{:<8}{:<8}", arg.reg_a, arg.reg_b);
}

pub fn output_instr(instr: &Instr) -> String {
    return match instr {
        Instr::Add(s) => format!("{:<8}{}", "add", r_arg_to_string(s)),
        Instr::Nor(s) => format!("{:<8}{}", "nor", r_arg_to_string(s)),
        Instr::Lw(s) => format!("{:<8}{}", "lw", l_arg_to_string(s)),
        Instr::Sw(s) => format!("{:<8}{}", "sw", l_arg_to_string(s)),
        Instr::Beq(s) => format!("{:<8}{}", "beq", l_arg_to_string(s)),
        Instr::Jalr(s) => format!("{:<8}{}", "jalr", j_arg_to_string(s)),
        Instr::Fill(s) => format!("{:<8}{}", ".fill", addr_to_string(s)),
        Instr::Noop => format!("{:<8}", "noop"),
        Instr::Halt => format!("{:<8}", "halt"),
    };
}

//implementing independent of display trait for now
pub fn output_asm(line: &AsmLine) -> String {
    return format!("{:<8} {}", line.label, output_instr(&line.instr));
}

pub fn output_asm_lines(lines: Vec<AsmLine>) -> String {
    return lines.iter().map(output_asm).collect::<Vec<_>>().join("\n");
}
