use crate::parse_data::{SysGroup, SysStats};
use plotly::{layout::GridPattern, layout::Layout, layout::LayoutGrid, Plot, Scatter};

pub fn plot_stats(sys_group: SysGroup, show: bool) {
    // simple_subplot(true);
    // return;

    let mut plot = Plot::new();

    let _len = sys_group.sys.len();
    let mut subplot = 1;
    for stat in sys_group.sys.into_iter() {
        let _subplot_name = if subplot == 1 {
            "".to_string()
        } else {
            subplot.to_string()
        };
        let trace = Scatter::new(stat.sec.clone(), stat.cpu.clone())
            .name(stat.title.clone())
            .x_axis("x")
            .y_axis("y");
        plot.add_trace(trace);
        subplot += 1;
    }

    let layout = Layout::new()
        .title(sys_group.title.as_str().into())
        .show_legend(true)
        .grid(
            LayoutGrid::new()
                .rows(1)
                .columns(1)
                .pattern(GridPattern::Independent),
        );
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("simple_subplot")));
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
            .rows(1)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("simple_subplot")));
}

// fn error_plot() {
//     let mut plot = Plot::new();
//     let ed = ErrorData::array(ErrorData::new(ErrorType::Data), vec![1.0, 2.0, 3.0]);
//     let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]).error_y(ed);
//     plot.add_trace(trace);

//     plot.write_html("out.html");

//     // plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0);
// }
