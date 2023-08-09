#![deny(clippy::all)]
mod structure;
mod util;
use napi;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use structure::default_config::{
  DEFAULT_ENUM_PREFFIX, DEFAULT_EXPLAIN, DEFAULT_GEN_COMMENT, DEFAULT_IGNORE_KEYS, DEFAULT_INDENT,
  DEFAULT_OPTIONAL, DEFAULT_PARSE_ERROR_MESSAGE, DEFAULT_PREFFIX, DEFAULT_SEMI,
};
use structure::Config;
use structure::JsonSchema;
use util::generate_enum_variants;
use util::{
  capitalize, check_is_valid_title, generate_comment, get_indent, parse_json, remove_comment,
};
#[macro_use]
extern crate napi_derive;

#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[napi(
  js_name = "schema2ts",
  ts_args_type = "schema: string, options?: IOptions"
)]
pub fn schema_2_ts(schema: String, options: Option<Config>) -> String {
  schema_to_ts(schema.as_str(), options)
}

fn schema_to_ts(schema: &str, options: Option<Config>) -> String {
  let opts = options.unwrap_or_default();
  let json_schema: JsonSchema = match parse_json(schema) {
    Some(parsed) => parsed,
    None => {
      return opts
        .parse_error_message
        .unwrap_or(DEFAULT_PARSE_ERROR_MESSAGE.to_string())
    }
  };

  let mut interfaces: VecDeque<String> = VecDeque::new();
  let mut cache_enum_types: HashSet<String> = HashSet::new();
  let mut enum_type_key_num_map: HashMap<String, i32> = HashMap::new();

  let key = if check_is_valid_title(json_schema.title.as_deref()) {
    json_schema.title.as_deref().unwrap_or("Schema")
  } else {
    "Schema"
  };

  generate_interface(
    &json_schema,
    key,
    &mut interfaces,
    &mut cache_enum_types,
    &mut enum_type_key_num_map,
    &opts,
  );

  if !opts
    .explain
    .as_deref()
    .unwrap_or(DEFAULT_EXPLAIN)
    .is_empty()
  {
    interfaces.push_front(opts.explain.unwrap_or(DEFAULT_EXPLAIN.to_string()));
  }

  let mut output = Vec::from(interfaces).join("\n");

  if !opts.semi.unwrap_or(DEFAULT_SEMI) {
    // remove all semicolons
    output = output.replace(';', "");
  }

  output
}

fn get_type(
  prop: Option<&JsonSchema>,
  key: &str,
  cache_enum_types: &HashSet<String>,
  enum_type_key_num_map: &mut HashMap<String, i32>,
  opts: &Config,
) -> String {
  let mut capitalized_key = capitalize(key);

  match prop.map(|p| p.json_type.as_deref()).flatten() {
    Some("string") | Some("number") | Some("boolean") | Some("integer") | Some("undefined")
    | Some("null") => {
      let prop = prop.unwrap();
      if let Some(_) = prop.enum_vals {
        if enum_type_key_num_map.contains_key(key) {
          let key_num = enum_type_key_num_map.get(key).unwrap_or(&1);
          let key_num = *key_num + 1;
          enum_type_key_num_map.insert(key.to_string(), key_num);
        } else {
          enum_type_key_num_map.insert(key.to_string(), 1);
        }

        let enum_type = generate_enum(prop, key, "", opts);

        if !cache_enum_types.contains(&enum_type) {
          let num = enum_type_key_num_map.get(key).unwrap_or(&1);

          if *num > 1 {
            capitalized_key.push_str(&*num.to_string().as_str());
          }
        }

        format!(
          "{}{}",
          opts
            .prefix_of_enum
            .as_deref()
            .unwrap_or(DEFAULT_ENUM_PREFFIX),
          capitalized_key
        )
      } else {
        prop.json_type.to_owned().unwrap_or("any".to_string())
      }
    }
    Some("object") => format!(
      "{}{}",
      opts.prefix.as_deref().unwrap_or(DEFAULT_PREFFIX),
      capitalized_key
    ),
    Some("array") => format!(
      "{}{}[]",
      opts.prefix.as_deref().unwrap_or(DEFAULT_PREFFIX),
      capitalized_key
    ),
    _ => "any".to_string(),
  }
}

