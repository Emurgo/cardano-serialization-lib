const fs = require('fs');
const json2ts = require('json-schema-to-typescript');
const path = require('path');

const schemasDir = path.join('rust', 'json-gen', 'schemas');
const schemaFiles = fs.readdirSync(schemasDir).filter(file => path.extname(file) === '.json');

function renameSchema(obj, visited) {
  if (visited != null && visited.has(obj)) {
    return;
  }
  visited = visited || new Set();
  visited.add(obj);
  if (obj.hasOwnProperty("title")) {
    if (!obj.title.endsWith("JSON")) {
      obj.title = obj.title + "JSON";
    }
  }
  renameDefinitions(obj, visited);
  renameRef(obj);
  goOverProperties(obj, visited);
  goOverMultiType(obj, visited);
}

function renameRef(obj) {
    if (obj.hasOwnProperty("$ref")) {
        const ref = obj["$ref"];
        if (!ref.endsWith("JSON")) {
          obj["$ref"] = ref + "JSON";
        }
    }
}

function renameDefinitions(obj, visited) {
  if (obj.hasOwnProperty("definitions")) {
    for (const [key, value] of Object.entries(obj.definitions)) {
      if (!key.endsWith("JSON") && !key.startsWith("__")) {
        renameObjectKey(obj.definitions, key, key + "JSON");
        renameSchema(value, visited)
      }
      visited.add(value);
    }
  }
}

function goOverProperties(obj, visited) {
  if (obj.hasOwnProperty("properties")) {
    for (const [key, value] of Object.entries(obj.properties)) {
      renameSchema(value, visited);
    }
  }
  if (obj.hasOwnProperty("items")) {
    if (Array.isArray(obj.items)) {
      for (const [key, value] of Object.entries(obj.items)) {
        renameSchema(value, visited);
      }
    } else if (typeof obj.items === "object") {
      renameSchema(obj.items, visited);
    }
  }

  if (obj.hasOwnProperty("additionalProperties")) {
    if (Array.isArray(obj.additionalProperties)) {
      for (const [key, value] of Object.entries(obj.additionalProperties)) {
        renameSchema(value, visited);
      }
    } else if (typeof obj.additionalProperties === "object") {
      renameSchema(obj.additionalProperties, visited);
    }

  }
}

function isMultiType(obj) {
    return obj.hasOwnProperty("anyOf") || obj.hasOwnProperty("oneOf") || obj.hasOwnProperty("allOf");
}

function goOverMultiType(obj, visited) {
  if (isMultiType(obj)) {
    for (const [key, value] of Object.entries(obj)) {
      if (key === "anyOf" || key === "oneOf" || key === "allOf") {
        for (const [index, type] of Object.entries(value)) {
          renameSchema(type, visited);
        }
      }
    }
  }
}

function renameObjectKey(obj, oldKey, newKey) {
  if (obj.hasOwnProperty(oldKey)) {
    delete Object.assign(obj, {[newKey]: obj[oldKey]})[oldKey];
  }
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

  renameSchema(schemaObj, null);
  // console.log(`NEW: ${JSON.stringify(schemaObj)}\n\n\n\n\n`);
  return json2ts.compile(schemaObj, schemaFile, {
    declareExternallyReferenced: true,
    additionalProperties: false,
    cwd: schemasDir,//path.join(process.cwd(), schemasDir),
    bannerComment: ''
  }).catch(e => { console.error(`${schemaFile}: ${e}`); });
  
})).then(tsDefs => {
  fs.mkdirSync(path.join('rust', 'json-gen', 'output'), { recursive: true });
  const defs = tsDefs.join('').split(/\r?\n/);
  //replace all auto-deduped defs with the first one
  for(let i = 0; i < defs.length; ++i) {
    defs[i] = defs[i].replace(/JSON\d+/, 'JSON');
  }
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
  return fs.writeFileSync(path.join('rust', 'json-gen', 'output', 'json-types.d.ts'), dedupedDefs.join('\n'));
});

