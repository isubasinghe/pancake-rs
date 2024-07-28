use std::collections::HashMap;
use std::sync::Arc;

use crate::ast::*;

pub struct CtxX {
    fns: HashMap<Ident, Function>,
}

pub type Ctx = Arc<CtxX>;

pub struct FnCtx {
    gctx: Ctx,
}

pub fn type_check_program(fns: Vec<Function>) {
    let mut kfns = HashMap::new();

    for f in fns {
        kfns.insert(f.name.clone(), f);
    }
    let ctx = Arc::new(CtxX { fns: kfns });

    for (_, f) in &ctx.fns {
        let mut fnctx = FnCtx { gctx: ctx.clone() };
        for stmt in &f.body {
            type_check_stmt(&mut fnctx, stmt.clone());
        }
    }
}
pub enum Typ {
    Bool,
    Nat,
}

pub fn type_check_expr(fnctx: &mut FnCtx, expr: Expr, typ: Typ) -> bool {
    false
}

pub fn type_check_stmt(fnctx: &mut FnCtx, stmt: Statement) -> bool {
    match &*stmt {
        StatementX::If(e, st) => {
            let valid = type_check_expr(fnctx, e.clone(), Typ::Bool);
            assert!(valid);

            for s in st {
                let valid = type_check_stmt(fnctx, s.clone());
                assert!(valid);
            }
        }
        StatementX::While(e, stmts) => {}
        StatementX::Return(e) => {}
        StatementX::DeclAssign(id, e) => {}
    };
    false
}
