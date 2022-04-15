# Pet Clinic UI

## Prerequisites

* Node, npm, rust, wash

## Building
This UI is built as an actor with the compiled assets embedded inside of it. The `build.rs` file
will run the proper npm steps to output the generated code before embedding it in the module
