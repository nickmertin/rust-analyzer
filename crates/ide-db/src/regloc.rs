#![cfg(feature = "regloc")]

use hir::{symbols::FileSymbol, Function, Semantics};
use syntax::{algo::find_node_at_range, ast::Fn};

use crate::{
    symbol_index::{world_symbols, Query},
    RootDatabase,
};

#[derive(Debug)]
pub struct RegressionRoot<'a> {
    pub sema: Semantics<'a, RootDatabase>,
    pub symbol: FileSymbol,
    pub item: Fn,
    pub def: Function,
}

impl RegressionRoot<'_> {
    pub fn find(db: &RootDatabase, query: String) -> Option<RegressionRoot<'_>> {
        let symbol = world_symbols(db, Query::new(query)).pop()?;
        let sema = Semantics::new(db);
        let source_file = sema.parse_or_expand(symbol.loc.hir_file_id)?;
        let item: Fn = find_node_at_range(&source_file, symbol.loc.ptr.text_range())?;
        let def = sema.to_def(&item)?;

        Some(RegressionRoot { sema, symbol, item, def })
    }
}

pub struct RegressionNode {
    pub def: Function,
}

impl RegressionNode {
    pub fn body(&self, db: &RootDatabase) -> Option<()> {
        // db.body(self.def.id);
        None
    }
}
