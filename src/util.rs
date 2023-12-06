use std::any::type_name;

use crate::structure::{EnumTypes, JsonSchema};
use anyhow::Context;
use napi::Status;
use regex::{self, Regex};
use serde::de::DeserializeOwned;

impl<T> MapErr<T> for Result<T, anyhow::Error> {}

pub trait MapErr<T>: Into<Result<T, anyhow::Error>> {
  fn convert_err(self) -> napi::Result<T> {
    self
      .into()
      .map_err(|err| napi::Error::new(Status::GenericFailure, format!("{:?}", err)))
  }
}

// Make the first letter uppercase
#[inline]
pub fn capitalize(s: &str) -> String {
  let mut chars = s.chars();
  match chars.next() {
    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    None => "".to_string(),
  }
}

// Handle code indent
pub fn get_indent(indent: i8) -> String {
  " ".repeat(indent as usize)
}

pub fn parse_json(schema: &str) -> Option<JsonSchema> {
  match serde_json::from_str(schema) {
    Ok(parsed) => Some(parsed),
    Err(e) => {
      eprintln!("{}", e);
      None
    }
  }
}

#[inline]
pub fn generate_comment(schema: &JsonSchema, indent: i8) -> String {
  let mut comment = String::new();

  if let Some(title) = &schema.title {
    comment.push_str(title);
  }

  if let Some(items) = &schema.items {
    if let Some(title) = items.title.as_deref() {
      comment.push_str(format!(" {}", title).as_str());
    }
  }

  if let Some(description) = &schema.description {
    if !comment.is_empty() {
      comment.push_str(" ");
    }
    comment.push_str(&format!("({})", description));
  }

  if !comment.is_empty() {
    format!("{}/** {} */\n", get_indent(indent), comment)
  } else {
    "".to_string()
  }
}

/// Generate enum type
pub fn generate_enum_variants(enum_vals: &EnumTypes) -> String {
  let mut result = String::new();

  match enum_vals {
    EnumTypes::EnumType(enum_vals) => {
      let len = enum_vals.len();

      for i in 0..len {
        let enum_val = enum_vals.get(i).unwrap();
        if enum_val.value.as_ref().is_none() {
          continue;
        }

        if i == len - 1 {
          let has_next_one = enum_vals.get(i + 1);
          if let Some(v) = has_next_one {
            if v.value.is_some() {
              let val = enum_val.value.as_ref().unwrap();
              result.push_str(format!("'{}' | ", val.trim()).as_str());
            }
          } else {
            let val = enum_val.value.as_ref().unwrap();
            result.push_str(format!("'{}'", val.trim()).as_str());
          }
        } else {
          let val = enum_val.value.as_ref().unwrap();
          result.push_str(format!("'{}' | ", val.trim()).as_str());
        }
      }
    }
    EnumTypes::StringEnum(enum_vals) => {
      let len = enum_vals.len();
      for (i, e) in enum_vals.iter().enumerate() {
        let trimmed_value = e.trim();
        if !trimmed_value.is_empty() {
          if i == len - 1 {
            result.push_str(&format!("'{}'", trimmed_value));
          } else {
            result.push_str(&format!("'{}' | ", trimmed_value));
          }
        }
      }
    }
  }

  result
}

pub fn remove_comment(interface_str: &str) -> String {
  let re = Regex::new(r"\s*\/\*\*(.*?)\*\/\s*").unwrap();
  interface_str
    .lines()
    .map(|line| re.replace(line, "").to_string())
    .filter(|v| !v.is_empty())
    .collect::<Vec<String>>()
    .join("\n")
}

pub fn check_is_valid_title(s: Option<&str>) -> bool {
  if let Some(str) = s {
    !str.is_empty()
      && str
        .chars()
        .any(|c| c.is_alphabetic() || c == '_' || c == '$')
  } else {
    false
  }
}

pub fn deserialize_json<T>(json: &str) -> Result<T, serde_json::Error>
where
  T: DeserializeOwned,
{
  let mut deserializer = serde_json::Deserializer::from_str(json);

  T::deserialize(&mut deserializer)
}

pub fn get_deserialized<T, B>(buffer: B) -> napi::Result<T>
where
  T: DeserializeOwned,
  B: AsRef<[u8]>,
{
  let mut deserializer = serde_json::Deserializer::from_slice(buffer.as_ref());

  let v = T::deserialize(&mut deserializer)
    .with_context(|| {
      format!(
        "Failed to deserialize buffer as {}\nJSON: {}",
        type_name::<T>(),
        String::from_utf8_lossy(buffer.as_ref())
      )
    })
    .convert_err()?;

  Ok(v)
}
