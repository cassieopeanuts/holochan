
  export type CellIdB64 = string;

  export interface LocalFile {
    file: File;
    dataURL: string;
  
    // Unique key used only by UI to give a persistent unique reference when rendered in a list
    // So that removing elements from the middle list removes that DOM element from the middle of the list
    // rather than from the end of the list.
    //
    // See https://svelte.dev/tutorial/svelte/keyed-each-blocks
    key: string;
  }
  
  export enum FileStatus {
    Pending, // Sent to holochain, but not published yet
    Loading, // Fetched from holochain
    Loaded, // Fetched from holochain and loaded into base64 data url
    Error,
  }
  
  export interface FileExtended {
    file?: File;
    status: FileStatus;
  }  