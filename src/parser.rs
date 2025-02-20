use crate::tokenization::*;
use std::process::exit;

pub struct NodeExprIntLit {
    pub int_lit: Token,
}

pub struct NodeExprIdent {
    pub ident: Token,
}

pub enum NodeExpr {
    IntLit(NodeExprIntLit),
    Ident(NodeExprIdent),
}

pub struct NodeStmtExit {
    pub expr: NodeExpr,
}

pub struct NodeStmtLet {
    pub ident: Token,
    pub expr: NodeExpr,
}

pub enum NodeStmt {
    Exit(NodeStmtExit),
    Let(NodeStmtLet),
}

pub struct NodeProg {
    pub stmts: Vec<NodeStmt>,
}

pub fn parse_prog(tokens: &Vec<Token>) -> Option<NodeProg> {
    let mut prog = NodeProg { stmts: Vec::new() };
    let mut i = 0;
    while peek(&tokens, i, 0).is_some() {
        if let Some(stmt) = parse_stmt(tokens, &mut i) {
            prog.stmts.push(stmt);
        } else {
            println!("invalid expression");
            exit(1);
        }
    }
    Some(prog)
}

fn parse_stmt(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeStmt> {
    if peek(&tokens, *i, 0).unwrap().ttype == TokenType::Exit
        && peek(&tokens, *i, 1).is_some()
        && peek(&tokens, *i, 1).unwrap().ttype == TokenType::OpenParen
    {
        consume(&tokens, i);
        consume(&tokens, i);
        let stmt_exit: NodeStmtExit;
        if let Some(node_expr) = parse_expr(&tokens, i) {
            stmt_exit = NodeStmtExit { expr: node_expr };
        } else {
            println!("invalid expression");
            exit(1);
        }
        if peek(&tokens, *i, 0).is_some()
            && peek(&tokens, *i, 0).unwrap().ttype == TokenType::CloseParen
        {
            consume(&tokens, i);
        } else {
            println!("expected `)`");
            exit(1);
        }
        if peek(&tokens, *i, 0).is_some() && peek(&tokens, *i, 0).unwrap().ttype == TokenType::Semi
        {
            consume(&tokens, i);
        } else {
            println!("expected `;`");
            exit(1);
        }
        return Some(NodeStmt::Exit(stmt_exit));
    } else if peek(&tokens, *i, 0).unwrap().ttype == TokenType::Let
        && peek(&tokens, *i, 2).is_some()
        && peek(&tokens, *i, 2).unwrap().ttype == TokenType::Ident
        && peek(&tokens, *i, 1).is_some()
        && peek(&tokens, *i, 1).unwrap().ttype == TokenType::Eq
    {
        consume(&tokens, i);
        let ident = consume(&tokens, i);
        consume(&tokens, i);
        let expr;
        if let Some(node_expr) = parse_expr(&tokens, i) {
            expr = node_expr;
        } else {
            println!("invalid expression");
            exit(1);
        }
        if peek(&tokens, *i, 0).is_some() && peek(&tokens, *i, 0).unwrap().ttype == TokenType::Semi
        {
            consume(&tokens, i);
        } else {
            println!("expected `;`");
            exit(1);
        }
        return Some(NodeStmt::Let(NodeStmtLet { ident, expr }));
    } else {
        return None;
    }
}

fn parse_expr(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeExpr> {
    if peek(&tokens, *i, 0).is_some() && peek(&tokens, *i, 0).unwrap().ttype == TokenType::IntLit {
        return Some(NodeExpr::IntLit(NodeExprIntLit {
            int_lit: consume(&tokens, i),
        }));
    } else if peek(&tokens, *i, 0).is_some()
        && peek(&tokens, *i, 0).unwrap().ttype == TokenType::Ident
    {
        return Some(NodeExpr::Ident(NodeExprIdent {
            ident: consume(&tokens, i),
        }));
    }
    None
}

fn peek(tokens: &Vec<Token>, i: usize, offset: usize) -> Option<Token> {
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
