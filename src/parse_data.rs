use byte_unit::Byte;
use regex::Regex;
use std::io::BufRead;
use std::path::Path;
use std::{fs, io::BufReader};

pub fn data_from_dir(dir: &str) -> SysGroup {
    let dir_name = Path::new(dir).iter().last().unwrap().to_str().unwrap();

    let paths = fs::read_dir(dir).unwrap();
    let mut paths: Vec<String> = paths
        .map(|path| path.unwrap().path().to_string_lossy().to_string())
        .collect();
    sort_file_by_size(&mut paths);

    let mut sys_stats = Vec::new();
    for path in paths {
        let file = fs::File::open(path.clone()).unwrap();
        // println!("{}", path);

        let mut sys: Option<SysStats> = None;
        for line in BufReader::new(file).lines() {
            let update = if let Some(mut sys) = sys.take() {
                sys.push(line.unwrap());
                // println!("{}", line.unwrap());
                Some(sys)
            } else {
                let path = path
                    .to_string()
                    .split('/')
                    .last()
                    .unwrap()
                    .strip_prefix("sys_")
                    .unwrap()
                    .to_string();
                Some(SysStats::new(path, line.unwrap()))
            };

            sys = update;
        }

        // println!("{:?}", sys.as_ref().unwrap());
        sys_stats.push(sys.unwrap());
    }

    SysGroup {
        title: dir_name.to_string(),
        sys: sys_stats,
    }
}

fn sort_file_by_size(paths: &mut [String]) {
    // sort by the payload size
    paths.sort_by(|a, b| {
        let a = a.split('/').last().unwrap();
        let b = b.split('/').last().unwrap();

        let re = Regex::new(r"^[a-z]*_[a-z]*_*[a-z]*").unwrap();
        let a = re.replace(a, "");
        let b = re.replace(b, "");
        let a = Byte::from_str(a).unwrap();
        let b = Byte::from_str(b).unwrap();

        a.cmp(&b)
    });
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
