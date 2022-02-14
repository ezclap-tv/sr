#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Platform {
  Youtube,
}

impl Platform {
  pub fn as_str(self) -> &'static str {
    match self {
      Platform::Youtube => "youtube",
    }
  }
}
