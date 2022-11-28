use plotly::{
    common::{ErrorData, ErrorType},
    layout::GridPattern,
    layout::Layout,
    layout::LayoutGrid,
    Plot, Scatter,
};

mod clean_data;

fn main() {
    // clean_data::clean_dir();

    println!("Hello, world!");
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
