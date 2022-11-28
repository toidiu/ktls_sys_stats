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

        temp.write_all(b"sec, CPU, net_name, net_rx, net_tx\n")
            .unwrap();

        let mut ts = 0.0;
        for line in BufReader::new(file).lines() {
            let mut line = line.unwrap();
            line.push('\n');
            // 338643, 0| ens5 164 472| lo 0 0|
            let mut net_split = line.split('|');

            let cpu = net_split.next().unwrap().split_whitespace().last().unwrap();

            let net: String = net_split.filter(|e| e.contains("ens5")).collect();
            // println!("{:?}", net.len());

            let net: Vec<&str> = net.split_whitespace().collect();
            let (rx, tx) = (net.get(1).unwrap(), net.get(2).unwrap());

            let data = format!("{}, {}, {}, {}\n", ts, cpu, rx, tx);
            println!("{}", data);

            temp.write_all(data.as_bytes()).unwrap();

            ts += 0.5;
        }
    }

    fs::rename(temp.into_temp_path(), path).unwrap();
}
