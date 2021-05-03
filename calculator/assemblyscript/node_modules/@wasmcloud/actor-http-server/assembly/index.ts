import { Decoder, Writer, Encoder, Sizer, Codec } from "@wapc/as-msgpack";

import { register } from "@wapc/as-guest";
export class Handlers {
  // Register a function to handle an incoming HTTP request from a linked provider
  static registerHandleRequest(handler: (request: Request) => Response): void {
    HandleRequestHandler = handler;
    register("HandleRequest", HandleRequestWrapper);
  }
}

var HandleRequestHandler: (request: Request) => Response;
function HandleRequestWrapper(payload: ArrayBuffer): ArrayBuffer {
  const decoder = new Decoder(payload);
  const request = new Request();
  request.decode(decoder);
  const response = HandleRequestHandler(request);
  return response.toBuffer();
}

// HTTP Request object
export class Request implements Codec {
  method: string = "";
  path: string = "";
  queryString: string = "";
  header: Map<string, string> = new Map<string, string>();
  body: ArrayBuffer = new ArrayBuffer(0);

  static decodeNullable(decoder: Decoder): Request | null {
    if (decoder.isNextNil()) return null;
    return Request.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): Request {
    const o = new Request();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "method") {
        this.method = decoder.readString();
      } else if (field == "path") {
        this.path = decoder.readString();
      } else if (field == "queryString") {
        this.queryString = decoder.readString();
      } else if (field == "header") {
        this.header = decoder.readMap(
          (decoder: Decoder): string => {
            return decoder.readString();
          },
          (decoder: Decoder): string => {
            return decoder.readString();
          }
        );
      } else if (field == "body") {
        this.body = decoder.readByteArray();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(5);
    encoder.writeString("method");
    encoder.writeString(this.method);
    encoder.writeString("path");
    encoder.writeString(this.path);
    encoder.writeString("queryString");
    encoder.writeString(this.queryString);
    encoder.writeString("header");
    encoder.writeMap(
      this.header,
      (encoder: Writer, key: string): void => {
        encoder.writeString(key);
      },
      (encoder: Writer, value: string): void => {
        encoder.writeString(value);
      }
    );
    encoder.writeString("body");
    encoder.writeByteArray(this.body);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): RequestBuilder {
    return new RequestBuilder();
  }
}

export class RequestBuilder {
  instance: Request = new Request();

  withMethod(method: string): RequestBuilder {
    this.instance.method = method;
    return this;
  }

  withPath(path: string): RequestBuilder {
    this.instance.path = path;
    return this;
  }

  withQueryString(queryString: string): RequestBuilder {
    this.instance.queryString = queryString;
    return this;
  }

  withHeader(header: Map<string, string>): RequestBuilder {
    this.instance.header = header;
    return this;
  }

  withBody(body: ArrayBuffer): RequestBuilder {
    this.instance.body = body;
    return this;
  }

  build(): Request {
    return this.instance;
  }
}

// HTTP Response object
export class Response implements Codec {
  statusCode: u32 = 0;
  status: string = "";
  header: Map<string, string> = new Map<string, string>();
  body: ArrayBuffer = new ArrayBuffer(0);

  static decodeNullable(decoder: Decoder): Response | null {
    if (decoder.isNextNil()) return null;
    return Response.decode(decoder);
  }

  // decode
  static decode(decoder: Decoder): Response {
    const o = new Response();
    o.decode(decoder);
    return o;
  }

  decode(decoder: Decoder): void {
    var numFields = decoder.readMapSize();

    while (numFields > 0) {
      numFields--;
      const field = decoder.readString();

      if (field == "statusCode") {
        this.statusCode = decoder.readUInt32();
      } else if (field == "status") {
        this.status = decoder.readString();
      } else if (field == "header") {
        this.header = decoder.readMap(
          (decoder: Decoder): string => {
            return decoder.readString();
          },
          (decoder: Decoder): string => {
            return decoder.readString();
          }
        );
      } else if (field == "body") {
        this.body = decoder.readByteArray();
      } else {
        decoder.skip();
      }
    }
  }

  encode(encoder: Writer): void {
    encoder.writeMapSize(4);
    encoder.writeString("statusCode");
    encoder.writeUInt32(this.statusCode);
    encoder.writeString("status");
    encoder.writeString(this.status);
    encoder.writeString("header");
    encoder.writeMap(
      this.header,
      (encoder: Writer, key: string): void => {
        encoder.writeString(key);
      },
      (encoder: Writer, value: string): void => {
        encoder.writeString(value);
      }
    );
    encoder.writeString("body");
    encoder.writeByteArray(this.body);
  }

  toBuffer(): ArrayBuffer {
    let sizer = new Sizer();
    this.encode(sizer);
    let buffer = new ArrayBuffer(sizer.length);
    let encoder = new Encoder(buffer);
    this.encode(encoder);
    return buffer;
  }

  static newBuilder(): ResponseBuilder {
    return new ResponseBuilder();
  }
}

export class ResponseBuilder {
  instance: Response = new Response();

  withStatusCode(statusCode: u32): ResponseBuilder {
    this.instance.statusCode = statusCode;
    return this;
  }

  withStatus(status: string): ResponseBuilder {
    this.instance.status = status;
    return this;
  }

  withHeader(header: Map<string, string>): ResponseBuilder {
    this.instance.header = header;
    return this;
  }

  withBody(body: ArrayBuffer): ResponseBuilder {
    this.instance.body = body;
    return this;
  }

  build(): Response {
    return this.instance;
  }
}
