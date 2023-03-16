use hir_def::DefWithBodyId;
use hir_ty::mir::{BasicBlockId, LocalId, MirBody, StatementKind, Terminator};
use im::HashMap;

use crate::{
    lazy::LazyCell,
    model::{BlockTree, Model, Statement},
};

pub fn traverse(body: &MirBody) -> Model {
    let mut locals = HashMap::new();
    for (x, y) in body.binding_locals.iter() {}
    for i in 0..body.arg_count {
        // locals.insert(, i)
    }
    let root = traverse_bb(body, body.start_block, &locals, &HashMap::new(), 0);
    Model { root }
}

fn traverse_bb(
    body: &MirBody,
    idx: BasicBlockId,
    locals: &LocalsMap,
    ancestors: &AncestorMap,
    depth: usize,
) -> BlockTree {
    // IDEA: recursively enumerate basic blocks, interprocedurally, until we hit recursion boundaries.
    // Recursion boundaries are encoded by describing how far up the tree to go.

    // let statements = Vec::new();

    let with_me = LazyCell::new(|| {
        let mut ancestors = ancestors.clone();
        ancestors.insert((body.owner, idx), depth);
        ancestors
    });

    let block = &body.basic_blocks[idx];
    for s in block.statements.iter() {
        // match &s.kind {
        //     StatementKind::Assign(place, value) => todo!(),
        //     StatementKind::Deinit(_) => todo!(),
        //     StatementKind::StorageLive(_) => todo!(),
        //     StatementKind::StorageDead(_) => todo!(),
        //     StatementKind::Nop => todo!(),
        // }
    }

    if let Some(t) = block.terminator.as_ref() {
        let result = match t {
            Terminator::Goto { target } => traverse_jump(body, idx, locals, &*with_me, depth),
            Terminator::SwitchInt { discr, targets } => todo!(),
            Terminator::Resume => todo!(),
            Terminator::Abort => todo!(),
            Terminator::Return => todo!(),
            Terminator::Unreachable => todo!(),
            Terminator::Drop { place, target, unwind } => {
                let ty = &body.locals[place.local].ty;
                todo!("look up destructor")
            }
            Terminator::DropAndReplace { place, value, target, unwind } => todo!(),
            Terminator::Call { func, args, destination, target, cleanup, from_hir_call } => todo!(),
            Terminator::Assert { cond, expected, target, cleanup } => todo!(),
            Terminator::Yield { value, resume, resume_arg, drop } => todo!(),
            Terminator::GeneratorDrop => todo!(),
            Terminator::FalseEdge { real_target, imaginary_target } => todo!(),
            Terminator::FalseUnwind { real_target, unwind } => todo!(),
        };
    }

    // BlockTree { statements }
    todo!()
}

fn traverse_jump(
    body: &MirBody,
    idx: BasicBlockId,
    locals: &LocalsMap,
    ancestors: &AncestorMap,
    depth: usize,
) -> Statement {
    let key = (body.owner, idx);
    if let Some(parent_depth) = ancestors.get(&key) {
        Statement::CallRecursive(depth - parent_depth)
    } else {
        Statement::CallEmbedded(traverse_bb(body, idx, locals, ancestors, depth + 1))
    }
}

type AncestorMap = HashMap<(DefWithBodyId, BasicBlockId), usize>;
type LocalsMap = HashMap<LocalId, usize>;
