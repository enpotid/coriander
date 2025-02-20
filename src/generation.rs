use crate::parser::*;

pub fn generate(root: NodeExit) -> String {
    let mut output = String::from("global _start\n_start:\n");
    output.push_str(&format!(
        "    mov rax, 60\n    mov rdi, {}\n    syscall\n",
        root.expr.int_lit.value.unwrap().parse::<usize>().unwrap()
    ));
    output
}
