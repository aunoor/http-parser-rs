extern crate hap_http_parser;

use self::hap_http_parser::*;

#[test]
fn test_interface() {
    let mut hp = HttpParser::new(HttpParserType::Both);

    struct Callback;

    impl HttpParserCallback for Callback {
        fn on_message_complete(&mut self, _ : &mut HttpParser) -> CallbackResult {
            Ok(ParseAction::None)
        }
    }

    let mut cb = Callback;
    hp.execute(&mut cb, &[b'a', b'b', b'c']);
}
