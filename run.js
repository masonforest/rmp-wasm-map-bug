const fs = require("fs");

async function run() {
  file = fs.readFileSync("wasm-map-bug.wasm");
  var {instance} = await WebAssembly.instantiate(file, {env: {}});
  var outputPtr = instance.exports.pointer_to_five();
  var output = new Uint32Array(instance.exports.memory.buffer, outputPtr, 1);
  console.log("Pointer to five points to: " + output[0]);
  var outputPtr = instance.exports.pointer_to_six();
  var output = new Uint32Array(instance.exports.memory.buffer, outputPtr, 1);
  console.log("Pointer to six points to: " + output[0]);
}

run();
