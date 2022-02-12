pub trait QueryExt {
  fn query_iter<T>(self, iter: impl Iterator<Item = T>) -> Self
  where
    T: serde::Serialize;

  fn query_opt<T>(self, opt: Option<T>) -> Self
  where
    T: serde::Serialize;
}

impl QueryExt for reqwest::RequestBuilder {
  fn query_iter<T>(self, iter: impl Iterator<Item = T>) -> Self
  where
    T: serde::Serialize,
  {
    let mut builder = self;
    for item in iter {
      builder = builder.query(&[item])
    }
    builder
  }

  fn query_opt<T>(self, opt: Option<T>) -> Self
  where
    T: serde::Serialize,
  {
    match opt {
      Some(v) => self.query(&[v]),
      None => self,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use reqwest::Client;

  #[test]
  fn add_to_query_using_iter() {
    let ids: &[&str] = &["0", "1", "2", "3"];

    let request = Client::new()
      .get("http://test.com/api")
      .query_iter(ids.iter().map(|id| ("id", *id)))
      .build()
      .unwrap();

    assert_eq!(
      request.url().as_str(),
      "http://test.com/api?id=0&id=1&id=2&id=3"
    );
  }

  #[test]
  fn add_to_query_using_option() {
    let a = Some(("a", "a"));
    let b = Option::<(&str, &str)>::None;

    let request = Client::new()
      .get("http://test.com/api")
      .query_opt(a)
      .query_opt(b)
      .build()
      .unwrap();

    assert_eq!(request.url().as_str(), "http://test.com/api?a=a");
  }
}
