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
    // todo
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Nec(Ident),
    Index,
    Enum(Ident, EnumDef),
    Struct(Ident, VariantData),
    Fn,
}

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
