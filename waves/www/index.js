// NOTE TO DEVELOPER: If this import isn't found, chance are your build didn't output to the correct directory.
// Either add the flag `--out-dir waves/www/wasm` to wasm-pack, or manually copy the wasm and js files to /www/wasm/
import init from './wasm/waves.js'

init().then(wasm => {
    window.WASM = wasm;
});
