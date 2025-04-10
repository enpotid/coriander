use crate::tokenization::*;
use std::process::exit;

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
    IntLit(String),
    Ident(String),
    Paren(NodeTermParen),
}

#[derive(Clone)]
pub enum NodeExpr {
    Term(Box<NodeTerm>),
    BinExpr(Box<NodeBinExpr>),
}

pub struct NodeStmtLet {
    pub ident: String,
    pub expr: NodeExpr,
}

pub enum NodeStmt {
    Assembly(String),
    Let(NodeStmtLet),
}

pub struct NodeFn {
    pub name: String,
    pub stmts: Vec<NodeStmt>,
    pub option: u8,
}

pub enum NodeItem {
    Fn(NodeFn),
}

pub struct NodeProg {
    pub entry: Option<String>,
    pub items: Vec<NodeItem>,
}

pub fn parse_prog(tokens: &Vec<Token>) -> Option<NodeProg> {
    let mut items = Vec::new();
    let mut entry = None;
    let mut i = 0;
    let mut is_first_fn = true;
    while peek(&tokens, i, 0).is_some() {
        if let Some(item) = parse_item(tokens, &mut i) {
            if is_first_fn {
                match &item {
                    NodeItem::Fn(func) => {
                        entry = Some(func.name.clone());
                        is_first_fn = false;
                    }
                }
            }

            items.push(item);
        } else {
            println!("invalid item");
            exit(1);
        }
    }
    Some(NodeProg { entry, items })
}

fn parse_item(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeItem> {
    if let Some(func) = parse_fn_prefix(tokens, i, &mut 0) {
        Some(NodeItem::Fn(func))
    } else {
        None
    }
}

fn parse_fn_prefix(tokens: &Vec<Token>, i: &mut usize, option: &mut u8) -> Option<NodeFn> {
    if let Some(prefix) = try_consume_vec_op(tokens, i, vec![TokenType::Ident, TokenType::Tilde]) {
        if prefix.ttype == TokenType::Ident {
            parse_fn(tokens, i, prefix.value.unwrap(), *option)
        } else if prefix.ttype == TokenType::Tilde {
            *option |= 1;
            parse_fn_prefix(tokens, i, option)
        } else {
            parse_fn_prefix(tokens, i, option)
        }
    } else {
        None
    }
}

fn parse_fn(tokens: &Vec<Token>, i: &mut usize, name: String, option: u8) -> Option<NodeFn> {
    let mut func = NodeFn {
        name,
        stmts: Vec::new(),
        option,
    };
    try_consume(tokens, i, TokenType::OpenParen, "expected `(`");
    try_consume(tokens, i, TokenType::CloseParen, "expected `)`");
    try_consume(tokens, i, TokenType::OpenBrace, "expected `{`");
    while peek(&tokens, *i, 0).is_some() {
        if peek(&tokens, *i, 0).unwrap().ttype == TokenType::CloseBrace {
            consume(tokens, i);
            return Some(func);
        } else if let Some(stmt) = parse_stmt(tokens, i) {
            func.stmts.push(stmt);
        } else {
            println!("invalid expression");
            exit(1);
        }
    }
    println!("expected `}}`");
    exit(1);
}

fn parse_stmt(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeStmt> {
    if try_consume_op(tokens, i, TokenType::Let).is_some() {
        let ident = try_consume(tokens, i, TokenType::Ident, "expected ident")
            .value
            .unwrap();
        try_consume(tokens, i, TokenType::Eq, "expected =");
        if let Some(expr) = parse_expr(&tokens, i, 0) {
            try_consume(tokens, i, TokenType::Semi, "expected `;`");
            return Some(NodeStmt::Let(NodeStmtLet { ident, expr }));
        } else {
            println!("invalid expression");
            exit(1);
        }
    } else if try_consume_op(tokens, i, TokenType::OpenBracket).is_some() {
        let code = try_consume(tokens, i, TokenType::String, "expected string")
            .value
            .unwrap();
        try_consume(tokens, i, TokenType::CloseBracket, "expected `]`");
        try_consume(tokens, i, TokenType::Semi, "expected `;`");
        return Some(NodeStmt::Assembly(code));
    } else {
        return None;
    }
}

fn parse_term(tokens: &Vec<Token>, i: &mut usize) -> Option<NodeTerm> {
    if let Some(int_lit) = try_consume_op(tokens, i, TokenType::IntLit) {
        return Some(NodeTerm::IntLit(int_lit.value.unwrap()));
    } else if let Some(ident) = try_consume_op(tokens, i, TokenType::Ident) {
        return Some(NodeTerm::Ident(ident.value.unwrap()));
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

fn try_consume_vec_op(tokens: &Vec<Token>, i: &mut usize, ttype: Vec<TokenType>) -> Option<Token> {
    if peek(&tokens, *i, 0).is_some() {
        for t in ttype {
            if peek(&tokens, *i, 0).unwrap().ttype == t {
                return Some(consume(tokens, i));
            }
        }
        None
    } else {
        None
    }
}

fn try_consume_op(tokens: &Vec<Token>, i: &mut usize, ttype: TokenType) -> Option<Token> {
    if peek(&tokens, *i, 0).is_some() && peek(&tokens, *i, 0).unwrap().ttype == ttype {
        return Some(consume(tokens, i));
    } else {
        return None;
    }
}
