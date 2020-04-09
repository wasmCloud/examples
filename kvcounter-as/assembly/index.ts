import { handleCall, consoleLog, handleAbort } from "wapc-guest-as";
import { Request, Response, ResponseBuilder, Handlers } from "wascc-actor-as/httpserver";
import { Host as KV } from "wascc-actor-as/kv";
import { JSONEncoder } from "assemblyscript-json";

function handleRequest(request: Request): Response {
  const kv = new KV("");
  const key = request.path.replaceAll("/", ":");      
  const result = kv.atomicAdd(key, 1);  
     
  let encoder = new JSONEncoder();

  // Construct output JSON
  encoder.pushObject("");
  encoder.setInteger("count", result.value);  
  encoder.popObject();

  // Get serialized data
  let json: Uint8Array = encoder.serialize();

  return new ResponseBuilder()
    .withStatusCode(200)
    .withStatus("OK")
    .withBody(json.buffer)
    .build();
}


// Ceremony required for module entry points

export function _start(): void {
  Handlers.handleRequest(handleRequest);
}

export function __guest_call(operation_size: usize, payload_size: usize): bool {
  return handleCall(operation_size, payload_size);
}

// Abort function - this should probably be in the actor SDK and not in an actor...
export function abort(
    message: string | null,
    fileName: string | null,
    lineNumber: u32,
    columnNumber: u32
  ): void {
    handleAbort(message, fileName, lineNumber, columnNumber);
  }