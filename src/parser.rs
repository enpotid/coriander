use crate::tokenization::*;
use std::process::exit;

#[derive(Clone)]
pub struct NodeTermIntLit {
    pub int_lit: Token,
}

#[derive(Clone)]
pub struct NodeTermIdent {
    pub ident: Token,
}

#[derive(Clone)]
pub struct NodeBinExprAdd {
    pub lhs: NodeExpr,
    pub rhs: NodeExpr,
}

#[derive(Clone)]
pub struct NodeBinExprMulti {
    pub lhs: NodeExpr,
    pub rhs: NodeExpr,
}

#[derive(Clone)]
pub enum NodeBinExpr {
    Add(NodeBinExprAdd),
    Multi(NodeBinExprMulti),
}

#[derive(Clone)]
pub enum NodeTerm {
    IntLit(NodeTermIntLit),
    Ident(NodeTermIdent),
}

#[derive(Clone)]
pub enum NodeExpr {
    Term(NodeTerm),
    BinExpr(Box<NodeBinExpr>),
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
    if try_consume_op(tokens, i, TokenType::Exit).is_some()
        && try_consume_op(tokens, i, TokenType::OpenParen).is_some()
    {
        let stmt_exit: NodeStmtExit;
        if let Some(node_expr) = parse_expr(&tokens, i) {
            stmt_exit = NodeStmtExit { expr: node_expr };
        } else {
            println!("invalid expression");
            exit(1);
        }
        try_consume(tokens, i, TokenType::CloseParen, "expected `)`");
        try_consume(tokens, i, TokenType::Semi, "expected `;`");
        return Some(NodeStmt::Exit(stmt_exit));
    } else if try_consume_op(tokens, i, TokenType::Let).is_some() {
        let ident = try_consume(tokens, i, TokenType::Ident, "expected ident");
        try_consume(tokens, i, TokenType::Eq, "expected =");
        if let Some(expr) = parse_expr(&tokens, i) {
            try_consume(tokens, i, TokenType::Semi, "expected `;`");
            return Some(NodeStmt::Let(NodeStmtLet { ident, expr }));
        } else {
            println!("invalid expression");
            exit(1);
        }
    } else {
        return None;
    }
}

fn parse_term(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeTerm> {
    if let Some(int_lit) = try_consume_op(tokens, i, TokenType::IntLit) {
        return Some(NodeTerm::IntLit(NodeTermIntLit { int_lit }));
    } else if let Some(ident) = try_consume_op(tokens, i, TokenType::Ident) {
        return Some(NodeTerm::Ident(NodeTermIdent { ident }));
    } else {
        return None;
    }
}

fn parse_expr(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeExpr> {
    if let Some(term) = parse_term(tokens, i) {
        if let Some(_) = try_consume_op(tokens, i, TokenType::Plus) {
            if let Some(rhs) = parse_expr(tokens, i) {
                return Some(NodeExpr::BinExpr(Box::new(NodeBinExpr::Add(
                    NodeBinExprAdd {
                        lhs: NodeExpr::Term(term),
                        rhs,
                    },
                ))));
            } else {
                println!("expected expression");
                exit(1);
            }
        } else {
            return Some(NodeExpr::Term(term));
        }
    } else {
        return None;
    }
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

fn try_consume(tokens: &Vec<Token>, i: &mut usize, ttype: TokenType, err: &str) -> Token {
    if peek(&tokens, *i, 0).is_some() && peek(&tokens, *i, 0).unwrap().ttype == ttype {
        return consume(tokens, i);
    } else {
        println!("{err}");
        exit(1);
    }
}

fn try_consume_op(tokens: &Vec<Token>, i: &mut usize, ttype: TokenType) -> Option<Token> {
    if peek(&tokens, *i, 0).is_some() && peek(&tokens, *i, 0).unwrap().ttype == ttype {
        return Some(consume(tokens, i));
    } else {
        return None;
    }
}
