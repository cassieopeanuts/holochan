// projects/holochan/ui/src/lib/utils.ts

import { encodeHashToBase64, decodeHashFromBase64, type HoloHash } from "@holochain/client";
import type { CellId } from "@holochain/client";
import type { CellIdB64 } from "./types";

/**
 * Encodes a CellId to a Base64 string using Holochain's utility.
 * @param cellId - The CellId to encode.
 * @returns The Base64-encoded CellId.
 */
export function encodeCellIdToBase64(cellId: CellId): CellIdB64 {
  const [agentPubKey, dnaHash] = cellId;
  const agentPubKeyB64 = encodeHashToBase64(agentPubKey);
  const dnaHashB64 = encodeHashToBase64(dnaHash);
  return `${agentPubKeyB64}.${dnaHashB64}`; // Using a delimiter to separate hashes
}

/**
 * Decodes a Base64 string back to a CellId using Holochain's utility.
 * @param base64 - The Base64-encoded CellId.
 * @returns The decoded CellId.
 */
export function decodeCellIdFromBase64(base64: CellIdB64): CellId {
  const [agentPubKeyB64, dnaHashB64] = base64.split(".");
  const agentPubKey = decodeHashFromBase64(agentPubKeyB64);
  const dnaHash = decodeHashFromBase64(dnaHashB64);
  return [agentPubKey, dnaHash];
}

/**
 * Converts a HoloHash (Uint8Array) to a Base64 string using Holochain's utility.
 * @param hash - The HoloHash to convert.
 * @returns The Base64-encoded HoloHash.
 */
export function holoHashToBase64(hash: HoloHash): string {
  return encodeHashToBase64(hash);
}

/**
 * Converts a Base64 string to a HoloHash (Uint8Array) using Holochain's utility.
 * @param hashB64 - The Base64-encoded HoloHash.
 * @returns The decoded HoloHash as Uint8Array.
 */
export function base64ToHoloHash(hashB64: string): HoloHash {
  return decodeHashFromBase64(hashB64);
}
