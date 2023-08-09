## Introduce

This is a tool that can help you transform your JSON Schema to TypeScript interface quickly🧲.

It's written by Rust(napi-rs) and only support in Node.js(Support for all platforms except Android).

If you want the same feature that can work in Browser or all other platforms, you can see [schema2ts](https://github.com/PuffMeow/schema2ts).

Git repository: [rusty-schema2ts](https://github.com/PuffMeow/rusty-schema2ts). If you like it, please give me a little star♥

__The api of them are all the same.__

## TypeScript vs Rust

You can find [benchmark here](https://github.com/PuffMeow/rusty-schema2ts/blob/main/bench/index.js)

| index | Task Name             | ops/sec | Average Time (ns)  | Margin | Samples |
| ----- | --------------------- | ------- | ------------------ | ------ | ------- |
| 0     | TypeScript: schema2ts | 2,796   | 357534.31021794415 | ±1.08% | 1399    |
| 1     | Rust: rustySchema2ts  | 5,431   | 184122.05448994122 | ±0.29% | 2716    |

## Support matrix

|                              | node12 | node14 | node16 | node18 |
| ---------------------------- | ------ | ------ | ------ | ------ |
| Windows x64                  | ✓      | ✓      | ✓      | ✓      |
| Windows x32                  | ✓      | ✓      | ✓      | ✓      |
| Windows arm64                | ✓      | ✓      | ✓      | ✓      |
| macOS x64                    | ✓      | ✓      | ✓      | ✓      |
| macOS arm64 (m chips)        | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu (glibc 2.17)   | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl               | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu (glibc 2.17)   | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu (glibc 2.17) | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl             | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64                  | ✓      | ✓      | ✓      | ✓      |

## Install

npm

```
npm i @puffmeow/rusty-schema2ts
```

pnpm

```
pnpm i @puffmeow/rusty-schema2ts
```

yarn

```
yarn add @puffmeow/rusty-schema2ts
```

## Quick start

It's really easy to use.

```ts
import { schema2ts } from "@puffmeow/rusty-schema2ts";

// The "options" we will introduce later
// schema2ts(schema: string, options?: IOptions): string
schema2ts(`your schema`, options);
```

If you have a schema like this:

### Input schema

```json
{
  "title": "Schema",
  "type": "object",
  "properties": {
    "firstName": {
      "type": "string"
    },
    "lastName": {
      "type": "string"
    },
    "age": {
      "type": "number"
    },
    "hairColor": {
      "enum": [
        {
          "title": "hair color1",
          "value": "color1"
        },
        {
          "title": "hair color2",
          "value": "color2"
        },
        {
          "title": "hair color3",
          "value": "color3"
        }
      ],
      "type": "string"
    },
    "obj": {
      "type": "object",
      "properties": {
        "key1": {
          "type": "string"
        },
        "key2": {
          "type": "number"
        },
        "key3": {
          "type": "boolean"
        }
      }
    },
    "arr": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "arr1": {
            "type": "string"
          },
          "arr2": {
            "type": "number"
          },
          "arr3": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "enen1": {
                  "type": "string"
                },
                "enen2": {
                  "type": "number"
                },
                "enen3": {
                  "type": "boolean"
                },
                "enen4": {
                  "type": "unknow type will transform to any"
                }
              }
            }
          }
        }
      }
    }
  }
}
```

### Output TypeScript interface

Finally it will output like this:

```ts
export type THairColor = "color1" | "color2" | "color3";

export interface ISchema {
  firstName?: string;
  lastName?: string;
  age?: number;
  hairColor?: THairColor;
  obj?: IObj;
  arr?: IArr[];
}

export interface IObj {
  key1?: string;
  key2?: number;
  key3?: boolean;
}

export interface IArr {
  arr1?: string;
  arr2?: number;
  arr3?: IArr3[];
}

export interface IArr3 {
  enen1?: string;
  enen2?: number;
  enen3?: boolean;
  enen4?: any;
}
```

## Options

| key               | type     | required | default                                          | description                                                  |
| ----------------- | -------- | -------- | ------------------------------------------------ | ------------------------------------------------------------ |
| preffix           | string   | ×        | I                                                | Interface preffix, if you don't like this, you can give it a empty string |
| preffixOfEnum     | string   | ×        | T                                                | Enum type preffix, if you don't like this, you can give it a empty string |
| isGenComment      | boolean  | ×        | false                                            | Whether to automatically generate comments                   |
| isExport          | boolean  | ×        | true                                             | Whether to export the interfaces and types                   |
| indent            | number   | ×        | 2                                                | Code indent                                                  |
| semi              | boolean  | ×        | true                                             | Is enable semicolon                                          |
| optional          | boolean  | ×        | true                                             | If this is enabled, it will generate the optional interface, default value is true |
| ignoreKeys        | string[] | ×        | []                                               | If you don't want to generate the type of an attribute in a root object, you can pass in the key name of the corresponding attribute.<br /><br />Like this, ignoreKeys: ["firstName", "lastName"]<br /><br />Schema2ts will ignore the two attributes and doesn't generate the type of them. |
| explain           | string   | ×        |                                                  | Display some comments at the top of the code                 |
| parseErrorMessage | string   | ×        | // Parse schema error, please check your schema. | When parsing schema error, this message will be return       |

## More examples

### 1.generate comment

```ts
schema2ts(`below json`, { isGenComment: true });
```

#### input json

```json
{
  "title": "Schema",
  "type": "object",
  "properties": {
    "firstName": {
      "title": "This is the first name",
      "type": "string"
    },
    "lastName": {
      "title": "This is the last name",
      "type": "string"
    },
    "age": {
      "title": "This is the age",
      "type": "number"
    },
    "hairColor": {
      "title": "This is the hair color",
      "enum": [
        {
          "title": "hair color1",
          "value": "color1"
        },
        {
          "title": "hair color2",
          "value": "color2"
        },
        {
          "title": "hair color3",
          "value": "color3"
        }
      ],
      "type": "string"
    },
    "obj": {
      "type": "object",
      "title": "Object test",
      "properties": {
        "key1": {
          "title": "This is the key1",
          "type": "string"
        },
        "key2": {
          "title": "This is the key2",
          "type": "number"
        },
        "key3": {
          "title": "This is the key3",
          "type": "boolean"
        }
      }
    },
    "arr": {
      "type": "array",
      "title": "Arr test",
      "items": {
        "type": "object",
        "title": "Nested array items",
        "properties": {
          "arr1": {
            "title": "This is the arr1",
            "type": "string"
          },
          "arr2": {
            "title": "This is the arr2",
            "type": "number"
          },
          "arr3": {
            "type": "array",
            "title": "Test arr3",
            "items": {
              "type": "object",
              "title": "Test nested arr3 items",
              "properties": {
                "enen1": {
                  "title": "This is the enen1",
                  "type": "string"
                },
                "enen2": {
                  "title": "This is the enen2",
                  "type": "number"
                },
                "enen3": {
                  "title": "This is the enen3",
                  "type": "boolean"
                }
              }
            }
          }
        }
      }
    }
  }
}
```

#### output

```ts
export type THairColor = "color1" | "color2" | "color3";

/** Schema */
export interface ISchema {
  /** This is the first name */
  firstName?: string;
  /** This is the last name */
  lastName?: string;
  /** This is the age */
  age?: number;
  /** This is the hair color */
  hairColor?: THairColor;
  /** Object test */
  obj?: IObj;
  /** Arr test Nested array items */
  arr?: IArr[];
}

/** Object test */
export interface IObj {
  /** This is the key1 */
  key1?: string;
  /** This is the key2 */
  key2?: number;
  /** This is the key3 */
  key3?: boolean;
}

/** Nested array items */
export interface IArr {
  /** This is the arr1 */
  arr1?: string;
  /** This is the arr2 */
  arr2?: number;
  /** Test arr3 Test nested arr3 items */
  arr3?: IArr3[];
}

/** Test nested arr3 items */
export interface IArr3 {
  /** This is the enen1 */
  enen1?: string;
  /** This is the enen2 */
  enen2?: number;
  /** This is the enen3 */
  enen3?: boolean;
}
```

### 2.ignoreKeys

```ts
schema2ts(`below json`, {
  ignoreKeys: ["firstName", "obj", "hairColor", "arr"],
  isGenComment: true,
  optional: false,
});
```

#### input json

```json
{
  "title": "Test",
  "type": "object",
  "properties": {
    "firstName": {
      "title": "This is the first name",
      "type": "string"
    },
    "lastName": {
      "title": "This is the last name",
      "type": "string"
    },
    "age": {
      "title": "This is the age",
      "type": "number"
    },
    "hairColor": {
      "title": "This is the hair color",
      "enum": [
        {
          "title": "hair color1",
          "value": "color1"
        },
        {
          "title": "hair color2",
          "value": "color2"
        },
        {
          "title": "hair color3",
          "value": "color3"
        }
      ],
      "type": "string"
    },
    "obj": {
      "type": "object",
      "title": "Object test",
      "properties": {
        "key1": {
          "title": "This is the key1",
          "type": "string"
        },
        "key2": {
          "title": "This is the key2",
          "type": "number"
        },
        "key3": {
          "title": "This is the key3",
          "type": "boolean"
        }
      }
    },
    "arr": {
      "type": "array",
      "title": "Arr test",
      "items": {
        "type": "object",
        "title": "Nested array items",
        "properties": {
          "arr1": {
            "title": "This is the arr1",
            "type": "string"
          },
          "arr2": {
            "title": "This is the arr2",
            "type": "number"
          },
          "arr3": {
            "type": "array",
            "title": "Test arr3",
            "items": {
              "type": "object",
              "title": "Test nested arr3 items",
              "properties": {
                "enen1": {
                  "title": "This is the enen1",
                  "type": "string"
                },
                "enen2": {
                  "title": "This is the enen2",
                  "type": "number"
                },
                "enen3": {
                  "title": "This is the enen3",
                  "type": "boolean"
                }
              }
            }
          }
        }
      }
    }
  }
}
```

#### output

```ts
/** Test */
export interface ITest {
  /** This is the last name */
  lastName: string;
  /** This is the age */
  age: number;
}
```
