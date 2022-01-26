const fs = require('fs');
const json2ts = require('json-schema-to-typescript');
const path = require('path');

const schemasDir = path.join('rust', 'json-gen', 'schemas');
const schemaFiles = fs.readdirSync(schemasDir).filter(file => path.extname(file) === '.json');

Promise.all(schemaFiles.map(schemaFile => {
  const completeName = path.join(schemasDir, schemaFile);
  console.log(`Processing: ${completeName}`);
  return json2ts.compileFromFile(completeName, {declareExternallyReferenced: false});
})).then(tsDefs => fs.writeFileSync('foo.d.ts', tsDefs.join()));

