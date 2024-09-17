use slang::{ast::{Expr, Name, Type}, Span};
use slang_ui::prelude::*;

use crate::ivl::{IVLStmt, IVLStmtKind};

impl IVLStmt {
    pub(crate) fn assign(name: &Name, expr: &Expr) -> IVLStmt {
        IVLStmt {
            span: Span::default(),
            kind: IVLStmtKind::Assignment { name: name.clone(), expr: expr.clone() }
        }
    }
    pub(crate) fn seq(&self, other: &IVLStmt) -> IVLStmt {
        IVLStmt {
            span: Span::default(),
            kind: IVLStmtKind::Seq(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
    pub(crate) fn seqs(stmts: &Vec<IVLStmt>) -> IVLStmt {
        stmts.iter().cloned()
        .reduce(|a, b| IVLStmt::seq(&a, &b))
        .unwrap_or(IVLStmt::nop())
    }
    pub(crate) fn nondet(&self, other: &IVLStmt) -> IVLStmt {
        IVLStmt {
            span: Span::default(),
            kind: IVLStmtKind::NonDet(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
    pub(crate) fn nondets(stmts: &Vec<IVLStmt>) -> IVLStmt {
        stmts.iter().cloned()
            .reduce(|a, b| IVLStmt::nondet(&a, &b))
            .unwrap_or(IVLStmt::unreachable())
    }
    pub(crate) fn assume(condition: &Expr) -> IVLStmt {
        IVLStmt {
            span: Span::default(),
            kind: IVLStmtKind::Assume { condition: condition.clone() }
        }
    }
    pub(crate) fn assert(condition: &Expr, message: &str) -> IVLStmt {
        IVLStmt {
            span: Span::default(),
            kind: IVLStmtKind::Assert { condition: condition.clone(), message: message.to_owned() },
        }
    }
    pub(crate) fn havoc(name: &Name, ty: &Type) -> IVLStmt {
        IVLStmt {
            kind: IVLStmtKind::Havoc { name: name.clone(), ty: ty.clone() },
            span: Span::default()
        }
    }
    pub(crate) fn nop() -> IVLStmt {
        IVLStmt::assume(&Expr::bool(true))
    }
    pub(crate) fn unreachable() -> IVLStmt {
        IVLStmt::assume(&Expr::bool(false))
    }
}

impl std::fmt::Display for IVLStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            IVLStmtKind::Assignment { name, expr } => write!(f, "{name} := {expr}"),
            IVLStmtKind::Havoc { name, .. } => write!(f, "havoc {name}"),
            IVLStmtKind::Assume { condition } => write!(f, "assume {condition}"),
            IVLStmtKind::Assert { condition, .. } => write!(f, "assert {condition}"),
            IVLStmtKind::Seq(c1, c2) => write!(f, "{c1} ; {c2}"),
            IVLStmtKind::NonDet(c1, c2) => write!(f, "{{ {c1} }} [] {{ {c2} }}"),
        }
    }
}
