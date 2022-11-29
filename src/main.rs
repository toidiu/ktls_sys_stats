use byte_unit::Byte;

mod clean_data;
mod parse_data;
mod plot_data;

fn main() {
    // done. should only be done once
    // clean_data::clean_dir();

    let send_sys_stats = parse_data::data_from_dir("./data/send_sys");
    let ktls_send_sys_stats = parse_data::data_from_dir("./data/ktls_send_sys");
    let ktls_sendfile_sys_stats = parse_data::data_from_dir("./data/ktls_sendfile_sys");

    plot_data::plot_stats(send_sys_stats, true);
}
