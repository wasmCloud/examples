# wasmCloud HTTP Server Actor Interface
 
This module provides wasmCloud actors with an interface to the HTTP Server capability provider. Actors using this
interface must have the claim `wasmcloud:httpserver` in order to have permission to handle requests, and they
must have an active, configured binding to an HTTP Server capability provider.

The HTTP Server provider is one-way, and only delivers messages to actors. Actors cannot make host calls
to this provider.

## Sample Actor
```typescript
import { Request, Response, ResponseBuilder, Handlers as HTTPHandlers } from "@wasmcloud/actor-http-server";
import { HealthCheckResponse, HealthCheckRequest, Handlers as CoreHandlers, HealthCheckResponseBuilder } from "@wasmcloud/actor-core";
import { JSONEncoder } from "assemblyscript-json";

export function wapc_init(): void {
  CoreHandlers.registerHealthRequest(HealthCheck);
  HTTPHandlers.registerHandleRequest(HandleRequest);
}

function HealthCheck(request: HealthCheckRequest): HealthCheckResponse {
  return new HealthCheckResponseBuilder().withHealthy(true).withMessage("AssemblyScript KVCounter Healthy").build();
}

function HandleRequest(request: Request): Response {
  return new ResponseBuilder()
    .withStatusCode(200)
    .withStatus("OK")
    .build(); //(TODO) Implement
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
```