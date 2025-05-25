use crate::Span;

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub ident: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TyKind {
    Prim(PrimTy),
    Tup(Vec<Ty>),
    Path(Path),
}

#[derive(Debug, Clone)]
pub enum PrimTy {
    Bool,
    Char,
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Enum(Ident, EnumDef),
    Struct(Ident, VariantData),
    Fn(Fn),
}

#[derive(Debug, Clone)]
pub struct Fn {
    pub prefix: Vec<FnPrefix>,
    pub ident: Ident,
    pub body: Option<Block>,
}

#[derive(Debug, Clone)]
pub enum FnPrefix {}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub ident: Ident,
    pub data: VariantData,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum VariantData {
    Struct(Vec<FieldDef>),
    Tuple(Vec<FieldDef>),
    Unit,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub ident: Option<Ident>,
    pub ty: Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub ident: Ident,
    pub segments: Option<Box<Path>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum PathTree {
    Simple(Path),
    Group(Vec<Path>),
}
