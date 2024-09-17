pub mod ivl; mod ivl_ext;

use ivl::{IVLStmt, IVLStmtKind};
use slang::ast::{Expr, Stmt, StmtKind};
use slang::smt::*;
use slang_ui::prelude::*;

pub struct App;

impl slang_ui::Hook for App {
    fn analyze(&self, cx: &mut slang_ui::Context, file: &slang::SourceFile) -> Result<()> {
        // Get reference to Z3 solver 
        let mut solver = cx.solver()?;

        // Iterate methods
        for m in file.methods() {
            // Get method's preconditions;
            let pres = m.requires();
            // Merge them into a single condition
            let pre = pres.cloned().reduce(|a, b| a & b).unwrap_or(Expr::bool(true));
            // Convert the expression into an SMT expression
            let spre = pre.smt()?;
            // Assert precondition
            solver.assert(spre.as_bool()?)?;

            // Get method's body
            let stmt = &m.body.clone().unwrap().stmt;
            // Encode it in IVL
            let ivl = stmt_to_ivlstmt(&stmt)?;
            // Calculate obligation and error message (if obligation is not verified)
            let (oblig, msg) = wp(&ivl, &Expr::bool(true))?;
            // Convert obligation to SMT expression
            let soblig = oblig.smt()?;

            // Run the following solver-related statements in a closed scope. That is,
            // after exiting the scope, all assertions are forgotten from subsequent 
            // executions of the solver
            solver.scope(|solver| {
                // Check validity of obligation
                solver.assert(!soblig.as_bool()?)?;
                // Run SMT solver on all current assertions
                match solver.check_sat_with_model()? {
                    // If the obligations result not valid, report the error 
                    // (on the span in which the error happens)
                    smtlib::SatResultWithModel::Sat(model) => {
                        cx.error(oblig.span, format!("{msg}: {model}"));
                    }
                    smtlib::SatResultWithModel::Unknown => {
                        cx.warning(oblig.span, "{msg}: unknown sat result");
                    }
                    smtlib::SatResultWithModel::Unsat => ()
                }
                Ok(())
            })?;
        };
        
        Ok(())
    }
}

// Encoding of (assert-only) statements into IVL (for programs comprised of only a single assertion)
fn stmt_to_ivlstmt(stmt: &Stmt) -> Result<IVLStmt> {
    match &stmt.kind {
        StmtKind::Assert { condition, .. } => 
            Ok(IVLStmt::assert(condition, "Assert might fail!")),
        _ => bail!("Not supported.")
    }
}

// Weakest precondition of (assert-only) IVL programs comprised of a single assertion
fn wp(ivl: &IVLStmt, _: &Expr) -> Result<(Expr, String)> {
    match &ivl.kind {
        IVLStmtKind::Assert { condition, message } => 
            Ok((condition.clone(), message.clone())),
        _ => bail!("Not supported."),
    }
}
