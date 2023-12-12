const { Bench } = require('tinybench');
const { schema2ts } = require('@puffmeow/schema2ts');
const { schema2ts: rustySchema2Ts } = require('../index');
const path = require('path');
const fs = require('fs');
const { promisify } = require('util');
const { compile } = require('json-schema-to-typescript');

const promisifyReadFile = promisify(fs.readFile);

promisifyReadFile(path.resolve(__dirname, './jsonSchema.json'), 'utf8').then(
  async (testJsonSchema) => {
    const parsed = JSON.parse(testJsonSchema);

    const bench = new Bench();

    bench
      .add('Rust: rustySchema2ts', () => {
        rustySchema2Ts(testJsonSchema);
      })
      .add('TypeScript: schema2ts', () => {
        schema2ts(testJsonSchema);
      })
      .add('Other: json-schema-to-typescript', () => {
        compile(parsed);
      });

    await bench.run();
    console.table(bench.table());
  }
);
