const fs = require('fs');
const oldPkg = require('../publish/package.json');

const flowFile = 'cardano_serialization_lib.js.flow';
if (oldPkg.files.find(entry => entry === flowFile) == null) {
  oldPkg.files.push('cardano_serialization_lib.js.flow');
}
if (oldPkg.name === 'cardano-serialization-lib') {
  oldPkg.name = '@emurgo/' + oldPkg.name + process.argv.slice(2)[0];
}
console.log(oldPkg);
fs.writeFileSync('./publish/package.json', JSON.stringify(oldPkg, null, 2));
