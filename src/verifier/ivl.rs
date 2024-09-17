use slang::{ast::{Expr, Name, Type}, Span};
use slang_ui::prelude::*;

#[derive(Debug, Clone)]
pub struct IVLStmt {
    pub span: Span,
    pub kind: IVLStmtKind,
}

#[derive(Debug, Clone)]
pub enum IVLStmtKind {
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

    Seq(Box<IVLStmt>, Box<IVLStmt>),
    NonDet(Box<IVLStmt>, Box<IVLStmt>),
}
