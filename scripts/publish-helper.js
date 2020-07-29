const fs = require('fs');
const oldPkg = require('../publish/package.json');

const flowFile = 'cardano_serialization_lib.js.flow';
if (oldPkg.files.find(entry => entry === flowFile) == null) {
  oldPkg.files.push(flowFile);
}
if (oldPkg.name === 'cardano-serialization-lib') {
  oldPkg.name = '@emurgo/' + oldPkg.name + process.argv.slice(2)[0];
}
if (process.argv.slice(2)[0] === '-browser') {
  // due to a bug in wasm-pack, this file is missing from browser builds
  const missingFile = 'cardano_serialization_lib_bg.js';
  if (oldPkg.files.find(entry => entry === missingFile) == null) {
    oldPkg.files.push(missingFile);
  }
}

oldPkg.repository = {
  type: "git",
  url: "git+https://github.com/Emurgo/cardano-serialization-lib.git"
};
oldPkg.author = "EMURGO";
oldPkg.license = "MIT";
console.log(oldPkg);
fs.writeFileSync('./publish/package.json', JSON.stringify(oldPkg, null, 2));
