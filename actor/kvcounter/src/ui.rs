use rust_embed::RustEmbed;

/// This embeds the compiled static assets inside of the WebAssembly module
#[derive(RustEmbed)]
#[folder = "./ui/build"]
pub(crate) struct Asset;
