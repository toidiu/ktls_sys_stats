use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};
use tempfile::{tempfile, NamedTempFile};

pub(crate) fn clean_dir(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        println!("{}", path.as_ref().unwrap().path().display());
        clean_file(path.unwrap().path())
    }
}

fn clean_file(path: PathBuf) {
    let mut temp = NamedTempFile::new().unwrap();
    // read file and close it before writing to it again
    {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path.clone())
            .unwrap();

        for line in BufReader::new(file).lines() {
            let mut line = line.unwrap();
            line.push('\n');
            temp.write_all(line.as_bytes()).unwrap();
        }
    }

    fs::rename(temp.into_temp_path(), path).unwrap();
}
