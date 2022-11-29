use crate::parse_data::{SysGroup, SysStats};
use plotly::{layout::GridPattern, layout::Layout, layout::LayoutGrid, Plot, Scatter};
use regex::Regex;

pub fn plot_stats<F, T>(sys_group: SysGroup, f: F, title_units: &str, show: bool)
where
    F: FnOnce(SysStats) -> Vec<T> + std::marker::Copy,
    T: serde::ser::Serialize + std::clone::Clone + 'static,
{
    let mut plot = Plot::new();

    let mut subplot = 1;
    for stat in sys_group.sys.into_iter() {
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
            format!("{} ({})", sys_group.title.as_str(), title_units)
                .as_str()
                .into(),
        )
        .show_legend(true)
        .height(1000)
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

pub fn plot_multiple_groups<F, T>(sys_groups: Vec<SysGroup>, f: F, title_units: &str, show: bool)
where
    F: FnOnce(SysStats) -> Vec<T> + std::marker::Copy,
    T: serde::ser::Serialize + std::clone::Clone + 'static,
{
    let mut plot = Plot::new();

    let mut subplot = 1;
    for sys_group in sys_groups {
        for stat in sys_group.sys.into_iter() {
            let subplot_name = if subplot == 1 {
                "".to_string()
            } else {
                subplot.to_string()
            };
            let name = stat.title.clone();
            let re = Regex::new(r"\d*[k,b]$").unwrap();
            let legend_group = re.replace(&name, "");
            println!("{}", legend_group);

            let trace = Scatter::new(stat.sec.clone(), f(stat))
                .name(name.clone())
                .legend_group(legend_group)
                .x_axis(format!("x{}", subplot_name))
                .y_axis(format!("y{}", subplot_name));
            plot.add_trace(trace);
        }
        subplot += 1;
    }

    let layout = Layout::new()
        .title(
            format!("{} ({})", "comparison by payload", title_units)
                .as_str()
                .into(),
        )
        .show_legend(true)
        .height(1000)
        .grid(
            LayoutGrid::new()
                .rows(3)
                .columns(4)
                .pattern(GridPattern::Independent),
        );
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    // println!("{}", plot.to_inline_html(Some("simple_subplot")));
}
