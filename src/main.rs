mod clean_data;
mod parse_data;
mod plot_data;

fn main() {
    // already done
    // clean_data::clean_dir();

    let send_sys_stats = parse_data::data_from_dir("./data/send_sys");
    let ktls_send_sys_stats = parse_data::data_from_dir("./data/ktls_send_sys");
    let ktls_sendfile_sys_stats = parse_data::data_from_dir("./data/ktls_sendfile_sys");

    plot_data::plot_stats(send_sys_stats, true);
    plot_data::plot_stats(ktls_send_sys_stats, true);
    plot_data::plot_stats(ktls_sendfile_sys_stats, true);
}
