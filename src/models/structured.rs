use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// A strongly‑typed representation of a JSON Schema definition.
/// Supports object schemas (with `properties`) and array schemas (with `items`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum JsonSchemaDefinition {
    /// Object schema validated via property definitions.
    Object {
        /// A map of property names to their definitions.
        properties: Map<String, Value>,
        /// List of required property names.
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<Vec<String>>,
        /// Indicates whether additional properties are allowed.
        /// Enum variant renaming does not rename its fields.
        #[serde(
            rename = "additionalProperties",
            skip_serializing_if = "Option::is_none"
        )]
        additional_properties: Option<bool>,
    },
    /// Array schema validated via an item definition.
    Array {
        /// The schema applied to each item in the array.
        items: Box<JsonSchemaDefinition>,
    },
}

/// JSON Schema configuration for requesting structured outputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchemaConfig {
    /// Name for schema, used to identify output type.
    pub name: String,
    /// If true, model response must strictly adhere to schema.
    pub strict: bool,
    /// The JSON Schema definition.
    pub schema: JsonSchemaDefinition,
}

#[cfg(test)]
mod tests {
    use super::JsonSchemaDefinition;
    use serde_json::{json, Map};

    #[test]
    fn strict_object_schema_preserves_additional_properties() -> Result<(), serde_json::Error> {
        let schema = JsonSchemaDefinition::Object {
            properties: Map::new(),
            required: Some(Vec::new()),
            additional_properties: Some(false),
        };
        let wire = json!({
            "type": "object",
            "properties": {},
            "required": [],
            "additionalProperties": false
        });

        assert_eq!(serde_json::to_value(&schema)?, wire);
        assert_eq!(
            serde_json::from_value::<JsonSchemaDefinition>(wire)?,
            schema
        );
        Ok(())
    }
}
