use crate::{
    ast::{AstNode, Decl, Expr, Stmt},
    io::{Importer, IoErr, IoResult},
    tok::{TokErr, Tokenizer},
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

    fn import_path(&mut self, scope_path: &Self::Path) -> IoResult<()> {
        let mut dir = self.root.join(scope_path).read_dir()?;

        while let Some(src) = dir.next().transpose()? {
            let path = src.path();
            if !path.extension().is_some_and(|ext| ext == SRC_FILE_EXT)
                || !src.file_type()?.is_file()
            {
                continue;
            }

            let file = BufReader::new(File::open(&path)?);
            let mut tok = Tokenizer::new(file);
            println!("{:?}", path);

            loop {
                if tok
                    .peek()
                    .map_err(|e| {
                        IoErr::from_parse_err(e.into(), path.to_string_lossy().to_string())
                    })?
                    .is_none()
                {
                    break;
                }

                let decl = Decl::expect(&mut tok)
                    .map_err(|e| IoErr::from_parse_err(e, path.to_string_lossy().to_string()))?;

                println!("{:?}", decl);
            }
        }

        Ok(())
    }
}
