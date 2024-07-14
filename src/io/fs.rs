use crate::{
    ast::{AstNode, Expr, GlobalEvalScope},
    io::{Importer, IoErr, IoResult},
    tok::Tokenizer,
};
use std::{fs::File, io::BufReader, path::PathBuf};

pub struct FsImporter {
    root: PathBuf,
}

const SRC_FILE_EXT: &str = "idk";

impl FsImporter {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl Importer for FsImporter {
    type Path = PathBuf;

    fn resolve_path(&self, path: &Expr) -> IoResult<Self::Path> {
        todo!()
    }

    fn import_path(&mut self, scope_path: &Self::Path) -> IoResult<GlobalEvalScope> {
        let mut dir = self.root.join(scope_path).read_dir()?;
        let mut scope = GlobalEvalScope::new();

        while let Some(src) = dir.next().transpose()? {
            let path = src.path();
            if !path.extension().is_some_and(|ext| ext == SRC_FILE_EXT)
                || !src.file_type()?.is_file()
            {
                continue;
            }

            let file = BufReader::new(File::open(&path)?);
            let mut tok = Tokenizer::new(file);
            let file_scope = GlobalEvalScope::expect(&mut tok)
                .map_err(|e| IoErr::from_parse_err(e, path.to_string_lossy().to_string()))?;

            println!("{:?} {:?}", path, file_scope);
            scope.merge(file_scope);
        }

        Ok(scope)
    }
}
