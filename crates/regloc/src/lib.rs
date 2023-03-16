mod lazy;
mod model;
mod traverse_mir;
mod traverse_hir;

use std::any::Any;

use anyhow::Error;
use hir::db::DefDatabase;
use ide_db::{regloc::RegressionRoot, RootDatabase};
use syntax::AstNode;

// use tokio::runtime;

pub fn run(_analysis: &dyn Any, db: &RootDatabase, regression: String) -> Result<(), Error> {
    // let runtime = runtime::Builder::new_multi_thread().enable_all().build()?;

    // runtime.block_on(future)

    // let symbol = analysis
    //     .symbol_search(Query::new(regression.clone()))?
    //     .pop()
    //     .ok_or_else(|| format!("No match for search: {}", regression));

    eprintln!("Finding regression root...");

    let root = RegressionRoot::find(db, regression).expect("No match for query");

    eprintln!("{:?}", root.def);
    // eprintln!("{:#?}", root.def.ret_type(db));

    let body = db.body(root.def.id().into());
    let scope = root.sema.scope(root.item.syntax()).expect("Failed to find scope");

    eprintln!("{:#?}", scope);

    // let body =
    //     db.mir_body(DefWithBodyId::FunctionId(root.def.into())).expect("Could not generate MIR");

    // eprintln!("{:#?}", body);

    // let model = traverse(&*body);

    // eprintln!("{:#?}", model);

    Ok(())
}
