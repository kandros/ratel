pub struct Tool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub input_schema: schemars::Schema,
    pub output_schema: schemars::Schema,
}

impl Tool {
    /// Build from [`serde_json::Value`] as received over FFI (e.g. NAPI). Each schema must be a
    /// JSON object or boolean (JSON Schema document root).
    pub fn try_from_json_schemas(
        id: String,
        name: String,
        description: String,
        input_schema: serde_json::Value,
        output_schema: serde_json::Value,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id,
            name,
            description,
            input_schema: input_schema.try_into()?,
            output_schema: output_schema.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Tool;
    use serde_json::json;

    #[test]
    fn try_from_json_schemas_accepts_objects() {
        let t = Tool::try_from_json_schemas(
            "id".into(),
            "name".into(),
            "desc".into(),
            json!({"type": "object"}),
            json!({}),
        )
        .unwrap();
        assert_eq!(t.id, "id");
    }

    #[test]
    fn try_from_json_schemas_accepts_bool_schemas() {
        assert!(
            Tool::try_from_json_schemas(
                "i".into(),
                "n".into(),
                "d".into(),
                json!(true),
                json!(false),
            )
            .is_ok()
        );
    }

    #[test]
    fn try_from_json_schemas_rejects_string() {
        assert!(
            Tool::try_from_json_schemas(
                "i".into(),
                "n".into(),
                "d".into(),
                json!("invalid"),
                json!({}),
            )
            .is_err()
        );
    }
}
