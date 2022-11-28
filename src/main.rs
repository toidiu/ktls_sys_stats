use plotly::{
    common::{ErrorData, ErrorType},
    layout::GridPattern,
    layout::Layout,
    layout::LayoutGrid,
    Plot, Scatter,
};
use std::io::BufRead;
use std::{fs, io::BufReader};

mod clean_data;

fn main() {
    // clean_data::clean_dir();

    data_from_dir();
    // simple_subplot(true);
}

fn simple_subplot(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(2)
            .columns(1)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("simple_subplot")));
}

fn error_plot() {
    let mut plot = Plot::new();
    let ed = ErrorData::array(ErrorData::new(ErrorType::Data), vec![1.0, 2.0, 3.0]);
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]).error_y(ed);
    plot.add_trace(trace);

    plot.write_html("out.html");

    // plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0);
}

pub fn data_from_dir() {
    let dir = "./data/send_sys";
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let file = fs::File::open(path.unwrap().path()).unwrap();
        // println!("{}", path.as_ref().unwrap().path().display());

        let mut sys: Option<Sys> = None;
        for line in BufReader::new(file).lines() {
            let update = if let Some(mut sys) = sys.take() {
                sys.push(line.unwrap());
                // println!("{}", line.unwrap());
                Some(sys)
            } else {
                Some(Sys::new(line.unwrap()))
            };

            sys = update;
        }

        println!("{:?}", sys.unwrap());
    }
}

#[derive(Debug)]
struct Sys {
    title: Vec<String>,
    sec: Vec<f64>,
    cpu: Vec<f64>,
    net_rx: Vec<f64>,
    net_tx: Vec<f64>,
}

impl Sys {
    fn new(s: String) -> Self {
        let s: Vec<String> = s.split(',').map(String::from).collect();
        Sys {
            title: s,
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
        self.sec
            .push(s.next().expect("next").parse().expect("parse"));
    }
}
