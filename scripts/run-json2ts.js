const fs = require('fs');
const json2ts = require('json-schema-to-typescript');
const path = require('path');

const schemasDir = path.join('rust', 'json-gen', 'schemas');
const schemaFiles = fs.readdirSync(schemasDir).filter(file => path.extname(file) === '.json');



Promise.all(schemaFiles.map(schemaFile => {
  const completeName = path.join(schemasDir, schemaFile);
  console.log(`Processing: ${completeName}`);
  const originalFile = fs.readFileSync(completeName, 'utf8');
  let schemaObj = JSON.parse(originalFile);
  // this gets rid of [k: string]: unknown in generated .ts
  schemaObj.additionalProperties = false;
  if (schemaObj.definitions != null) {
    // we can't remove the definitions or else they won't resolve in json2ts
    //schemaObj.definitions = [];
  }
  fs.writeFileSync(completeName, JSON.stringify(schemaObj));
  return json2ts.compileFromFile(completeName, {declareExternallyReferenced: false, unreachableDefinitions: true});
})).then(tsDefs => {
  //console.log(`\n\n\n\nTSDEFS:${tsDefs}`);
  fs.mkdirSync(path.join('rust', 'json-gen', 'output'), { recursive: true });
  tsDefs = tsDefs.map(def => {
    // this gets rid of the auto-generated comment so it doesn't get repeated a million times
    return def.split('\n').slice(6).join('\n');
  });
  return fs.writeFileSync(path.join('rust', 'json-gen', 'output', 'json-types.d.ts'), tsDefs.join(''));
});

