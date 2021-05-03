import {
  register,
  handleCall,
  hostCall,
  handleAbort,
} from "../assembly";

register("hello", function(payload: ArrayBuffer): ArrayBuffer {
  hostCall("myBinding", "sample", "hello", String.UTF8.encode("Simon"))
  return String.UTF8.encode("Hello")
})

// This must be present in the entry file.
export function __guest_call(operation_size: usize, payload_size: usize): bool {
  return handleCall(operation_size, payload_size);
}

// Abort function
function abort(message: string | null, fileName: string | null, lineNumber: u32, columnNumber: u32): void {
  handleAbort(message, fileName, lineNumber, columnNumber)
}