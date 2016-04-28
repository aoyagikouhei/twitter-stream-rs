extern crate time;
extern crate rand;
extern crate url;

mod param_list;
mod encode;

use param_list::*;
use std::borrow::Cow;
use rand::Rng;

pub struct TwitterStream<'a> {
    consumer_key: Cow<'a, str>,
    consumer_secret: Cow<'a, str>,
    access_key: Cow<'a, str>,
    access_secret: Cow<'a, str>,
}

impl<'a> TwitterStream<'a> {
    pub fn new<CK, CS, AK, AS>(
        consumer_key: CK, 
        consumer_secret: CS, 
        access_key: AK, 
        access_secret: AS
    ) -> TwitterStream<'a>
        where
            CK: Into<Cow<'a, str>>,
            CS: Into<Cow<'a, str>>,
            AK: Into<Cow<'a, str>>,
            AS: Into<Cow<'a, str>>
    {
        TwitterStream {
            consumer_key: consumer_key.into(),
            consumer_secret: consumer_secret.into(),
            access_key: access_key.into(),
            access_secret: access_secret.into(),
        }
    }

    fn get_timestamp(&self) -> String 
    {
      return format!("{}", time::now_utc().to_timespec().sec);
    }

    fn get_nonce(&self) -> String 
    {
      return rand::thread_rng().gen_ascii_chars().take(32).collect::<String>();
    }

    pub fn filter(&self, track: &'a str) -> String
    {
      let mut params = ParamList::new();
      params.insert_param("abc", "efg");
      return track.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ts = TwitterStream::new("ck", "cs", "ak", "as");
        assert!(ts.get_timestamp().len() > 0);
        assert!(ts.get_nonce().len() > 0);
        assert_eq!(ts.filter("aaa"), "aaa");
    }
}
