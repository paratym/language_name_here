use io::{FsImporter, Importer};

pub mod ast;
pub mod io;
pub mod tok;

fn main() {
    let mut importer = FsImporter::new("./tour");
    importer.import_path(&"./".into()).unwrap();
}
