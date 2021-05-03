@external("wapc", "__guest_request")
export declare function __guest_request(operation_ptr: i32, payload_ptr: i32): void
@external("wapc", "__guest_response")
export declare function __guest_response(ptr: i32, len: usize): void
@external("wapc", "__guest_error")
export declare function __guest_error(ptr: i32, len: usize): void

@external("wapc", "__host_call")
export declare function __host_call(
  binding_ptr: i32, binding_len: usize,
  namespace_ptr: i32, namespace_len: usize,
  operation_ptr: i32, operation_len: usize,
  payload_ptr: i32, payload_len: usize): bool
@external("wapc", "__host_response_len")
export declare function __host_response_len(): usize
@external("wapc", "__host_response")
export declare function __host_response(ptr: i32): void
@external("wapc", "__host_error_len")
export declare function __host_error_len(): usize
@external("wapc", "__host_error")
export declare function __host_error(ptr: i32): void

@external("wapc", "__console_log")
export declare function __console_log(ptr: i32, len: usize): void

export type Function = (payload: ArrayBuffer) => ArrayBuffer

var functions = new Map<string, Function>()

export function register(operation: string, fn: Function): void {
  functions.set(operation, fn)
}

function getFunction(name: string): Function {
  if (!functions.has(name)) {
    return errorFunction
  }
  return functions.get(name)
}

function errorFunction(payload: ArrayBuffer): ArrayBuffer {
  return new ArrayBuffer(1)
}

export function handleCall(operation_size: usize, payload_size: usize): bool {
  const operationBuf = new ArrayBuffer(changetype<i32>(operation_size))
  const payload = new ArrayBuffer(changetype<i32>(payload_size))
  __guest_request(changetype<i32>(operationBuf), changetype<i32>(payload));

  const operation = String.UTF8.decode(operationBuf)
  const fn = getFunction(operation)
  if (fn != errorFunction) {
    const response = fn(payload)
    __guest_response(changetype<i32>(response), response.byteLength)
    return true
  }

  const message = String.UTF8.encode("Could not find function \"" + operation + "\"")
  __guest_error(changetype<i32>(message), message.byteLength)
  return false;
}

export function hostCall(binding: string, namespace: string, operation: string, payload: ArrayBuffer): ArrayBuffer {
  const bindingBuf = String.UTF8.encode(binding)
  const namespaceBuf = String.UTF8.encode(namespace)
  const operationBuf = String.UTF8.encode(operation)
  const result = __host_call(
    changetype<i32>(bindingBuf), bindingBuf.byteLength,
    changetype<i32>(namespaceBuf), namespaceBuf.byteLength,
    changetype<i32>(operationBuf), operationBuf.byteLength,
    changetype<i32>(payload), payload.byteLength)
  if (!result) {
      const errorLen = __host_error_len();
      const message = new ArrayBuffer(changetype<i32>(errorLen))
      __host_error(changetype<i32>(message))
      const errorMsg = "Host error: " + String.UTF8.decode(message)
      consoleLog(errorMsg)
      throw new Error(errorMsg)
      //__guest_error(message.ptr, message.len);
      //return error.HostError;
  }

  const responseLen = __host_response_len()
  const response = new ArrayBuffer(changetype<i32>(responseLen))
  __host_response(changetype<i32>(response))

  return response
}

export function consoleLog(message: string): void {
  const messageBuf = String.UTF8.encode(message)
  __console_log(changetype<i32>(messageBuf), messageBuf.byteLength)
}

export function handleAbort(
  message: string | null,
  fileName: string | null,
  lineNumber: u32,
  columnNumber: u32
): void{
  var errorMessage = (message!=null) ?message! :"error occurred"
  if (fileName != null && lineNumber != 0 && columnNumber != 0) {
    errorMessage += "; " + fileName! + " (" + lineNumber.toString() + "," + columnNumber.toString() + ")"
  }
  const messageBuf = String.UTF8.encode(errorMessage)
  
  __guest_error(changetype<i32>(messageBuf), messageBuf.byteLength)
}
