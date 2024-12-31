use crate::ast::{Expr, Opcode};
use crate::lc2k::{AsmLine, Instr, JArgs, LArgs, Offset, RArgs, Reg};

// some register conventions
// R0 is always 0
// R1 is return address
// R6 and R7 are stratch registers

pub fn compile(expr: Expr) -> Vec<AsmLine> {
    let mut out: Vec<AsmLine> = vec![];
    let (first, second) = compile_rec(expr);
    out.extend(first);
    out.push(AsmLine {
        label: "end".to_string(),
        instr: Instr::Halt,
    });
    out.extend(second);
    out
}

// has a preamble and suffix
pub fn compile_rec(expr: Expr) -> (Vec<AsmLine>, Vec<AsmLine>) {
    match expr {
        Expr::Number(num) => {
            let val_name = "val_".to_string() + &num.to_string();
            (
                vec![AsmLine {
                    label: "".to_string(),
                    instr: Instr::Lw(LArgs {
                        reg_a: Reg::R0,
                        reg_b: Reg::R1,
                        addr: Offset::Symbolic(val_name.clone()),
                    }),
                }],
                vec![AsmLine {
                    label: val_name,
                    instr: Instr::Fill(Offset::Numeric(num)),
                }],
            )
        }

        Expr::Op(left, op, right) => {
            assert!(op == Opcode::Add);

            //this ordering is wrong

            let (left_pre, left_post) = compile_rec(*left);
            let (right_pre, right_post) = compile_rec(*right);
            let mut pre: Vec<AsmLine> = vec![];
            pre.extend(left_pre);
            // put it in stratch register R6, no move is annoying
            pre.extend(vec![AsmLine {
                label: "".to_string(),
                instr: Instr::Add(RArgs {
                    reg_a: Reg::R1,
                    reg_b: Reg::R0,
                    dest_reg: Reg::R6,
                }),
            }]);
            pre.extend(right_pre);
            //add the rest of it
            pre.extend(vec![AsmLine {
                label: "".to_string(),
                instr: Instr::Add(RArgs {
                    reg_a: Reg::R1,
                    reg_b: Reg::R6,
                    dest_reg: Reg::R1,
                }),
            }]);
            let mut post: Vec<AsmLine> = vec![];
            post.extend(left_post);
            post.extend(right_post);
            (pre, post)
        }
    }
}
