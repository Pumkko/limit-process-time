#[cfg(test)]
mod stat_file_parser_test {
    use crate::process::stat_file_parser::parse_stat_file;

    #[test]
    fn parse_stat_file_must_return_comm_name() {
        let stat_content = "74759 (hoi4) S 74756 74755 74755 0 -1 4194304 1070889 13726 4 0 4185 714 0 2 20 0 38 0 1545851 7122784256 885973 18446744073709551615 4268032 50458741 140723794619328 0 0 0 0 0 17646 0 0 0 17 2 0 0 0 0 0 64010424 64228200 84303872 140723794625127 140723794625271 140723794625271 140723794632626 0";

        let process_info = parse_stat_file(stat_content);

        assert_eq!(process_info.name, "hoi4");
        assert_eq!(process_info.been_running_for_seconds, 0);
    }
}