#[inline]
fn generate_root_interface(
  schema: &JsonSchema,
  name: &str,
  cache_enum_types: &HashSet<String>,
  enum_type_key_num_map: &mut HashMap<String, i32>,
  opts: &Config,
) -> String {
  let mut interface_str = String::new();

  if opts.is_gen_comment.unwrap_or(DEFAULT_GEN_COMMENT) {
    interface_str.push_str(&generate_comment(schema, 0));
  }

  interface_str.push_str(&format!(
    "export interface {}{} {{\n",
    &opts.prefix.as_deref().unwrap_or(DEFAULT_PREFFIX),
    capitalize(name)
  ));

  if let Some(properties) = &schema.properties {
    for (key, prop) in properties {
      if let Some(ignore_keys) = &opts.ignore_keys {
        if ignore_keys.contains(&key) {
          continue;
        }
      }

      let schema_type = get_type(
        Some(prop),
        &key,
        &cache_enum_types,
        enum_type_key_num_map,
        &opts,
      );
      // generate comment
      if opts.is_gen_comment.unwrap_or(DEFAULT_GEN_COMMENT) {
        interface_str.push_str(&generate_comment(
          prop,
          opts.indent.unwrap_or(DEFAULT_INDENT),
        ));
      }

      interface_str.push_str(&format!(
        "{}{}{}: {}{}\n",
        get_indent(opts.indent.unwrap_or(DEFAULT_INDENT)),
        key,
        if opts.optional.unwrap_or(DEFAULT_OPTIONAL) {
          "?"
        } else {
          ""
        },
        schema_type,
        if opts.semi.unwrap_or(DEFAULT_SEMI) {
          ";"
        } else {
          ""
        }
      ))
    }
  }

  interface_str.push_str("}\n");
  interface_str
}

#[inline]
fn generate_enum(schema: &JsonSchema, key: &str, suffix_num: &str, opts: &Config) -> String {
  if let Some(enum_vals) = &schema.enum_vals {
    format!(
      "export type {}{}{} = {}{}\n",
      opts
        .prefix_of_enum
        .as_deref()
        .unwrap_or(DEFAULT_ENUM_PREFFIX),
      capitalize(key),
      suffix_num,
      generate_enum_variants(enum_vals),
      if opts.semi.unwrap_or(DEFAULT_SEMI) {
        ";"
      } else {
        ""
      }
    )
  } else {
    "".to_string()
  }
}

#[inline]
fn generate_interface(
  schema: &JsonSchema,
  key: &str,
  interfaces: &mut VecDeque<String>,
  cache_enum_types: &mut HashSet<String>,
  enum_type_key_num_map: &mut HashMap<String, i32>,
  opts: &Config,
) {
  let interface_str = generate_root_interface(
    &schema,
    &key,
    &cache_enum_types,
    enum_type_key_num_map,
    &opts,
  );

  let plain_interface_str = if opts.is_gen_comment.unwrap_or(DEFAULT_GEN_COMMENT) {
    remove_comment(&interface_str)
  } else {
    interface_str.clone()
  };

  if !cache_enum_types.contains(&plain_interface_str) {
    cache_enum_types.insert(plain_interface_str);
    interfaces.push_back(interface_str);
  }

  if let Some(properties) = &schema.properties {
    for (key, prop) in properties {
      if opts
        .ignore_keys
        .as_deref()
        .unwrap_or(&DEFAULT_IGNORE_KEYS)
        .contains(&key)
      {
        continue;
      }

      if let Some(_) = &prop.enum_vals {
        let mut enum_type = generate_enum(prop, key, "", &opts);

        if !cache_enum_types.contains(&enum_type) {
          let num = enum_type_key_num_map.get(key).unwrap_or(&1);
          if *num > 1 {
            enum_type = generate_enum(prop, key, &(*num).to_string(), &opts);
          }
          cache_enum_types.insert(enum_type.clone());
          interfaces.push_front(enum_type);
        }
      }

      if let Some(_) = &prop.properties {
        generate_interface(
          prop,
          &capitalize(key),
          interfaces,
          cache_enum_types,
          enum_type_key_num_map,
          opts,
        );
      }

      if let Some(item_properties) = &prop.items {
        generate_interface(
          item_properties,
          &capitalize(key),
          interfaces,
          cache_enum_types,
          enum_type_key_num_map,
          opts,
        );
      }
    }
  }
}
