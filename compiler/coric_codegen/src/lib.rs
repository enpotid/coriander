pub mod target;
pub use target::*;

mod codegen_asm;
use coric_ast::Program;

pub fn generate(ast: Program, target: Target) -> String {
    match target {
        Target::Asm => codegen_asm::generate(ast),
    }
}
