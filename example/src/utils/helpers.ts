import { Buffer } from "buffer";

export function hexToBytes(hex: string) {
  return Buffer.from(hex, 'hex');
}

export function bytesToHex(bytes: Uint8Array) {
  return Buffer.from(bytes).toString('hex');
}
