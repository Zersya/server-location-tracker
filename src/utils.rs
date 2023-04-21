pub mod change_to_lowercase {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = match Deserialize::deserialize(deserializer) {
            Ok(s) => s,
            Err(err) => return Err(serde::de::Error::custom(err)),
        };

        Ok(s.to_lowercase())
    }
}
