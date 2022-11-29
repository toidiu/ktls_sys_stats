use crate::parse_data::{SysGroup, SysStats};
use plotly::{layout::GridPattern, layout::Layout, layout::LayoutGrid, Plot, Scatter};

pub fn plot_stats<F, T>(sys_group: SysGroup, f: F, title: &str, show: bool)
where
    F: FnOnce(SysStats) -> Vec<T> + std::marker::Copy,
    T: serde::ser::Serialize + std::clone::Clone + 'static,
{
    let mut plot = Plot::new();

    let _len = sys_group.sys.len();
    let mut subplot = 1;
    for stat in sys_group.sys.into_iter() {
        let _subplot_name = if subplot == 1 {
            "".to_string()
        } else {
            subplot.to_string()
        };
        let name = stat.title.clone();
        let trace = Scatter::new(stat.sec.clone(), f(stat))
            .name(name)
            .x_axis("x")
            .y_axis("y");
        plot.add_trace(trace);
        subplot += 1;
    }

    let layout = Layout::new()
        .title(
            format!("{} ({})", sys_group.title.as_str(), title)
                .as_str()
                .into(),
        )
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
    // println!("{}", plot.to_inline_html(Some("simple_subplot")));
}

// fn error_plot() {
//     let mut plot = Plot::new();
//     let ed = ErrorData::array(ErrorData::new(ErrorType::Data), vec![1.0, 2.0, 3.0]);
//     let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]).error_y(ed);
//     plot.add_trace(trace);

//     plot.write_html("out.html");

//     // plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0);
// }
