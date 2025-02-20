use crate::parser::{NodeExpr::*, NodeStmt::*, *};

pub fn gen_prog(prog: NodeProg) -> String {
    let mut output = String::from("global _start\n_start:\n");

    for stmt in prog.stmts {
        output.push_str(&gen_stmt(stmt));
    }

    output.push_str("    mov rax, 60\n    mov rdi, 0\n    syscall\n");
    output
}

fn gen_expr(expr: NodeExpr) -> String {
    match expr {
        IntLit(expr_int_lit) => format!(
            "    mov rax, {}\n    push rax\n",
            expr_int_lit.int_lit.value.unwrap()
        ),
        Ident(_) => String::new(),
    }
}

fn gen_stmt(stmt: NodeStmt) -> String {
    match stmt {
        Exit(stmt_exit) => format!(
            "{}    mov rax, 60\n    pop rdi\n    syscall\n",
            gen_expr(stmt_exit.expr)
        ),
        Let(_) => String::new(),
    }
}
