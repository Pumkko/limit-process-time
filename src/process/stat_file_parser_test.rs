#[cfg(test)]
mod stat_file_parser_test {
    use crate::process::stat_file_parser::parse_stat_file;

    #[test]
    fn parse_stat_file_must_return_comm_name() {
        let stat_content = "92121 (hoi4) R 92118 92117 92117 0 -1 4194304 1069577 13732 0 0 2694 455 0 2 20 0 38 0 2086189 7127048192 888530 18446744073709551615 4268032 50458741 140730131163376 0 0 0 0 0 17646 0 0 0 17 9 0 0 0 0 0 64010424 64228200 79720448 140730131165799 140730131165943 140730131165943 140730131173298 0";

        let process_info = parse_stat_file(stat_content);

        assert_eq!(process_info.name, "hoi4");
        assert_eq!(process_info.been_running_for_seconds, 0);
    }
}
