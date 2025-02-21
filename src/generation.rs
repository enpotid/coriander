use crate::parser::{NodeExpr::*, NodeStmt::*, *};
use std::{collections::HashMap, process::exit};

struct Var {
    stack_loc: usize,
}

pub fn gen_prog(prog: NodeProg) -> String {
    let mut output = String::from("global _start\n_start:\n");

    let mut stack_size = 0;
    let mut vars: HashMap<String, Var> = HashMap::new();
    for stmt in prog.stmts {
        output.push_str(&gen_stmt(stmt, &mut stack_size, &mut vars));
    }

    output.push_str("    mov rax, 60\n    mov rdi, 0\n    syscall\n");
    output
}

fn gen_expr(expr: NodeExpr, stack_size: &mut usize, vars: &mut HashMap<String, Var>) -> String {
    match expr {
        IntLit(expr_int_lit) => format!(
            "    mov rax, {}\n{}\n",
            expr_int_lit.int_lit.value.unwrap(),
            push("rax", stack_size)
        ),
        Ident(expr_ident) => {
            if !vars.contains_key(&expr_ident.ident.value.clone().unwrap()) {
                println!("undeclared identifier: {}", expr_ident.ident.value.unwrap());
                exit(1);
            } else if *stack_size
                == vars
                    .get(&expr_ident.ident.value.clone().unwrap())
                    .unwrap()
                    .stack_loc
            {
                println!(
                    "it cannot reference itself: {}",
                    expr_ident.ident.value.unwrap()
                );
                exit(1);
            }
            let var = vars.get(&expr_ident.ident.value.clone().unwrap()).unwrap();
            push(
                &format!("QWORD [rsp + {}]\n", (*stack_size - var.stack_loc - 1) * 8),
                stack_size,
            )
        }
    }
}

fn gen_stmt(stmt: NodeStmt, stack_size: &mut usize, vars: &mut HashMap<String, Var>) -> String {
    match stmt {
        Exit(stmt_exit) => format!(
            "{}    mov rax, 60\n{}\n    syscall\n",
            gen_expr(stmt_exit.expr, stack_size, vars),
            pop("rdi", stack_size)
        ),
        Let(stmt_let) => {
            if vars.contains_key(&stmt_let.ident.value.clone().unwrap()) {
                println!("identifier already used: {}", stmt_let.ident.value.unwrap());
                exit(1);
            }
            vars.insert(
                stmt_let.ident.value.unwrap(),
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
