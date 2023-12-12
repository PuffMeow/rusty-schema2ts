const { schema2ts: _schema2ts } = require('./binding');

function toBuffer(t) {
  return Buffer.from(JSON.stringify(t));
}

function schema2ts(schema, options) {
  return _schema2ts(schema, toBuffer(options || {}));
}

module.exports.schema2ts = schema2ts;
