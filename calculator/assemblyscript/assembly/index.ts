import { Request, Response, ResponseBuilder, Handlers as HTTPHandlers } from "@wasmcloud/actor-http-server";
import { HealthCheckResponse, HealthCheckRequest, Handlers as CoreHandlers, HealthCheckResponseBuilder } from "@wasmcloud/actor-core";

export function wapc_init(): void {
  CoreHandlers.registerHealthRequest(HealthCheck);
  HTTPHandlers.registerHandleRequest(HandleRequest);
}

function HealthCheck(request: HealthCheckRequest): HealthCheckResponse {
  return new HealthCheckResponseBuilder().withHealthy(true).withMessage("AssemblyScript Calculator Healthy").build();
}

function HandleRequest(request: Request): Response {
  
  const nums = request.queryString.split(",")
  const numOne = parseInt(nums[0])
  const numTwo = parseInt(nums[1])
  let result: string;
  
  if (request.path == "add") {
    result = "add: " + numOne.toString() + " + " + numTwo.toString() + " = " + (numOne + numTwo).toString()
  }
  else if (request.path == "sub") {
    result = "subtract: " + numOne.toString() + " - " + numTwo.toString() + " = " + (numOne - numTwo).toString()
//TODO: add multiplication
  } else if (request.path == "div") {
    if (numTwo === 0) {
      result = "Can not divide by zero!"
    } else {
      result = "divide: " + numOne.toString() + " / " + numTwo.toString() +  " = " +  (numOne / numTwo).toString()
    }
  } else {
    result = "Unsupported operation"
  }
  
  return new ResponseBuilder()
    .withStatusCode(200)
    .withStatus("OK")
    .withBody(String.UTF8.encode(result, true))
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
