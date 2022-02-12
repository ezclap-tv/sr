use serde::{Deserialize, Deserializer};

fn parse_0_or_1<E>(v: &str) -> Result<bool, E>
where
  E: serde::de::Error,
{
  v.parse::<u8>()
    .map_err(serde::de::Error::custom)
    .and_then(|v| {
      if v == 0 {
        Ok(false)
      } else if v == 1 {
        Ok(true)
      } else {
        Err(serde::de::Error::custom("must be 0 or 1"))
      }
    })
}

fn parse_bool<E>(v: &str) -> Result<bool, E>
where
  E: serde::de::Error,
{
  v.parse::<bool>().map_err(serde::de::Error::custom)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  let v = <&'de str as Deserialize<'de>>::deserialize(deserializer)?;
  if v.is_empty() {
    Ok(true)
  } else {
    parse_0_or_1(v).or_else(|_: D::Error| parse_bool(v))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize, PartialEq)]
  struct Test {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize")]
    v: bool,
  }

  const TRUE: Test = Test { v: true };
  const FALSE: Test = Test { v: false };

  fn de(v: &str) -> Test {
    serde_urlencoded::from_str::<Test>(v).unwrap()
  }

  #[test]
  fn parses_numeric() {
    assert_eq!(de("v=0"), FALSE);
    assert_eq!(de("v=1"), TRUE);
  }

  #[test]
  fn parses_bool() {
    assert_eq!(de("v=false"), FALSE);
    assert_eq!(de("v=true"), TRUE);
  }

  #[test]
  fn parses_unit() {
    assert_eq!(de("v"), TRUE);
    assert_eq!(de(""), FALSE);
  }
}
