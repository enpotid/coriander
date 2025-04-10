use crate::parser::{NodeBinExpr::*, NodeExpr::*, NodeItem::*, NodeStmt::*, NodeTerm::*, *};
use std::{collections::HashMap, process::exit};

struct Var {
    stack_loc: usize,
}

pub fn gen_prog(prog: NodeProg) -> String {
    let mut output = String::new();
    let entry = prog.entry;
    if let Some(entry) = entry.clone() {
        output.push_str(&format!("global {entry}\n\n"));
    }

    let mut stack_size = 0;
    let mut vars: HashMap<String, Var> = HashMap::new();
    for item in prog.items {
        output.push_str(&format!(
            "{}\n",
            gen_item(item, entry.clone(), &mut stack_size, &mut vars)
        ));
    }

    output
}

fn gen_item(
    item: NodeItem,
    entry: Option<String>,
    stack_size: &mut usize,
    vars: &mut HashMap<String, Var>,
) -> String {
    match item {
        Fn(item_fn) => {
            let mut output = format!("{}:\n", item_fn.name);
            for stmt in item_fn.stmts {
                output.push_str(&gen_stmt(stmt, stack_size, vars));
            }
            if item_fn.name != entry.unwrap() && item_fn.option & 1 == 0 {
                output.push_str("    ret\n");
            }
            output
        }
    }
}

fn gen_term(term: &NodeTerm, stack_size: &mut usize, vars: &mut HashMap<String, Var>) -> String {
    match term {
        IntLit(int_lit) => format!("    mov rax, {}\n{}", int_lit, push("rax", stack_size)),
        Ident(ident) => {
            if !vars.contains_key(ident) {
                println!("undeclared identifier: {}", ident);
                exit(1);
            } else if *stack_size == vars.get(ident).unwrap().stack_loc {
                println!("it cannot reference itself: {}", ident);
                exit(1);
            }
            let var = vars.get(ident).unwrap();
            push(
                &format!("QWORD [rsp + {}]", (*stack_size - var.stack_loc - 1) * 8),
                stack_size,
            )
        }
        Paren(expr_paren) => gen_expr(expr_paren.clone().expr, stack_size, vars),
    }
}

fn gen_bin_expr(
    bin_expr: &NodeBinExpr,
    stack_size: &mut usize,
    vars: &mut HashMap<String, Var>,
) -> String {
    match bin_expr {
        Add(add) => format!(
            "{}{}{}{}    add rax, rbx\n{}",
            gen_expr(add.lhs.clone(), stack_size, vars),
            gen_expr(add.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rax", stack_size)
        ),
        Sub(sub) => format!(
            "{}{}{}{}    sub rax, rbx\n{}",
            gen_expr(sub.lhs.clone(), stack_size, vars),
            gen_expr(sub.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rax", stack_size)
        ),
        Multi(multi) => format!(
            "{}{}{}{}    mul rbx\n{}",
            gen_expr(multi.lhs.clone(), stack_size, vars),
            gen_expr(multi.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rax", stack_size)
        ),
        Div(div) => format!(
            "{}{}{}{}    xor rdx, rdx\n    div rbx\n{}",
            gen_expr(div.lhs.clone(), stack_size, vars),
            gen_expr(div.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rax", stack_size)
        ),
        Mod(modd) => format!(
            "{}{}{}{}    xor rdx, rdx\n    div rbx\n{}",
            gen_expr(modd.lhs.clone(), stack_size, vars),
            gen_expr(modd.rhs.clone(), stack_size, vars),
            pop("rbx", stack_size),
            pop("rax", stack_size),
            push("rdx", stack_size)
        ),
    }
}

fn gen_expr(expr: NodeExpr, stack_size: &mut usize, vars: &mut HashMap<String, Var>) -> String {
    match expr {
        Term(expr_term) => gen_term(expr_term.as_ref(), stack_size, vars),
        BinExpr(expr_bin) => gen_bin_expr(expr_bin.as_ref(), stack_size, vars),
    }
}

fn gen_stmt(stmt: NodeStmt, stack_size: &mut usize, vars: &mut HashMap<String, Var>) -> String {
    match stmt {
        Assembly(asm_code) => format!("    {}\n", asm_code),
        Let(stmt_let) => {
            if vars.contains_key(&stmt_let.ident.clone()) {
                println!("identifier already used: {}", stmt_let.ident);
                exit(1);
            }
            vars.insert(
                stmt_let.ident,
                Var {
                    stack_loc: *stack_size,
                },
            );
            gen_expr(stmt_let.expr, stack_size, vars)
        }
    }
}

fn push(reg: &str, stack_size: &mut usize) -> String {
    *stack_size += 1;
    format!("    push {reg}\n")
}

fn pop(reg: &str, stack_size: &mut usize) -> String {
    *stack_size -= 1;
    format!("    pop {reg}\n")
}
