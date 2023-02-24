# The Joker

This is a demo that leverages actor to actor calls.

### Why? 

At this time, TinyGo actors can't import/use any Go package that it wants.  And example of this is the JSON package.

Here, we have a API Gateway that was written in go...and we _NEED_ to be able to unmarshal some JSON into a struct, but we can't 😓

### Solution 

While we wait on TinyGo's JSON package to support stand alone webassembly, we can just ask Rust from some help.

In this example, the API Gateway (Go) gets a payload from the internet, sends it over to the parser (Rust) and gets a structure in return.

## How to use?

- Clone the repo
- Build the interface files -> run `wash gen` from the root of the repo
- CD into each actor directory and `wash build`
- Start from file inside the washboard 

> You'll need an httpserver and httpclient provider.  Link them both to the API Gateway
