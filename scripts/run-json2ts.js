const fs = require('fs');
const json2ts = require('json-schema-to-typescript');
const path = require('path');

const schemasDir = path.join('rust', 'json-gen', 'schemas');
const schemaFiles = ['NativeScript.json'];//fs.readdirSync(schemasDir).filter(file => path.extname(file) === '.json');

function replaceRef(obj) {
  if (obj['$ref'] != null && typeof obj['$ref'] === 'string' && obj['$ref'].startsWith('#/definitions/')) {
    obj['$ref'] = obj['$ref'].replace(/^(#\/definitions\/)/, '') + '.json';//`file:`) + '.json#';
    console.log(`replacing: ${obj['$ref']}`);
  }
}

function replaceRefs(node) {
  Object.entries(node).forEach(([k, v]) => {
    if (typeof v === 'object') {
      replaceRef(v);
      replaceRefs(v);
      /*if (v.additionalProperties != null) {
        replaceRef(v.additionalProperties);
      }*/
    }
  });
}

console.log(`cwd = ${process.cwd()}`);

Promise.all(schemaFiles.map(schemaFile => {
  const completeName = path.join(schemasDir, schemaFile);
  console.log(`Processing: ${completeName}`);
  const originalFile = fs.readFileSync(completeName, 'utf8');
  let schemaObj = JSON.parse(originalFile);
  //console.log(`ORIGINAL: ${JSON.stringify(schemaObj)}\n`);
  // this gets rid of [k: string]: unknown in generated .ts
  schemaObj.additionalProperties = false;
  // we need to make all references by external so we don't duplicate declarations
  if (schemaObj.definitions != null) {
    // eliminate in-file definitions
    schemaObj.definitions = [];
    // change all refs from local to external
    /*if (typeof schemaObj.properties === 'object') {
      replaceRefs(schemaObj.properties);
    }
    if (typeof schemaObj.items === 'object') {
      replaceRef(schemaObj.items);
    }*/
    replaceRefs(schemaObj);
  }
  console.log(`NEW: ${JSON.stringify(schemaObj)}\n\n\n\n\n`);
  //fs.writeFileSync(completeName, JSON.stringify(schemaObj));
  return json2ts.compile(schemaObj, schemaFile, {
  //return json2ts.compileFromFile(completeName, {
    declareExternallyReferenced: false,
    cwd: schemasDir,//path.join(process.cwd(), schemasDir),
    bannerComment: ''
  }).catch(e => { console.error(`${schemaFile}: ${e}`); });
  
})).then(tsDefs => {
  fs.mkdirSync(path.join('rust', 'json-gen', 'output'), { recursive: true });
  return fs.writeFileSync(path.join('rust', 'json-gen', 'output', 'json-types.d.ts'), tsDefs.join(''));
});

