use crate::{
    ast::{Expr, GlobalEvalScope},
    io::IoResult,
};

pub trait Importer {
    type Path;
    fn resolve_path(&self, path: &Expr) -> IoResult<Self::Path>;
    fn import_path(&mut self, path: &Self::Path) -> IoResult<GlobalEvalScope>;
}
