// Copyright 2018 the Deno authors. All rights reserved. MIT license.
import { TypedArray } from "./types";

let logDebug = false;

// @internal
export function setLogDebug(debug: boolean): void {
    logDebug = debug;
}

/**
 * Debug logging for deno.
 * Enable with the `--log-debug` or `-D` command line flag.
 * @internal
 */
// tslint:disable-next-line:no-any
export function log(...args: any[]): void {
    if (logDebug) {
        console.log("DEBUG JS -", ...args);
    }
}

// @internal
export function assert(cond: boolean, msg = "assert") {
    if (!cond) {
        throw Error(msg);
    }
}

// @internal
export function typedArrayToArrayBuffer(ta: TypedArray): ArrayBuffer {
    const ab = ta.buffer.slice(ta.byteOffset, ta.byteOffset + ta.byteLength);
    return ab as ArrayBuffer;
}

export function arrayBufferToString(ab: ArrayBuffer): string {
    return new TextDecoder("utf-8").decode(ab);
}

export function arrayToString(ui8: Uint8Array): string {
    return String.fromCharCode(...ui8);
}

export function stringToArrayBuffer(str: string): ArrayBuffer {
    return new TextEncoder().encode(str);
}

/**
 * A `Resolvable` is a Promise with the `reject` and `resolve` functions
 * placed as methods on the promise object itself. It allows you to do:
 *
 *     const p = createResolvable<number>();
 *     ...
 *     p.resolve(42);
 *
 * It'd be prettier to make Resolvable a class that inherits from Promise,
 * rather than an interface. This is possible in ES2016, however typescript
 * produces broken code when targeting ES5 code.
 * See https://github.com/Microsoft/TypeScript/issues/15202
 * At the time of writing, the github issue is closed but the problem remains.
 *
 * @internal
 */

export interface ResolvableMethods<T> {
    resolve: (value?: T | PromiseLike<T>) => void;
    // tslint:disable-next-line:no-any
    reject: (reason?: any) => void;
}

// @internal
export type Resolvable<T> = Promise<T> & ResolvableMethods<T>;

// @internal
export function createResolvable<T>(): Resolvable<T> {
    let methods: ResolvableMethods<T>;
    const promise = new Promise<T>((resolve, reject) => {
        methods = { resolve, reject };
    });
    // TypeScript doesn't know that the Promise callback occurs synchronously
    // therefore use of not null assertion (`!`)
    return Object.assign(promise, methods!) as Resolvable<T>;
}

// @internal
export function notImplemented(): never {
    throw new Error("Not implemented");
}

// @internal
export function unreachable(): never {
    throw new Error("Code not reachable");
}

// @internal
export function hexdump(u8: Uint8Array): string {
    return Array.prototype.map
        .call(u8, (x: number) => {
            return ("00" + x.toString(16)).slice(-2);
        })
        .join(" ");
}

// @internal
export function containsOnlyASCII(str: string): boolean {
    if (typeof str !== "string") {
        return false;
    }
    return /^[\x00-\x7F]*$/.test(str);
}

// @internal
export function isError(err: any): err is Error {
    return err instanceof Error || (err && typeof err.message === 'string');

export function isIterable<T>(obj: Array<T> | IterableIterator<T> | any): obj is IterableIterator<T> {
    if (obj == null) {
        return false;
    }
    if (typeof obj !== "object") {
        return false;
    }
    return typeof obj[Symbol.iterator] === 'function';
}
}