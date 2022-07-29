const fs = require('fs');
const json2ts = require('json-schema-to-typescript');
const path = require('path');

const schemasDir = path.join('rust', 'json-gen', 'schemas');
const schemaFiles = fs.readdirSync(schemasDir).filter(file => path.extname(file) === '.json');

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

Promise.all(schemaFiles.map(schemaFile => {
  const completeName = path.join(schemasDir, schemaFile);
  //console.log(`Processing: ${completeName}`);
  const originalFile = fs.readFileSync(completeName, 'utf8');
  let schemaObj = JSON.parse(originalFile);
  //console.log(`ORIGINAL: ${JSON.stringify(schemaObj)}\n`);

  // this gets rid of [k: string]: unknown in generated .ts
  // but we shouldn't do this if it already exists in the case
  // of map types
  if (typeof schemaObj.additionalProperties !== 'object') {
    schemaObj.additionalProperties = false;
  }

  // NOTE: Due to infinite recursion in recursive types (e.g. NativeScript)
  // possibly due to a bug in the schema->ts lib, we instead just remove the
  // duplicates after the fact. Self-referencing is confirmed not supported,
  // but there has been mentions in those github issues of support for other
  // recursive types correctly.
  //
  // It's possibly related to directly using "Foo.json" in replaceRefs()
  // but I was not able to get this working using the $ref external file: syntax
  // that seemed to be valid online.
  // We can also take advantage of the post-removal to easily remove 2 weird unused
  // outputs that shouldn't have been generated to begin with (String and MapOf_*)

  /*
  // we need to make all references by external so we don't duplicate declarations
  if (schemaObj.definitions != null) {
    // eliminate in-file definitions
    schemaObj.definitions = [];
    // change all refs from local to external
    replaceRefs(schemaObj);
  }
  */

  //console.log(`NEW: ${JSON.stringify(schemaObj)}\n\n\n\n\n`);
  return json2ts.compile(schemaObj, schemaFile, {
    declareExternallyReferenced: false,
    cwd: schemasDir,//path.join(process.cwd(), schemasDir),
    bannerComment: ''
  }).catch(e => { console.error(`${schemaFile}: ${e}`); });
  
})).then(tsDefs => {
  fs.mkdirSync(path.join('rust', 'json-gen', 'output'), { recursive: true });
  const defs = tsDefs.join('').split(/\r?\n/);
  let dedupedDefs = [];
  let start = null;
  let added = new Set();
  const addDef = (cur) => {
    if (start != null) {
      let defName = defs[start].match(/export\s+(type|interface)\s+(\w+).*/);
      let defKey = null;
      if (defName != null && defName.length > 2) {
        defKey = defName[2];
      } else {
        console.error(`run-json2ts.js could not find name for de-dup(${defName != null}): "${defs[start]}"`);
      }
      if (defKey == null || !added.has(defKey)) {
        for (let j = start; j < cur; ++j) {
          dedupedDefs.push(defs[j]);
        }
        if (defKey != null) {
          added.add(defKey);
        }
      }
    }
    start = cur;
  };
  for (let i = 0; i < defs.length; ++i) {
    if (defs[i].startsWith('export')) {
      addDef(i);
    }
  }
  addDef(defs.length);
  // prepend 'JSON' to all identifiers here so they don't conflict with main .ts types
  for (let i = 0; i < dedupedDefs.length; ++i) {
    for (let id of added) {
      dedupedDefs[i] = dedupedDefs[i].replace(new RegExp(`\\b${id}\\b`), id + 'JSON');
    }
  }
  return fs.writeFileSync(path.join('rust', 'json-gen', 'output', 'json-types.d.ts'), dedupedDefs.join('\n'));
});

