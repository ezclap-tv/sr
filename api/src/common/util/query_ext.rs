pub trait QueryExt {
  fn query_iter<T>(self, key: &str, value: impl Iterator<Item = T>) -> Self
  where
    T: serde::Serialize;

  fn query_opt<T>(self, key: &str, value: Option<T>) -> Self
  where
    T: serde::Serialize;
}

impl QueryExt for reqwest::RequestBuilder {
  fn query_iter<T>(mut self, key: &str, value: impl Iterator<Item = T>) -> Self
  where
    T: serde::Serialize,
  {
    for item in value {
      self = self.query(&[(key, item)])
    }
    self
  }

  fn query_opt<T>(self, key: &str, value: Option<T>) -> Self
  where
    T: serde::Serialize,
  {
    match value {
      Some(v) => self.query(&[(key, v)]),
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
      .query_iter("id", ids.iter())
      .build()
      .unwrap();

    assert_eq!(request.url().as_str(), "http://test.com/api?id=0&id=1&id=2&id=3");
  }

  #[test]
  fn add_to_query_using_option() {
    let a = Some("a");
    let b = Option::<&str>::None;

    let request = Client::new()
      .get("http://test.com/api")
      .query_opt("a", a)
      .query_opt("b", b)
      .build()
      .unwrap();

    assert_eq!(request.url().as_str(), "http://test.com/api?a=a");
  }
}
