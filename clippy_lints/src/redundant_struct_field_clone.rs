use clippy_utils::{is_trait_method, mir::enclosing_mir};
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.86.0"]
    pub REDUNDANT_STRUCT_FIELD_CLONE,
    nursery,
    "default lint description"
}

declare_lint_pass!(RedundantStructFieldClone => [REDUNDANT_STRUCT_FIELD_CLONE]);

impl<'tcx> LateLintPass<'tcx> for RedundantStructFieldClone {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Struct(qpath, expr_fields, struct_tail_expr) = expr.kind {
            // dbg!(expr);
            let Some(mir) = enclosing_mir(cx.tcx, expr.hir_id) else {
                return;
            };

            // if is_trait_method(cx, n.expr, sym::clone) {
            //     dbg!("yes");
            // } else {
            //     dbg!("no");
            // }

            cx.tcx.hir_enclosing_body_owner();
            // dbg!(mir);
            for n in expr_fields {
                let node = cx.tcx.hir_node(n.hir_id);
                // dbg!(node);
                dbg!(n.expr);
                // dbg!(cx.tcx.local_def_id_to_hir_id(key))
                if let Some(mir) = enclosing_mir(cx.tcx, n.hir_id) {
                    // dbg!(mir);
                }
            }
        }
    }
}
