const fs = require('fs');

const inputFile = fs.readFileSync('./rust/pkg/cardano_serialization_lib_bg.js', 'utf8').split(/\r?\n/);

const regex = /(\s*if \(cached[A-Za-z0-9]+Memory[0-9]* === null ||) (cached[A-Za-z0-9]+Memory[0-9]*)\.byteLength === 0\) {/;
const replacer = '$1 $2.buffer !== wasm.memory.buffer) {';

for (let i = 0; i < inputFile.length; ++i) {
    let line = inputFile[i];
    inputFile[i] = line.replace(regex, replacer);
}

fs.writeFile(
    './rust/pkg/cardano_serialization_lib_bg.js',
    inputFile.join('\n'),
    (err) => {
        if (err != null) {
            console.log(`err writing file: ${err}`)
        }
    }
);
