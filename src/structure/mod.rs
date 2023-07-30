use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
pub mod default_config;

#[napi(object, js_name = "IOptions")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  /// Interface prefix, default value is "I"
  pub prefix: Option<String>,

  /// Enum type prefix, default value is "T"
  pub prefix_of_enum: Option<String>,

  /// When parse schema error, this message will be returned
  pub parse_error_message: Option<String>,

  /// Display comments at the top of the code
  pub explain: Option<String>,

  /// Whether to automatically generate comments, default value is false
  pub is_gen_comment: Option<bool>,

  /// Default value is 2
  pub indent: Option<i8>,

  /// Enable semicolon, default value is true
  pub semi: Option<bool>,

  /// If this is enabled, it will generate the optional interface, default value is true
  pub optional: Option<bool>,

  /// If you don't want to generate the type of an attribute in a root object,
  /// you can pass in the key name of the corresponding attribute.
  /// Like this, ignore_keys: ["firstName", "lastName"]
  /// Schema2ts will ignore the two attributes and doesn't generate the type of them.
  pub ignore_keys: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSchema {
  pub title: Option<String>,
  #[serde(rename(deserialize = "type"))]
  pub json_type: Option<String>,
  pub properties: Option<IndexMap<String, JsonSchema>>,
  pub items: Option<Box<JsonSchema>>,
  #[serde(rename(deserialize = "enum"))]
  pub enum_vals: Option<EnumTypes>,
  pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EnumTypes {
  EnumType(Vec<EnumType>),
  StringEnum(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumType {
  pub title: Option<String>,
  pub value: Option<String>,
}
