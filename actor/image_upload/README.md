# Image upload Actor

This actor listens for http commands 
and uploads images to a blobstore capability provider.

The instructions below are tailored for the blobstore-s3 provider 
and refer to "buckets", although the functionality described below 
should be the same for other blobstore implementations. 
(with one exception: The 'alias' feature allows a blobstore-s3
link definition to contain a mapping that replaces bucket names. This is
a feature of the link definition, not a feature of the blobstore api).

## Build and Install

You will need to have rust SDK and wasmcloud developer setup
   https://wasmcloud.dev/overview/installation

1. **AWS Setup**
 
   - Provision an AWS account, and the assumed role account, if you plan to use Sts AssumeRole.
   The first account should have a _trust policy_ that allows the principal to perform `sts:AssumeRole`.
   The assumed role account (or first account if not using AssumeRole) should have the following s3 permissions:
     - s3:ListAllMyBuckets, s3:ListBucket, s3:PutObject, s3:GetObject

   - Note that use of all the features of the blobstore-s3 api requires additional permissions, 
   including CreateBucket, DeleteObject, etc., however if the blobstore-s3 is only used through the image_upload
   actor, the more limited list of permissions above should be sufficient. Please consult the IAM and S3 documentation
   for details on setting access policies.

   - Create a bucket for uploading images. For this example, we use the bucket name `images.from.wasmcloud`. 

     ```shell
     aws s3 mb s3://images.from.wasmcloud
     ```

   - If you wish to use a different bucket name, edit `image_upload/Makefile` to
   replace `images.from.wasmcloud` with your preferred bucket name.
   The upload_actor uses an alias "images", which is mapped to the real bucket name through the link with the provider.

2. **Build the capability provider and actor**

   - clone `github.com/wasmcloud/capability-providers` and `github.com/wasmcloud/examples`.
    
   - In `capability-providers/blobstore-s3`, run `make`. 
     If you want to run the test suite, `make test`. All tests should 
     pass. (Ignore "ERROR nats::jetstream failed to parse API response" which is not a test error.)

   - In `examples/actor/image_upload`, run `make`


3. **Configure and Start wasmcloud**

   - start nats and a local registry
     ```shell
     docker run -d --rm --name nats -p 127.0.0.1:4222:4222 nats:2.7.2 -js
     docker run -d --rm --name registry -p 127.0.0.1:5000:5000 registry:2.7
     ```
   - set up aws credentials and environment
     - The blobstore-s3 provider will fetch credentials from aws-standard config
     files (~/.aws/config and ~/.aws/credentials), or the environment, or values
     in the link definition. See blobstore-s3/README.md for more details.
     - If you are using environment variables, you must define, at a minimum,
     `AWS_ACCONT_KEY_ID`, `AWS_SECRET_KEY`, and `AWS_REGION`.
       (AWS_REGION can be a multi-region endpoint if desired).
     - also set
     ```shell
     export WASMCLOUD_OCI_ALLOWED_INSECURE=localhost:5000
     export RUST_LOG=debug
     ```
   - [Start the wasmcloud host](https://wasmcloud.dev/overview/installation/).
   Keep a terminal window open tailing the host
  log.


4. **Start capability providers and actor**

   - in blobstore-s3, run `make inspect` and look for the 56-character
   Service key starting with `V`. Copy the key and paste it into
   `image_upload/Makefile` to replace the value for `BLOBSTORE_ID` (near line 27).

   - in blobstore-s3, `make run`, and wait for it to complete.
   - in image_upload, `make run`.
   - If there were no startup errors, you should be ready to go.

## Run

The actor understands a small set of HTTP commands, which can be
demonstrated with `curl`. To avoid leaking server details to an
attacker, most errors return '404 not found', and you'll need to inspect
the host logs for more detail. These commands generate output in json, so
you might want to pipe the output to `jq` to make it more readable.
(using `jq` may require running curl in silent mode (`-s`) to prevent it from
ending extraneous non-json progress info to stdout)

### List buckets

List all readable buckets.

```shell
curl -s localhost:8080/containers | jq
```

### List images

List contents of the image bucket. 
Note that the `/images` part of the url is not the name of the bucket, it is the alias. 

```shell
curl -s localhost:8080/images | jq
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

Because Sha256 is effectively collision-free, presence of the sha value in the 
bucket means that the image has already been uploaded, and the actor will not attempt
to send the file to s3 again. Whether a file is uploaded the first time, or nth time,
the sha values are the same, and the content of the response to the HTTP PUT is the same.

(Currently the non-duplicate guarantee only applies if the file type is detected. If the file type is not recognized,
the current implementation uses the file extension provided with the request,
so requests with the same not-recognized file and different extensions could result in multiple
copies of the file in the bucket, with the same sha filename prefix and different extensions.)
