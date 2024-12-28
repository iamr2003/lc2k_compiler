mod lc2k;
use lc2k::{AsmLine, Instr, JArgs, LArgs, Offset, RArgs, Reg};

// figure out how to do proper import later

fn main() {
    // yes this representation is a pain to write, might redefine a macro to simplify again

    let instrs: Vec<lc2k::AsmLine> = vec![
        AsmLine {
            label: "".to_string(),
            instr: Instr::Lw(LArgs {
                reg_a: Reg::R0,
                reg_b: Reg::R1,
                addr: Offset::Symbolic("five".to_string()),
            }),
        },
        AsmLine {
            label: "".to_string(),
            instr: Instr::Lw(LArgs {
                reg_a: Reg::R1,
                reg_b: Reg::R2,
                addr: Offset::Numeric(3),
            }),
        },
        AsmLine {
            label: "start".to_string(),
            instr: Instr::Add(RArgs {
                reg_a: Reg::R1,
                reg_b: Reg::R2,
                dest_reg: Reg::R1,
            }),
        },
        AsmLine {
            label: "".to_string(),
            instr: Instr::Beq(LArgs {
                reg_a: Reg::R0,
                reg_b: Reg::R1,
                addr: Offset::Numeric(2),
            }),
        },
        AsmLine {
            label: "".to_string(),
            instr: Instr::Beq(LArgs {
                reg_a: Reg::R0,
                reg_b: Reg::R0,
                addr: Offset::Symbolic("start".to_string()),
            }),
        },
        AsmLine {
            label: "".to_string(),
            instr: Instr::Noop,
        },
        AsmLine {
            label: "done".to_string(),
            instr: Instr::Halt,
        },
        AsmLine {
            label: "five".to_string(),
            instr: Instr::Fill(Offset::Numeric(5)),
        },
        AsmLine {
            label: "neg1".to_string(),
            instr: Instr::Fill(Offset::Numeric(-1)),
        },
        AsmLine {
            label: "stAddr".to_string(),
            instr: Instr::Fill(Offset::Symbolic("start".to_string())),
        },
    ];

    println!("{}", lc2k::output_asm_lines(instrs));
}
