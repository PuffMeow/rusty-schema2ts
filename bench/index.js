const { Bench } = require("tinybench");
const { schema2ts } = require("@puffmeow/schema2ts");
const { schema2ts: rustySchema2Ts } = require("../index");
const path = require("path");
const fs = require("fs");
const { promisify } = require("util");

const promisifyReadFile = promisify(fs.readFile);

promisifyReadFile(path.resolve(__dirname, "./jsonSchema.json"), "utf8").then(
  async (testJsonSchema) => {
    const bench = new Bench();

    bench
      .add("TypeScript: schema2ts", () => {
        schema2ts(testJsonSchema);
      })
      .add("Rust: rustySchema2ts", () => {
        rustySchema2Ts(testJsonSchema);
      });

    await bench.run();
    console.table(bench.table());
  }
);
