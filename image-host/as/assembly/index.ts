import { Request, Response, ResponseBuilder, Handlers as HTTPHandlers } from "@wasmcloud/actor-http-server";
import { Host as Blob, ContainerBuilder, FileChunkBuilder } from "@wasmcloud/actor-blobstore";
import { HealthCheckResponse, HealthCheckRequest, Handlers as CoreHandlers, HealthCheckResponseBuilder } from "@wasmcloud/actor-core";

export function wapc_init(): void {
  CoreHandlers.registerHealthRequest(HealthCheck);
  HTTPHandlers.registerHandleRequest(HandleRequest);
}

function HandleRequest(request: Request): Response {
  if (request.method == "GET") {
    return download_image();
  } else if (request.method == "POST") {
    return upload_image(request.path, request.body);
  } else {
    return new ResponseBuilder()
      .withStatusCode(400)
      .withStatus("Bad Request")
      .withBody(String.UTF8.encode(`method ${request.method} not supported`, true))
      .build();
  }
}

function upload_image(path: string, image_bytes: ArrayBuffer): Response {
  const blobstore = new Blob("default");
  let container = new ContainerBuilder().withId("wasmcloud-bucket").build();
  let image = new FileChunkBuilder()
    .withSequenceNo(0)
    .withContainer(container)
    .withId(path.replaceAll("/", "").substring(1))
    .withTotalBytes(image_bytes.byteLength)
    .withChunkSize(image_bytes.byteLength)
    .withChunkBytes(image_bytes)
    .build();
  blobstore.StartUpload(image);
  blobstore.UploadChunk(image);
  return new ResponseBuilder()
    .withStatusCode(200)
    .withStatus("OK")
    .withBody(String.UTF8.encode("upload successful", true))
    .build();
}

function download_image(): Response {
  return new ResponseBuilder()
    .withStatusCode(400)
    .withStatus("Bad Request")
    .withBody(String.UTF8.encode("download not implemented", true))
    .build();
}

function HealthCheck(request: HealthCheckRequest): HealthCheckResponse {
  return new HealthCheckResponseBuilder().withHealthy(true).withMessage("Image Host (AS) Healthy").build();
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
