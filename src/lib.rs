#![deny(clippy::all)]
use napi;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use structure::Config;
use structure::JsonSchema;
use util::{
  capitalize, check_is_valid_title, generate_comment, get_indent, parse_json, remove_comment,
};

use crate::structure::default_config::{
  DEFAULT_ENUM_PREFFIX, DEFAULT_EXPLAIN, DEFAULT_GEN_COMMENT, DEFAULT_IGNORE_KEYS, DEFAULT_INDENT,
  DEFAULT_OPTIONAL, DEFAULT_PARSE_ERROR_MESSAGE, DEFAULT_PREFFIX, DEFAULT_SEMI,
};
use crate::util::generate_enum_variants;
#[macro_use]
extern crate napi_derive;
mod structure;
mod util;

#[napi(ts_args_type = "schema: string, options?: IOptions")]
pub fn rusty_schema_to_ts(schema: String, options: Option<Config>) -> String {
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
  let mut cache_type_name: HashSet<String> = HashSet::new();
  let type_key_map: Rc<RefCell<HashMap<String, i32>>> = Rc::new(RefCell::new(HashMap::new()));

  fn get_type(prop: Option<&JsonSchema>, key: &str, opts: &Config) -> String {
    let capitalized_key = capitalize(key);

    match prop.map(|p| p.json_type.as_deref()).flatten() {
      Some("string") | Some("number") | Some("boolean") | Some("integer") | Some("undefined")
      | Some("null") => {
        if let Some(_) = &prop.unwrap().enum_vals {
          format!(
            "{}{}",
            opts
              .prefix_of_enum
              .clone()
              .as_deref()
              .unwrap_or(DEFAULT_ENUM_PREFFIX),
            capitalized_key
          )
        } else {
          prop
            .unwrap()
            .json_type
            .clone()
            .unwrap_or_else(|| "any".to_string())
        }
      }
      Some("object") => format!(
        "{}{}",
        opts.prefix.clone().as_deref().unwrap_or(DEFAULT_PREFFIX),
        capitalized_key
      ),
      Some("array") => format!(
        "{}{}[]",
        opts.prefix.clone().as_deref().unwrap_or(DEFAULT_PREFFIX),
        capitalized_key
      ),
      _ => "any".to_string(),
    }
  }

  fn generate_root_interface(schema: &JsonSchema, name: &str, opts: &Config) -> String {
    let mut interface_str = String::new();

    if opts.is_gen_comment.clone().unwrap_or(DEFAULT_GEN_COMMENT) {
      interface_str.push_str(&generate_comment(schema, 0));
    }

    interface_str.push_str(&format!(
      "export interface {}{} {{\n",
      &opts.prefix.clone().as_deref().unwrap_or(DEFAULT_PREFFIX),
      capitalize(name)
    ));

    if let Some(properties) = &schema.properties {
      for (key, prop) in properties {
        if let Some(ignore_keys) = &opts.ignore_keys.clone() {
          if ignore_keys.contains(&key) {
            continue;
          }
        }

        let schema_type = get_type(Some(prop), &key, &opts);
        // generate comment
        if opts.is_gen_comment.clone().unwrap_or(DEFAULT_GEN_COMMENT) {
          interface_str.push_str(&generate_comment(
            prop,
            opts.indent.clone().unwrap_or(DEFAULT_INDENT),
          ));
        }

        interface_str.push_str(&format!(
          "{}{}{}: {}{}\n",
          get_indent(opts.indent.clone().unwrap_or(DEFAULT_INDENT)),
          key,
          if opts.optional.clone().unwrap_or(DEFAULT_OPTIONAL) {
            "?"
          } else {
            ""
          },
          schema_type,
          if opts.semi.clone().unwrap_or(DEFAULT_SEMI) {
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

  fn generate_enum(schema: &JsonSchema, key: &str, suffix_num: String, opts: &Config) -> String {
    if let Some(enum_vals) = &schema.enum_vals {
      format!(
        "export type {}{}{} = {}{}\n",
        opts
          .prefix_of_enum
          .clone()
          .as_deref()
          .unwrap_or(DEFAULT_ENUM_PREFFIX),
        capitalize(key),
        suffix_num,
        generate_enum_variants(enum_vals),
        if opts.semi.clone().unwrap_or(DEFAULT_SEMI) {
          ";"
        } else {
          ""
        }
      )
    } else {
      String::new()
    }
  }

  fn generate_interface(
    schema: &JsonSchema,
    key: &str,
    interfaces: &mut VecDeque<String>,
    cache_type_name: &mut HashSet<String>,
    type_key_map: &Rc<RefCell<HashMap<String, i32>>>,
    opts: &Config,
  ) {
    let interface_str = generate_root_interface(&schema, &key, &opts);

    let plain_interface_str = if opts.is_gen_comment.clone().unwrap_or(DEFAULT_GEN_COMMENT) {
      remove_comment(&interface_str)
    } else {
      interface_str.clone()
    };

    if !cache_type_name.contains(&plain_interface_str) {
      cache_type_name.insert(plain_interface_str);
      interfaces.push_back(interface_str);
    }

    if let Some(properties) = &schema.properties {
      for (key, prop) in properties {
        if opts
          .ignore_keys
          .clone()
          .as_deref()
          .unwrap_or(&DEFAULT_IGNORE_KEYS)
          .contains(&key)
        {
          continue;
        }

        if let Some(_) = &prop.enum_vals {
          let mut enum_type = generate_enum(prop, key, String::from(""), &opts);

          if type_key_map.borrow().contains_key(key) {
            let mut type_key_map = type_key_map.borrow_mut();
            let key_num = type_key_map.get(key).unwrap_or(&1);
            let key_num = *key_num + 1;
            type_key_map.insert(key.clone(), key_num);
          } else {
            type_key_map.borrow_mut().insert(key.clone(), 1);
          }

          if !cache_type_name.contains(&enum_type) {
            let type_key_map = type_key_map.borrow();
            let num = type_key_map.get(key).unwrap_or(&1);
            if *num > 1 {
              enum_type = generate_enum(prop, key, (*num).to_string(), &opts);
            }
            cache_type_name.insert(enum_type.clone());
            interfaces.push_front(enum_type);
          }
        }

        if let Some(_) = &prop.properties {
          generate_interface(
            prop,
            &capitalize(key),
            interfaces,
            cache_type_name,
            type_key_map,
            opts,
          );
        }

        if let Some(item_properties) = &prop.items {
          generate_interface(
            item_properties,
            &capitalize(key),
            interfaces,
            cache_type_name,
            &type_key_map,
            opts,
          );
        }
      }
    }
  }

  let key = if check_is_valid_title(json_schema.title.as_deref()) {
    json_schema.title.as_deref().unwrap_or("Schema")
  } else {
    "Schema"
  };

  generate_interface(
    &json_schema,
    key,
    &mut interfaces,
    &mut cache_type_name,
    &type_key_map,
    &opts,
  );

  if !opts
    .explain
    .clone()
    .as_deref()
    .unwrap_or(DEFAULT_EXPLAIN)
    .is_empty()
  {
    interfaces.push_front(opts.explain.clone().unwrap_or(DEFAULT_EXPLAIN.to_string()));
  }

  let mut output = Vec::from(interfaces).join("\n");

  if !opts.semi.clone().unwrap_or(DEFAULT_SEMI) {
    // remove all semicolons
    output = output.replace(';', "");
  }

  output
}
