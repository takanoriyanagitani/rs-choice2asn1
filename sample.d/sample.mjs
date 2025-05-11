import { readFile, writeFile } from "node:fs/promises";

import { bind } from "./io.mjs";

/**
 * @import { IO } from "./io.mjs"
 */

/** @type string */
const wasmName = "./rs_choice2asn1.wasm";

/** @type {function(string): IO<ArrayBuffer>} */
const filename2buffer = (filename) => () => readFile(filename);

/** @type {function(ArrayBuffer): IO<WebAssembly.Module>} */
const buffer2module = (buffer) => () => WebAssembly.compile(buffer);

/** @type {function(WebAssembly.Module): IO<WebAssembly.Instance>} */
const module2instance = (module) => () => WebAssembly.instantiate(module);

/**
 * @param {WebAssembly.Exports} exp
 * @returns WebAssembly.Memory
 */
function exports2memory(exp) {
  /** @type WebAssembly.ExportValue */
  const ev = exp.memory;

  /** @type any */
  const an = ev;

  /** @type WebAssembly.Memory */
  const mm = an;

  return mm;
}

/**
 * @param {WebAssembly.Exports} exp
 * @param {string} fname
 * @returns Function
 */
function exports2function(exp, fname) {
  /** @type WebAssembly.ExportValue */
  const ev = exp[fname];

  /** @type any */
  const an = ev;

  /** @type Function */
  const f = an;

  return f;
}

/** @type {IO<Void>} */
const main = () => {
  /** @type IO<ArrayBuffer> */
  const ibuf = filename2buffer(wasmName);

  /** @type IO<WebAssembly.Module> */
  const imdl = bind(ibuf, buffer2module);

  /** @type IO<WebAssembly.Instance> */
  const iins = bind(imdl, module2instance);

  return Promise.resolve()
    .then((_) => iins())
    .then((instance) => {
      /** @type WebAssembly.Exports */
      const exports = instance.exports;

      /** @type WebAssembly.Memory */
      const memory = exports2memory(exports);

      /** @type Function */
      const intSet = exports2function(exports, "integer_set");

      /** @type Function */
      const fltSet = exports2function(exports, "real_set");

      /** @type Function */
      const setTrue = exports2function(exports, "bool_set_true");

      /** @type Function */
      const setFalse = exports2function(exports, "bool_set_false");

      /** @type Function */
      const intGet = exports2function(exports, "integer_value");

      /** @type Function */
      const encode = exports2function(exports, "encode");

      /** @type Function */
      const offset = exports2function(exports, "offset");

      intSet(BigInt(42));
      fltSet(42.0);
      setTrue();
      setFalse();

      /** @type number */
      const size = encode();

      /** @type number */
      const off = offset();

      /** @type ArrayBuffer */
      const buf = memory.buffer;

      const view = new DataView(buf, off, size);

      return writeFile("/dev/stdout", view);
    });
};

main().catch(console.error);
