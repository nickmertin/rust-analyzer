//! Extracted base model of the relevant Rust code.

#[derive(Debug)]
pub struct Model {
    pub root: BlockTree,
}

#[derive(Debug)]
pub struct BlockTree {
    pub statements: Vec<Statement>,
    pub terminator: Terminator,
}

#[derive(Debug)]
pub enum Statement {
    CallEmbedded(BlockTree),
    CallRecursive(usize),
}

#[derive(Debug)]
pub enum Terminator {}
