use std::{
    fs::{File, ReadDir},
    io::{self, BufRead, BufReader, Error, ErrorKind, Read},
    path::PathBuf,
};

const SRC_EXT: &str = "idk";

pub struct DirReader {
    dir: ReadDir,
    reader: BufReader<File>,
}

impl DirReader {
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        let mut dir = path.read_dir()?;
        let reader = match Self::next_file(&mut dir)? {
            Some(r) => r,
            None => {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "no source files found",
                ))
            }
        };

        Ok(Self { dir, reader })
    }

    fn next_file(dir: &mut ReadDir) -> Result<Option<BufReader<File>>, io::Error> {
        loop {
            let entry = match dir.next() {
                Some(e) => e?,
                None => return Ok(None),
            };

            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == SRC_EXT) && entry.file_type()?.is_file() {
                return Ok(Some(BufReader::new(File::open(path)?)));
            }
        }
    }
}

// blame the borrow checker for these impls
impl Read for DirReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let inner = self.fill_buf()?;
        let n = inner.len();
        buf.copy_from_slice(inner);
        Ok(n)
    }
}

impl BufRead for DirReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        loop {
            let buf = self.reader.fill_buf()?;
            if !buf.is_empty() {
                return self.reader.fill_buf();
            }

            if let Some(reader) = Self::next_file(&mut self.dir)? {
                self.reader = reader;
            } else {
                return self.reader.fill_buf();
            }
        }
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}
