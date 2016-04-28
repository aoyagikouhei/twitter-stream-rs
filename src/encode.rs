use url::percent_encoding;

pub fn encode(s: &str) -> String {
    percent_encoding::utf8_percent_encode(
        s, 
        percent_encoding::QUERY_ENCODE_SET
    ).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("%E3%81%82", encode("あ"));
    }
}
