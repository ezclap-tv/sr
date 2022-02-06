use serde::{Deserialize, Deserializer};

fn inner<'de, D>(v: &'de str) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  Ok(
    match v {
      "youtube" => v,
      _ => return Err(serde::de::Error::custom("valid values are: youtube")),
    }
    .to_string(),
  )
}

pub fn known<'de, D>(d: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  inner::<'de, D>(<&'de str as Deserialize>::deserialize(d)?)
}

pub fn known_opt<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
  D: Deserializer<'de>,
{
  match Option::<&'de str>::deserialize(d)? {
    Some(v) => inner::<'de, D>(v).map(Some),
    None => Ok(None),
  }
}
