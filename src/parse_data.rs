use byte_unit::Byte;
use std::io::BufRead;
use std::path::Path;
use std::{fs, io::BufReader};

pub fn data_from_dir(dir: &str) -> SysGroup {
    let dir_name = Path::new(dir).iter().last().unwrap().to_str().unwrap();
    let paths = fs::read_dir(dir).unwrap();

    let mut sys_stats = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let file = fs::File::open(path.clone()).unwrap();
        // println!("{}", path.as_ref().unwrap().path().display());

        let mut sys: Option<SysStats> = None;
        for line in BufReader::new(file).lines() {
            let update = if let Some(mut sys) = sys.take() {
                sys.push(line.unwrap());
                // println!("{}", line.unwrap());
                Some(sys)
            } else {
                Some(SysStats::new(
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                    line.unwrap(),
                ))
            };

            sys = update;
        }

        println!("{:?}", sys.as_ref().unwrap());
        sys_stats.push(sys.unwrap());
    }
    SysGroup {
        title: dir_name.to_string(),
        sys: sys_stats,
    }
}

pub struct SysGroup {
    pub title: String,
    pub sys: Vec<SysStats>,
}

#[derive(Debug)]
pub struct SysStats {
    pub title: String,
    pub legend: Vec<String>,
    pub sec: Vec<f64>,
    pub cpu: Vec<f64>,
    pub net_rx: Vec<Byte>,
    pub net_tx: Vec<Byte>,
}

impl SysStats {
    fn new(title: String, legend: String) -> Self {
        let legend: Vec<String> = legend.split(',').map(String::from).collect();
        SysStats {
            title,
            legend,
            sec: Vec::new(),
            cpu: Vec::new(),
            net_rx: Vec::new(),
            net_tx: Vec::new(),
        }
    }

    fn push(&mut self, s: String) {
        let mut s = s.split(',').map(|s| String::from(s.trim()));

        self.sec
            .push(s.next().expect("next").parse().expect("parse"));
        self.cpu
            .push(s.next().expect("next").parse().expect("parse"));
        self.net_rx
            .push(s.next().expect("next").parse().expect("parse"));
        self.net_tx
            .push(s.next().expect("next").parse().expect("parse"));
    }
}
