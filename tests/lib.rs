extern crate twitter_stream_rs;

use twitter_stream_rs::*;

#[test]
fn it_works() {
    let ts = TwitterStream::new("ck", "cs", "ak", "as");
}
