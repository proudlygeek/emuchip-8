/* tslint:disable */
import * as wasm from './emuchip_8_bg';

export function __wbg_random_fabf73e8a709437c() {
    return Math.random();
}

function freeEmulator(ptr) {

    wasm.__wbg_emulator_free(ptr);
}
/**
*/
export class Emulator {

    static __wrap(ptr) {
        const obj = Object.create(Emulator.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeEmulator(ptr);
    }

    /**
    * @returns {Emulator}
    */
    static new() {
        return Emulator.__wrap(wasm.emulator_new());
    }
    /**
    * @returns {void}
    */
    load_fontset() {
        return wasm.emulator_load_fontset(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_memory() {
        return wasm.emulator_get_memory(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_gfx() {
        return wasm.emulator_get_gfx(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_keys() {
        return wasm.emulator_get_keys(this.ptr);
    }
    /**
    * @returns {void}
    */
    tick() {
        return wasm.emulator_tick(this.ptr);
    }
    /**
    * @returns {boolean}
    */
    draw_flag() {
        return (wasm.emulator_draw_flag(this.ptr)) !== 0;
    }
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

