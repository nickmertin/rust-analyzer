use hir::{db::DefDatabase, Function, HasSource, Semantics};
use hir_def::{
    body::Body,
    expr::{BindingId, Expr, ExprId},
    FunctionId,
};
use ide_db::{regloc::RegressionRoot, RootDatabase};
use im::HashMap;
use syntax::AstNode;

use crate::model::{BlockTree, Model};

pub fn generate_model(root: &RegressionRoot) -> Model {
    let sema = &root.sema;
    let scope = sema.scope(root.item.syntax()).expect("Failed to find scope");
    let root = traverse_fn(sema, &root.def, &HashMap::new(), 0);

    Model { root }
}

fn traverse_fn(
    sema: &Semantics<'_, RootDatabase>,
    function: &Function,
    ancestors: &AncestorMap,
    depth: usize,
) -> BlockTree {
    let body = sema.db.body(function.id().into());

    let src = function.source(sema.db).expect("source");

    let mut bindings = HashMap::new();

    traverse_expr(sema, &body, body.body_expr, &bindings, ancestors, depth)
}

fn traverse_expr(
    sema: &Semantics<'_, RootDatabase>,
    body: &Body,
    expr: ExprId,
    bindings: &BindingsMap,
    ancestors: &AncestorMap,
    depth: usize,
) -> BlockTree {
    let statements = Vec::new();

    let expr = &body.exprs[expr];

    // let scope = sema.scope(expr.sou);

    let terminator = match expr {
        Expr::Missing => panic!("missing expression"),
        Expr::Path(_) => todo!(),
        Expr::If { condition, then_branch, else_branch } => todo!(),
        Expr::Let { pat, expr } => todo!(),
        Expr::Block { id, statements, tail, label } => todo!(),
        Expr::TryBlock { id, statements, tail } => todo!(),
        Expr::Async { id, statements, tail } => todo!(),
        Expr::Const { id, statements, tail } => todo!(),
        Expr::Unsafe { id, statements, tail } => todo!(),
        Expr::Loop { body, label } => todo!(),
        Expr::While { condition, body, label } => todo!(),
        Expr::For { iterable, pat, body, label } => todo!(),
        Expr::Call { callee, args, is_assignee_expr } => todo!(),
        Expr::MethodCall { receiver, method_name, args, generic_args } => todo!(),
        Expr::Match { expr, arms } => todo!(),
        Expr::Continue { label } => todo!(),
        Expr::Break { expr, label } => todo!(),
        Expr::Return { expr } => todo!(),
        Expr::Yield { expr } => todo!(),
        Expr::Yeet { expr } => todo!(),
        Expr::RecordLit { path, fields, spread, ellipsis, is_assignee_expr } => todo!(),
        Expr::Field { expr, name } => todo!(),
        Expr::Await { expr } => todo!(),
        Expr::Try { expr } => todo!(),
        Expr::Cast { expr, type_ref } => todo!(),
        Expr::Ref { expr, rawness, mutability } => todo!(),
        Expr::Box { expr } => todo!(),
        Expr::UnaryOp { expr, op } => todo!(),
        Expr::BinaryOp { lhs, rhs, op } => todo!(),
        Expr::Range { lhs, rhs, range_type } => todo!(),
        Expr::Index { base, index } => todo!(),
        Expr::Closure { args, arg_types, ret_type, body, closure_kind } => todo!(),
        Expr::Tuple { exprs, is_assignee_expr } => todo!(),
        Expr::Array(_) => todo!(),
        Expr::Literal(_) => todo!(),
        Expr::Underscore => todo!(),
    };

    BlockTree { statements, terminator }
}

type AncestorMap = HashMap<(FunctionId, ExprId), usize>;
type BindingsMap = HashMap<BindingId, usize>;
