use crate::*;
use std::process::exit;

pub struct NodeExpr {
    pub int_lit: Token,
}

pub struct NodeExit {
    pub expr: NodeExpr,
}

pub fn parse(tokens: &Vec<Token>) -> Option<NodeExit> {
    let mut exit_node = None;
    let mut i = 0;
    while peak(&tokens, i, 0).is_some() {
        if peak(&tokens, i, 0).unwrap().ttype == TokenType::Exit {
            consume(&tokens, &mut i);
            if let Some(node_expr) = parse_expr(&tokens, &mut i) {
                exit_node = Some(NodeExit { expr: node_expr });
            } else {
                println!("invalid expression");
                exit(1);
            }
            if peak(&tokens, i, 0).is_none()
                || peak(&tokens, i, 0).unwrap().ttype != TokenType::Semi
            {
                println!("invalid expression");
                exit(1);
            }
        }
        consume(&tokens, &mut i);
    }
    exit_node
}

fn parse_expr(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeExpr> {
    if peak(&tokens, *i, 0).is_some() && peak(&tokens, *i, 0).unwrap().ttype == TokenType::IntLit {
        return Some(NodeExpr {
            int_lit: consume(&tokens, i),
        });
    }
    None
}

fn peak(tokens: &Vec<Token>, i: usize, offset: usize) -> Option<Token> {
    if i + offset >= tokens.len() {
        return None;
    } else {
        return Some(tokens[i + offset].clone());
    }
}

fn consume(tokens: &Vec<Token>, i: &mut usize) -> Token {
    let v = tokens[*i].clone();
    *i += 1;
    v
}
