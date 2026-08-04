/// <reference lib="dom" />

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export default function init(input?: InitInput | Promise<InitInput>): Promise<void>;
