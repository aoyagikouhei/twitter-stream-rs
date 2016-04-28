use std::borrow::Cow;
use std::collections::HashMap;
use encode::*;

pub type ParamList<'a> = HashMap<Cow<'a, str>, Cow<'a, str>>;

pub trait ParamListMethods<'a> {
    fn insert_param<K, V>(&mut self, key: K, value: V)
        where
            K: Into<Cow<'a, str>>,
            V: Into<Cow<'a, str>>;

    fn join(&self) -> String;
}

impl<'a> ParamListMethods<'a> for ParamList<'a> {
    fn insert_param<K, V>(&mut self, key: K, value: V)
        where
            K: Into<Cow<'a, str>>,
            V: Into<Cow<'a, str>>
    {
        self.insert(key.into(), value.into());
    }

    fn join(&self) -> String
    {
        let mut pairs = self.iter()
          .map(|(k, v)| format!("{}={}", encode(&k), encode(&v)))
          .collect::<Vec<_>>();
        pairs.sort();
        pairs.join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut params = ParamList::new();
        params.insert_param("abc", "efg");
        params.insert_param("hij", "lmn");
        assert_eq!("abc=efg&hij=lmn", params.join());
    }
}
