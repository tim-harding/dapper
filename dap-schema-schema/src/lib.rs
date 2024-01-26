use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub title: String,
    pub description: String,
    pub r#type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &'static str = include_str!("../../debugAdapterProtocol.json");

    #[test]
    fn read_schema() {
        let _schema: Schema = serde_json::de::from_str(JSON).unwrap();
    }
}
