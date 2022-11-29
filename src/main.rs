use parse_data::SysGroup;
use plot_data::plot_multiple_groups;

mod clean_data;
mod parse_data;
mod plot_data;

fn main() {
    // already done
    // clean_data::clean_dir();

    let send_sys_stats = parse_data::data_from_dir("./data/send_sys");
    let ktls_send_sys_stats = parse_data::data_from_dir("./data/ktls_send_sys");
    let ktls_sendfile_sys_stats = parse_data::data_from_dir("./data/ktls_sendfile_sys");

    // plot_cpu(
    //     send_sys_stats.clone(),
    //     ktls_send_sys_stats.clone(),
    //     ktls_sendfile_sys_stats.clone(),
    // );
    // plot_net_tx(
    //     send_sys_stats.clone(),
    //     ktls_send_sys_stats.clone(),
    //     ktls_sendfile_sys_stats.clone(),
    // );
    plot_by_payload_size(send_sys_stats, ktls_send_sys_stats, ktls_sendfile_sys_stats);
}

// --------- net rx: is not interesting since we sent data in the scenario
fn plot_by_payload_size(
    send_sys_stats: SysGroup,
    ktls_send_sys_stats: SysGroup,
    ktls_sendfile_sys_stats: SysGroup,
) {
    let len = send_sys_stats.sys.len();

    let mut group_by_payload = Vec::new();
    for i in 0..len {
        let send = send_sys_stats.sys.get(i).unwrap().clone();
        let ktl_send = ktls_send_sys_stats.sys.get(i).unwrap().clone();
        let ktl_sendfile = ktls_sendfile_sys_stats.sys.get(i).unwrap().clone();

        let grp = SysGroup {
            title: "".to_string(),
            sys: vec![send, ktl_send, ktl_sendfile],
        };
        group_by_payload.push(grp);
    }

    let show = true;
    plot_multiple_groups(
        group_by_payload.clone(),
        |sys_stat| sys_stat.cpu,
        "sec vs cpu %",
        show,
    );

    plot_multiple_groups(
        group_by_payload,
        |sys_stat| sys_stat.net_tx.iter().map(|v| v.get_bytes()).collect(),
        "sec vs net_tx bytes",
        show,
    );
}

fn plot_cpu(
    send_sys_stats: SysGroup,
    ktls_send_sys_stats: SysGroup,
    ktls_sendfile_sys_stats: SysGroup,
) {
    let show = true;
    plot_data::plot_stats(
        send_sys_stats,
        |sys_stat| sys_stat.cpu,
        "sec vs cpu %",
        show,
    );
    plot_data::plot_stats(
        ktls_send_sys_stats,
        |sys_stat| sys_stat.cpu,
        "sec vs cpu %",
        show,
    );
    plot_data::plot_stats(
        ktls_sendfile_sys_stats,
        |sys_stat| sys_stat.cpu,
        "sec vs cpu %",
        show,
    );
}

// --------- net rx: is not interesting since we sent data in the scenario
fn plot_net_tx(
    send_sys_stats: SysGroup,
    ktls_send_sys_stats: SysGroup,
    ktls_sendfile_sys_stats: SysGroup,
) {
    let show = true;
    plot_data::plot_stats(
        send_sys_stats,
        |sys_stat| sys_stat.net_tx.iter().map(|v| v.get_bytes()).collect(),
        "sec vs net_tx bytes",
        show,
    );
    plot_data::plot_stats(
        ktls_send_sys_stats,
        |sys_stat| sys_stat.net_tx.iter().map(|v| v.get_bytes()).collect(),
        "sec vs net_tx bytes",
        show,
    );
    plot_data::plot_stats(
        ktls_sendfile_sys_stats,
        |sys_stat| sys_stat.net_tx.iter().map(|v| v.get_bytes()).collect(),
        "sec vs net_tx bytes",
        show,
    );
}
