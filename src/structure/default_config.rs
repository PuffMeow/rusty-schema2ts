use super::Config;

pub const DEFAULT_EXPLAIN: &str = "";
pub const DEFAULT_PREFFIX: &str = "I";
pub const DEFAULT_ENUM_PREFFIX: &str = "T";
pub const DEFAULT_PARSE_ERROR_MESSAGE: &str = "// Parse schema error, please check your schema.";
pub const DEFAULT_INDENT: i8 = 2;
pub const DEFAULT_SEMI: bool = true;
pub const DEFAULT_OPTIONAL: bool = true;
pub const DEFAULT_GEN_COMMENT: bool = false;
pub const DEFAULT_IGNORE_KEYS: Vec<String> = vec![];

impl Default for Config {
  fn default() -> Self {
    Config {
      parse_error_message: Some("// Parse schema error, please check your schema.".to_string()),
      optional: Some(true),
      explain: Some(String::new()),
      is_gen_comment: Some(false),
      indent: Some(2),
      semi: Some(true),
      ignore_keys: Some(vec![]),
      prefix: Some("I".to_string()),
      prefix_of_enum: Some("T".to_string()),
    }
  }
}
