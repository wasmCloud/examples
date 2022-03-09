# Image upload Actor

This actor listens for http commands 
and uploads images to a blobstore capability provider.

The instructions below are for the blobstore-s3 and refer to "buckets",
although the functionality described below should work for other
blobstore implementations. (* note: The 'alias' feature allows a blobstore-s3
link definition to contain a mapping that replaces bucket names. This is
not strictly part of the interface specification, but is a link feature
of blobstore-s3.)

## Build and Install

You will need to have rust SDK and wasmcloud developer setup
   https://wasmcloud.dev/overview/installation

1. AWS Setup
   - provision aws account, and ROLE account if using Sts AssumeRole.
   - Create a bucket for uploading images. For this example, we use the
   bucket name `images.from.wasmcloud`. 

   `aws s3 mb s3://images.from.wasmcloud`

   If you wish to use a different bucket name, edit image_upload/Makefile to
   replace `images.from.wasmcloud` with your preferred bucket name.
   This actor uses an alias "images", which is mapped to the real bucket
   name 

2. Build the capability provider and actor.

   - In capability-providers/blobstore-s3, `make`. 
     If you want to run the test suite, `make test`. You should see
     tests passing. (Ignore "ERROR nats::jetstream failed to parse API
     response" which is not a test error.)

   - In examples/actor/image_upload, `make`

3. Configure and Start wasmcloud

  - start nats and a local registry
    `docker run -d --rm --name nats -p 127.0.0.1:4222:4222 nats:2.7.2 -js`
    `docker run -d --rm --name registry -p 127.0.0.1:5000:5000 registry:2.7`
  - setup aws credentials and environment
    - you can do this with either ~/.aws/config and ~/.aws/credentials,
      or setting environment variables (see blobstore-s3/README.md for
      details).
    - be sure to set AWS_ACCONT_KEY_ID, AWS_SECRET_KEY, AWS_REGION,
      and additional vars if you are using STS AssumeRole
    - also set
    `export WASMCLOUD_OCI_ALLOWED_INSECURE=localhost:5000`
    `export RUST_LOG=debug`
  - start wasmcloud host. Keep a terminal window open tailing the host
  log.

4. Start capability providers and actor

   - in blobstore-s3, `make run`, and wait for it to complete.
   - in image_upload, `make run`.
   - If there were no startup errors, you should be ready to go.

## Run

The actor understands a small set of HTTP commands, which can be
demonstrated with `curl`. To avoid leaking server details to an
attacker, most errors return '404 not found', and you'll need to inspect
the host logs for more detail.

### List buckets

List all readable buckets.

```shell
curl localhost:8080/containers | jq
```

### List images

List contents of the image bucket. 
Note that the `/images` part of the url is not the name of the bucket, it is the alias. 

```shell
curl localhost:8080/images | jq
```

### Upload image

Images are uploaded using HTTP PUT. 
The last part of the url path (after `/image/`) should contain the original file name.

```shell
curl -T elephant.jpg localhost:8080/image/elephant.jpg
```

The file name within the bucket will be SHA.EXT, where SHA is the SHA256 digest of the file contents,
as 64 lowercase hex digits, and EXT is an extension based on the file type.

The actor attempts to determine the file type based on magic bytes in the file header, 
using [`guess_format` from the images crate](https://docs.rs/image/latest/image/fn.guess_format.html).
If an image type is found by guess_format, it is used as the extension EXT, otherwise the extension
from the original file name is used. If the original filename had no extension, `data` is used.

The purpose of the file type guessing is to normalize extensions, so files ending in '.JPG', '.jpeg', and '.jpg'
will all end up with a '.jpg' extension.
The original name is returned as part of the json response for the `PUT /image` command, 
but it is not stored anywhere in s3.
