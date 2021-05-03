# waPC Guest Library for AssemblyScript

This is the [AssemblyScript](https://assemblyscript.org/) implementation of the **waPC** standard for WebAssembly guest modules. It allows any waPC-compliant WebAssembly host to invoke to procedures inside a AssemblyScript compiled guest and similarly for the guest to invoke procedures exposed by the host.

## Example
The following is a simple example of synchronous, bi-directional procedure calls between a WebAssembly host runtime and the guest module.

```typescript
import {
  register,
  handleCall,
  hostCall,
  handleAbort,
} from "wapc";

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
```

```sh
asc example/hello.ts -b example/hello.wasm --use abort=example/hello/abort --validate --optimize
```