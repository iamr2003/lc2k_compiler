use crate::ast::{Expr, Opcode};
use crate::lc2k::{AsmLine, Instr, JArgs, LArgs, Offset, RArgs, Reg};
use std::collections::hash_map::HashMap;

// some register conventions
// R0 is always 0
// R1 is return address
// R5 and R6 are stratch registers
// R7 is stack register

// need to handle statements
// will need to keep a context mapping for variables and stack offsets, from there we can attempt
// from there can compile variable accesses back and forth

pub struct Context {
    pub var_names_offsets: HashMap<String, i8>,
    pub unused_stack_offset: i8,
}

impl Context {
    fn new() -> Self {
        Context {
            var_names_offsets: HashMap::new(),
            unused_stack_offset: 0,
        }
    }
}

fn var_offset_naming(s: &String) -> String {
    return s.to_string() + "_offset_";
}

pub fn compute_names(stmts: &Vec<Box<Expr>>, ctx: &mut Context) {
    // no we're just going to need to initialize with R7
    for stmt in stmts {
        compute_names_rec(*&stmt, ctx);
    }
}

pub fn compute_names_rec(stmt: &Expr, ctx: &mut Context) {
    match stmt {
        Expr::Op(l, _, r) => {
            compute_names_rec(*&l, ctx);
            compute_names_rec(*&r, ctx);
        }
        Expr::Var(name) => {
            if !ctx.var_names_offsets.contains_key(name) {
                ctx.var_names_offsets
                    .insert(name.clone(), ctx.unused_stack_offset);
                ctx.unused_stack_offset += 1;
            }
        }
        Expr::Number(_) => {}
        Expr::Assign(name, expr) => {
            compute_names_rec(*&expr, ctx);
            if !ctx.var_names_offsets.contains_key(name) {
                ctx.var_names_offsets
                    .insert(name.clone(), ctx.unused_stack_offset);
                ctx.unused_stack_offset += 1;
            }
        }
    }
}

pub fn compile(stmts: Vec<Box<Expr>>) -> Vec<AsmLine> {
    let mut out: Vec<AsmLine> = vec![];
    let mut firsts: Vec<AsmLine> = vec![];
    let mut seconds: Vec<AsmLine> = vec![];

    let mut ctx: Context = Context::new();
    compute_names(&stmts, &mut ctx);

    for stmt in stmts {
        let (first, second) = compile_rec(*stmt, &mut ctx);
        firsts.extend(first);
        seconds.extend(second);
    }
    out.extend(firsts);
    out.push(AsmLine {
        label: "end".to_string(),
        instr: Instr::Halt,
    });
    out.extend(seconds);
    // handle context offsets
    for (name, offset) in ctx.var_names_offsets.iter() {
        out.push(AsmLine {
            label: var_offset_naming(name),
            instr: Instr::Fill(Offset::Numeric(*offset)),
        });
    }
    //initialize R7 properly
    let filled_lines = out.len() + 3; //we'll add two at beginning and end
    let (mut pre_r7_fill, post_r7_fill) = compile_rec(Expr::Number(filled_lines as i8), &mut ctx);
    pre_r7_fill.extend(out);
    out = pre_r7_fill;
    out.insert(
        1,
        AsmLine {
            label: "stk_init".to_string(),
            instr: Instr::Add(RArgs {
                reg_a: Reg::R1,
                reg_b: Reg::R0,
                dest_reg: Reg::R7,
            }),
        },
    );
    out.extend(post_r7_fill);
    out
}

// has a preamble and suffix
pub fn compile_rec(expr: Expr, ctx: &mut Context) -> (Vec<AsmLine>, Vec<AsmLine>) {
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

            let (left_pre, left_post) = compile_rec(*left, ctx);
            let (right_pre, right_post) = compile_rec(*right, ctx);
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
        Expr::Var(name) => {
            let mut pre: Vec<AsmLine> = vec![];
            pre.extend(vec![AsmLine {
                label: "".to_string(),
                instr: Instr::Lw(LArgs {
                    reg_a: Reg::R7,
                    reg_b: Reg::R1,
                    addr: Offset::Symbolic(var_offset_naming(&name)),
                }),
            }]);
            (pre, vec![])
        }
        Expr::Assign(name, expr) => {
            let (mut pre_expr, post_expr) = compile_rec(*expr, ctx);
            pre_expr.push(AsmLine {
                label: "".to_string(),
                instr: Instr::Sw(LArgs {
                    reg_a: Reg::R7,
                    reg_b: Reg::R1,
                    addr: Offset::Symbolic(var_offset_naming(&name)),
                }),
            });
            (pre_expr, post_expr)
        }
    }
}
