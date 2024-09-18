use slang::{ast::{Expr, Name, Type}, Span};
use slang_ui::prelude::*;

use crate::ivl::{IVLCmd, IVLCmdKind};

impl IVLCmd {
    pub(crate) fn assign(name: &Name, expr: &Expr) -> IVLCmd {
        IVLCmd {
            span: Span::default(),
            kind: IVLCmdKind::Assignment { name: name.clone(), expr: expr.clone() }
        }
    }
    pub(crate) fn seq(&self, other: &IVLCmd) -> IVLCmd {
        IVLCmd {
            span: Span::default(),
            kind: IVLCmdKind::Seq(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
    pub(crate) fn seqs(cmds: &Vec<IVLCmd>) -> IVLCmd {
        cmds.iter().cloned()
        .reduce(|a, b| IVLCmd::seq(&a, &b))
        .unwrap_or(IVLCmd::nop())
    }
    pub(crate) fn nondet(&self, other: &IVLCmd) -> IVLCmd {
        IVLCmd {
            span: Span::default(),
            kind: IVLCmdKind::NonDet(Box::new(self.clone()), Box::new(other.clone()))
        }
    }
    pub(crate) fn nondets(cmds: &Vec<IVLCmd>) -> IVLCmd {
        cmds.iter().cloned()
            .reduce(|a, b| IVLCmd::nondet(&a, &b))
            .unwrap_or(IVLCmd::unreachable())
    }
    pub(crate) fn assume(condition: &Expr) -> IVLCmd {
        IVLCmd {
            span: Span::default(),
            kind: IVLCmdKind::Assume { condition: condition.clone() }
        }
    }
    pub(crate) fn assert(condition: &Expr, message: &str) -> IVLCmd {
        IVLCmd {
            span: Span::default(),
            kind: IVLCmdKind::Assert { condition: condition.clone(), message: message.to_owned() },
        }
    }
    pub(crate) fn havoc(name: &Name, ty: &Type) -> IVLCmd {
        IVLCmd {
            kind: IVLCmdKind::Havoc { name: name.clone(), ty: ty.clone() },
            span: Span::default()
        }
    }
    pub(crate) fn nop() -> IVLCmd {
        IVLCmd::assume(&Expr::bool(true))
    }
    pub(crate) fn unreachable() -> IVLCmd {
        IVLCmd::assume(&Expr::bool(false))
    }
}

impl std::fmt::Display for IVLCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            IVLCmdKind::Assignment { name, expr } => write!(f, "{name} := {expr}"),
            IVLCmdKind::Havoc { name, .. } => write!(f, "havoc {name}"),
            IVLCmdKind::Assume { condition } => write!(f, "assume {condition}"),
            IVLCmdKind::Assert { condition, .. } => write!(f, "assert {condition}"),
            IVLCmdKind::Seq(c1, c2) => write!(f, "{c1} ; {c2}"),
            IVLCmdKind::NonDet(c1, c2) => write!(f, "{{ {c1} }} [] {{ {c2} }}"),
        }
    }
}
