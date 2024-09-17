use slang::{ast::{Expr, Name, Type}, Span};
use slang_ui::prelude::*;

#[derive(Debug, Clone)]
pub struct IVL0Stmt {
    pub span: Span,
    pub kind: IVL0StmtKind,
}

#[derive(Debug, Clone)]
pub enum IVL0StmtKind {
    Assignment {
        name: Name,
        expr: Expr,
    },
    Havoc {
        name: Name,
        ty: Type,
    },

    Assume {
        condition: Expr
    },
    Assert {
        condition: Expr,
        message: String,
    },

    Seq(Box<IVL0Stmt>, Box<IVL0Stmt>),
    NonDet(Box<IVL0Stmt>, Box<IVL0Stmt>),
}
