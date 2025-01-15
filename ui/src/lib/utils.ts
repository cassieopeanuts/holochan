    
  import { Base64 } from "js-base64";
  import type { CellId } from "@holochain/client";
  import type { CellIdB64 } from "./types";

  export function encodeCellIdToBase64(cellId: CellId): CellIdB64 {
    return Base64.fromUint8Array(new Uint8Array([...cellId[0], ...cellId[1]]), true);
  }
  
  export function decodeCellIdFromBase64(base64: CellIdB64): CellId {
    const bytes = Base64.toUint8Array(base64);
    return [bytes.slice(0, 39), bytes.slice(39)];
  }
  