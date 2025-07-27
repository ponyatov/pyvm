import * as config from '/config.js';

/// 64K shared memory
const memory = new WebAssembly.Memory({initial: 1});

const importObject = {
    console: {
        log: (addr) => {
            const mem = new Uint8Array(memory.buffer);
        }
    },
    js: {mem: memory}
};

WebAssembly.instantiateStreaming(fetch('/hello.wasm'), importObject);
// .then(obj => {
//     obj.instance.exports.hello();
// })
// .catch(console.error);

$(() => {
    console.log(memory);
    $('#screen_').css({width: config.width, height: config.height})
});
