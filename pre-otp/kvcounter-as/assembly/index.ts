import { Request, Response, ResponseBuilder, Handlers as HTTPHandlers } from "@wasmcloud/actor-http-server";
import { Host as KV } from "@wasmcloud/actor-keyvalue";
import { HealthCheckResponse, HealthCheckRequest, Handlers as CoreHandlers, HealthCheckResponseBuilder } from "@wasmcloud/actor-core";

export function wapc_init(): void {
  CoreHandlers.registerHealthRequest(HealthCheck);
  HTTPHandlers.registerHandleRequest(HandleRequest);
}

function HealthCheck(request: HealthCheckRequest): HealthCheckResponse {
  return new HealthCheckResponseBuilder().withHealthy(true).withMessage("AssemblyScript KVCounter Healthy").build();
}

function HandleRequest(request: Request): Response {
  const kv = new KV("default");
  const key = request.path.replace("/", ":");
  const result = kv.Add(key, 1);

  // Get serialized data
  let json = Uint8Array.wrap(String.UTF8.encode('{"greeting":"hello"}'));

  return new ResponseBuilder()
    .withStatusCode(200)
    .withStatus("OK")
    .withBody(json.buffer)
    .build();
}

// Boilerplate code for waPC.  Do not remove.
import { handleCall, handleAbort } from "@wapc/as-guest";

export function __guest_call(operation_size: usize, payload_size: usize): bool {
  return handleCall(operation_size, payload_size);
}

// Abort function
function abort(
  message: string | null,
  fileName: string | null,
  lineNumber: u32,
  columnNumber: u32
): void {
  handleAbort(message, fileName, lineNumber, columnNumber);
}
