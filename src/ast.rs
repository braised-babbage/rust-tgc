use crate::lexer::TokenPos as Pos;
use crate::intern::Symbol;

#[derive(Debug, PartialEq)]
pub enum Var<'a> {
    Simple(Symbol, Pos),
    Field(Box<Var<'a>>, Symbol, Pos),
    Subscript(Box<Var<'a>>, Box<Expr<'a>>, Pos),
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Divide,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    VarRef(Box<Var<'a>>),
    Nil,
    Int(i32),
    String(&'a str),
    Call {
        func: Symbol,
        args: Vec<Expr<'a>>,
        pos: Pos,
    },
    BinOp {
        left: Box<Expr<'a>>,
        oper: Op,
        right: Box<Expr<'a>>,
        pos: Pos,
    },
    Record {
        fields: Vec<(Symbol, Box<Expr<'a>>, Pos)>,
        rtype: Symbol,
        pos: Pos,
    },
    Seq(Vec<(Box<Expr<'a>>, Pos)>),
    Assign {
        var: Var<'a>,
        expr: Box<Expr<'a>>,
        pos: Pos,
    },
    If {
        test: Box<Expr<'a>>,
        then_branch: Box<Expr<'a>>,
        else_branch: Option<Box<Expr<'a>>>,
        pos: Pos,
    },
    While {
        test: Box<Expr<'a>>,
        body: Box<Expr<'a>>,
        pos: Pos,
    },
    For {
        var: Symbol,
        // todo: escape
        lo: Box<Expr<'a>>,
        hi: Box<Expr<'a>>,
        pos: Pos,
    },
    Break(Pos),
    Let {
        decls: Vec<Box<Decl<'a>>>,
        body: Box<Expr<'a>>,
        pos: Pos,
    },
    Array {
        etype: Symbol,
        size: Box<Expr<'a>>,
        init: Box<Expr<'a>>,
        pos: Pos,
    },
}

#[derive(Debug, PartialEq)]
pub struct Field {
    name: Symbol,
    // todo: escape,
    ftype: Symbol,
    pos: Pos,
}

#[derive(Debug, PartialEq)]
pub struct Fundecl<'a> {
    name: Symbol,
    params: Vec<Box<Field>>,
    result: Option<(Symbol, Pos)>,
    body: Box<Expr<'a>>,
    pos: Pos,
}

#[derive(Debug, PartialEq)]
pub struct Typedecl {
    name: Symbol,
    ty: Ty,
    pos: Pos,
}

#[derive(Debug, PartialEq)]
pub enum Ty {
    Name(Symbol, Pos),
    Record(Vec<Box<Field>>),
    Array(Symbol, Pos),
}

#[derive(Debug, PartialEq)]
pub enum Decl<'a> {
    Function(Vec<Box<Fundecl<'a>>>),
    Var {
        name: Symbol,
        // todo: escape
        vtype: Option<(Symbol, Pos)>,
        init: Box<Expr<'a>>,
        pos: Pos,
    },
    Type(Vec<Box<Typedecl>>),
}
