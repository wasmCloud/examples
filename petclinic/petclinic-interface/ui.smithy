namespace org.wasmcloud.examples.petclinic

use org.wasmcloud.model#wasmbus

@wasmbus( actorReceive: true )
service Ui {
  version: "0.1",
  operations: [GetAsset]
}

/// Gets the asset with the given path. The input string should be the path part of a URL with the
/// leading `/`
operation GetAsset {
    input: String,
    output: GetAssetResponse
}

structure GetAssetResponse {
    /// True if the asset was found, false if request was successful, but asset was not found
    @required
    found: Boolean,

    /// Optionally hint to the caller what the content type is. Should be a valid MIME type
    contentType: String,

    /// The raw asset as bytes
    @required
    asset: Blob,
}
