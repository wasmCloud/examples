import { Request, Response, ResponseBuilder, Handlers as HTTPHandlers } from "../../../actor-interfaces/assemblyscript/http-server/assembly/module";
import { Host as KV } from "../../../actor-interfaces/assemblyscript/keyvalue/assembly/module";
import { HealthCheckResponse, HealthCheckRequest, Handlers as CoreHandlers } from "../../../actor-interfaces/assemblyscript/actor-core/assembly/module";
import { JSONEncoder } from "assemblyscript-json";

export function wapc_init(): void {
  CoreHandlers.registerHealthRequest(HealthCheck);
  HTTPHandlers.registerHandleRequest(HandleRequest);
}

function HealthCheck(request: HealthCheckRequest): HealthCheckResponse {
  return new HealthCheckResponse();
}

function HandleRequest(request: Request): Response {
  const kv = new KV("");
  const key = request.path.replace("/", ":");
  const result = kv.Add(key, 1);

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
