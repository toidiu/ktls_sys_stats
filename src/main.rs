mod clean_data;
mod parse_data;
mod plot_data;

fn main() {
    // already done
    // clean_data::clean_dir();

    let send_sys_stats = parse_data::data_from_dir("./data/send_sys");
    let ktls_send_sys_stats = parse_data::data_from_dir("./data/ktls_send_sys");
    let ktls_sendfile_sys_stats = parse_data::data_from_dir("./data/ktls_sendfile_sys");

    let mut show = true;
    plot_data::plot_stats(
        send_sys_stats.clone(),
        |sys_stat| sys_stat.cpu,
        "cpu %",
        show,
    );
    plot_data::plot_stats(
        ktls_send_sys_stats.clone(),
        |sys_stat| sys_stat.cpu,
        "cpu %",
        show,
    );
    plot_data::plot_stats(
        ktls_sendfile_sys_stats.clone(),
        |sys_stat| sys_stat.cpu,
        "cpu %",
        show,
    );

    // --------- net tx
    plot_data::plot_stats(
        send_sys_stats.clone(),
        |sys_stat| sys_stat.net_tx.iter().map(|v| v.get_bytes()).collect(),
        "net tx bytes",
        show,
    );
    plot_data::plot_stats(
        ktls_send_sys_stats.clone(),
        |sys_stat| sys_stat.net_tx.iter().map(|v| v.get_bytes()).collect(),
        "net tx bytes",
        show,
    );
    plot_data::plot_stats(
        ktls_sendfile_sys_stats.clone(),
        |sys_stat| sys_stat.net_tx.iter().map(|v| v.get_bytes()).collect(),
        "net tx bytes",
        show,
    );

    // --------- net rx: not interesting since we sent data in the scenario
    show = false;
    plot_data::plot_stats(
        send_sys_stats,
        |sys_stat| sys_stat.net_rx.iter().map(|v| v.get_bytes()).collect(),
        "net rx bytes",
        show,
    );
    plot_data::plot_stats(
        ktls_send_sys_stats,
        |sys_stat| sys_stat.net_rx.iter().map(|v| v.get_bytes()).collect(),
        "net rx bytes",
        show,
    );
    plot_data::plot_stats(
        ktls_sendfile_sys_stats,
        |sys_stat| sys_stat.net_rx.iter().map(|v| v.get_bytes()).collect(),
        "net rx bytes",
        show,
    );
}
