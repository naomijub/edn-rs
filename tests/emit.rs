#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use edn_rs::Edn;

    fn edn_to_string_unwrap(s: &str) -> String {
        Edn::from_str(s).unwrap().to_string()
    }

    #[test]
    fn map_formatting() {
        let edn = edn_to_string_unwrap("{nil \"`HQ5|>^>rQNL9E..y#}U63/S_Qo- IMpw>gnM']jD\"}");
        assert_eq!(edn, "{nil \"`HQ5|>^>rQNL9E..y#}U63/S_Qo- IMpw>gnM']jD\"}");
    }

    #[test]
    fn array_formatting() {
        let edn =
            edn_to_string_unwrap("[#inst \"2014-06-01T08:11:11.296-00:00\", #inst \"2018-07-02T19:56:08.059-00:00\", #inst \"1985-05-21T13:50:33.038-00:00\"]");
        assert_eq!(edn, "[#inst \"2014-06-01T08:11:11.296-00:00\" #inst \"2018-07-02T19:56:08.059-00:00\" #inst \"1985-05-21T13:50:33.038-00:00\"]");
    }

    #[test]
    fn char_formatting() {
        let edn = edn_to_string_unwrap("(\\b \\g \\m)");
        assert_eq!(edn, "(\\b \\g \\m)");

        assert_eq!(edn_to_string_unwrap("[\\space \\@ \\` \\tab \\return \\newline \\# \\% \\' \\g \\( \\* \\j \\+ \\, \\l \\- \\. \\/ \\0 \\2 \\r \\: \\; \\< \\\\ \\] \\} \\~ \\? \\_]"),
        "[\\space \\@ \\` \\tab \\return \\newline \\# \\% \\' \\g \\( \\* \\j \\+ \\, \\l \\- \\. \\/ \\0 \\2 \\r \\: \\; \\< \\\\ \\] \\} \\~ \\? \\_]")
    }
}
