export declare type Function = (payload: ArrayBuffer) => ArrayBuffer;
export declare function register(operation: string, fn: Function): void;
export declare function handleCall(operation_size: usize, payload_size: usize): bool;
export declare function hostCall(module: string, operation: string, payload: ArrayBuffer): ArrayBuffer;
export declare function consoleLog(message: string): void;
export declare function handleAbort(message: string | null, fileName: string | null, lineNumber: u32, columnNumber: u32): void;
