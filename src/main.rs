use std::env;
use std::fs;
use std::io;
mod ast;
mod cimple;
mod cimple_compiler;
mod lc2k;
use cimple_compiler::compile;
use lc2k::{AsmLine, Instr, JArgs, LArgs, Offset, RArgs, Reg};

// figure out how to do proper import later

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];

    let contents = fs::read_to_string(input_file)?;

    let block = cimple::BlockParser::new().parse(&contents).unwrap();

    // println!("{:?}", block);
    // println!("{:?}", expr2);

    print!("{}", lc2k::output_asm_lines(compile(block).to_vec()));
    Ok(())
}
