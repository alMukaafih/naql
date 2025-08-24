use json_strip_comments::StripComments;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::from_reader;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

pub trait ReadFromFile: DeserializeOwned {
    fn read_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let parsed = from_reader(StripComments::new(BufReader::new(File::open(path)?)))?;

        Ok(parsed)
    }
}

pub trait WriteToFile: Serialize {
    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let s = serde_json::to_string(self)?;
        let mut f = BufWriter::new(File::create(path)?);
        f.write(s.as_bytes())?;
        Ok(())
    }
}

impl<T: DeserializeOwned> ReadFromFile for T {}
impl<T: Serialize> WriteToFile for T {}
