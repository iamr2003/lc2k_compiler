mod lc2k;
use lc2k::{Address, AsmLine, Instr, LArgs, Reg};

// figure out how to do proper import later

fn main() {
    // yes this representation is a pain to write, might redefine a macro to simplify again

    let instrs: Vec<lc2k::AsmLine> = vec![AsmLine {
        label: "".to_string(),
        instr: Instr::Lw(LArgs {
            reg_a: Reg::R0,
            reg_b: Reg::R1,
            addr: Address::Symbolic("five".to_string()),
        }),
    }];

    println!("{}", lc2k::output_asm_lines(instrs));
}
