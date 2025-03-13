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
pub struct NodeTermParen {
    pub expr: NodeExpr,
}

#[derive(Clone)]
pub struct NodeBinExprAdd {
    pub lhs: NodeExpr,
    pub rhs: NodeExpr,
}

#[derive(Clone)]
pub struct NodeBinExprSub {
    pub lhs: NodeExpr,
    pub rhs: NodeExpr,
}

#[derive(Clone)]
pub struct NodeBinExprMulti {
    pub lhs: NodeExpr,
    pub rhs: NodeExpr,
}

#[derive(Clone)]
pub struct NodeBinExprDiv {
    pub lhs: NodeExpr,
    pub rhs: NodeExpr,
}

#[derive(Clone)]
pub struct NodeBinExprMod {
    pub lhs: NodeExpr,
    pub rhs: NodeExpr,
}

#[derive(Clone)]
pub enum NodeBinExpr {
    Add(NodeBinExprAdd),
    Sub(NodeBinExprSub),
    Multi(NodeBinExprMulti),
    Div(NodeBinExprDiv),
    Mod(NodeBinExprMod),
}

#[derive(Clone)]
pub enum NodeTerm {
    IntLit(NodeTermIntLit),
    Ident(NodeTermIdent),
    Paren(NodeTermParen),
}

#[derive(Clone)]
pub enum NodeExpr {
    Term(Box<NodeTerm>),
    BinExpr(Box<NodeBinExpr>),
}

pub struct NodeStmtExit {
    pub expr: NodeExpr,
}

pub struct NodeStmtLet {
    pub ident: Token,
    pub expr: NodeExpr,
}

pub enum NodePExpr {
    Str(String),
    Expr(NodeExpr),
}

pub struct NodePrint {
    pub plist: Vec<NodePExpr>,
}

pub enum NodeStmt {
    Exit(NodeStmtExit),
    Let(NodeStmtLet),
    Print(NodePrint),
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
    if try_consume_op(tokens, i, TokenType::Exit).is_some() {
        let stmt_exit: NodeStmtExit;
        if let Some(node_expr) = parse_expr(&tokens, i, 0) {
            stmt_exit = NodeStmtExit { expr: node_expr };
        } else {
            println!("invalid expression");
            exit(1);
        }
        try_consume(tokens, i, TokenType::Semi, "expected `;`");
        return Some(NodeStmt::Exit(stmt_exit));
    } else if try_consume_op(tokens, i, TokenType::Let).is_some() {
        let ident = try_consume(tokens, i, TokenType::Ident, "expected ident");
        try_consume(tokens, i, TokenType::Eq, "expected =");
        if let Some(expr) = parse_expr(&tokens, i, 0) {
            try_consume(tokens, i, TokenType::Semi, "expected `;`");
            return Some(NodeStmt::Let(NodeStmtLet { ident, expr }));
        } else {
            println!("invalid expression");
            exit(1);
        }
    } else if try_consume_op(tokens, i, TokenType::Print).is_some() {
        let mut plist = Vec::new();
        loop {
            if let Some(_) = try_consume_op(tokens, i, TokenType::Semi) {
                break;
            } else {
                let pexpr = try_consume_vec(
                    tokens,
                    i,
                    vec![TokenType::Msg, TokenType::IntLit, TokenType::Ident],
                    "expected ';'",
                );

                match pexpr.ttype {
                    TokenType::Msg => {
                        plist.push(NodePExpr::Str(pexpr.value.unwrap()));
                    }
                    TokenType::Ident => {
                        plist.push(NodePExpr::Expr(NodeExpr::Term(Box::new(NodeTerm::Ident(
                            NodeTermIdent { ident: pexpr },
                        )))));
                    }
                    _ => {
                        plist.push(NodePExpr::Expr(NodeExpr::Term(Box::new(NodeTerm::IntLit(
                            NodeTermIntLit { int_lit: pexpr },
                        )))));
                    }
                }
            }
        }
        return Some(NodeStmt::Print(NodePrint { plist }));
    } else {
        return None;
    }
}

fn parse_term(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeTerm> {
    if let Some(int_lit) = try_consume_op(tokens, i, TokenType::IntLit) {
        return Some(NodeTerm::IntLit(NodeTermIntLit { int_lit }));
    } else if let Some(ident) = try_consume_op(tokens, i, TokenType::Ident) {
        return Some(NodeTerm::Ident(NodeTermIdent { ident }));
    } else if let Some(_) = try_consume_op(tokens, i, TokenType::OpenParen) {
        if let Some(expr) = parse_expr(tokens, i, 0) {
            try_consume(tokens, i, TokenType::CloseParen, "expected `)`");
            return Some(NodeTerm::Paren(NodeTermParen { expr }));
        } else {
            println!("expected expression");
            exit(1);
        }
    } else {
        return None;
    }
}

fn parse_expr(tokens: &Vec<Token>, i: &mut usize, min_prec: usize) -> Option<NodeExpr> {
    let term_lhs = parse_term(tokens, i)?;
    let mut expr_lhs = NodeExpr::Term(Box::new(term_lhs));

    loop {
        let curr_tok = peek(tokens, *i, 0);
        let prec;
        if curr_tok.is_some() {
            let p = bin_prec(curr_tok.unwrap().ttype);
            if p.is_some() {
                if p.unwrap() < min_prec {
                    break;
                }
                prec = p.unwrap();
            } else {
                break;
            }
        } else {
            break;
        }
        let op = consume(tokens, i);
        let next_min_prec = prec + 1;
        let expr_rhs_o = parse_expr(tokens, i, next_min_prec);
        if expr_rhs_o.is_none() {
            println!("unable to parse expression");
            exit(1);
        }
        let expr_rhs = expr_rhs_o.unwrap();

        if op.ttype == TokenType::Plus {
            expr_lhs = NodeExpr::BinExpr(Box::new(NodeBinExpr::Add(NodeBinExprAdd {
                lhs: expr_lhs,
                rhs: expr_rhs,
            })));
        } else if op.ttype == TokenType::Sub {
            expr_lhs = NodeExpr::BinExpr(Box::new(NodeBinExpr::Sub(NodeBinExprSub {
                lhs: expr_lhs,
                rhs: expr_rhs,
            })));
        } else if op.ttype == TokenType::Star {
            expr_lhs = NodeExpr::BinExpr(Box::new(NodeBinExpr::Multi(NodeBinExprMulti {
                lhs: expr_lhs,
                rhs: expr_rhs,
            })));
        } else if op.ttype == TokenType::Div {
            expr_lhs = NodeExpr::BinExpr(Box::new(NodeBinExpr::Div(NodeBinExprDiv {
                lhs: expr_lhs,
                rhs: expr_rhs,
            })));
        } else if op.ttype == TokenType::Mod {
            expr_lhs = NodeExpr::BinExpr(Box::new(NodeBinExpr::Mod(NodeBinExprMod {
                lhs: expr_lhs,
                rhs: expr_rhs,
            })));
        }
    }

    return Some(expr_lhs);
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

fn try_consume_vec(tokens: &Vec<Token>, i: &mut usize, ttype: Vec<TokenType>, err: &str) -> Token {
    if peek(&tokens, *i, 0).is_some() {
        for t in ttype {
            if peek(&tokens, *i, 0).unwrap().ttype == t {
                return consume(tokens, i);
            }
        }
        println!("{err}");
        exit(1);
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
