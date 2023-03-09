# `wasi-messaging-demo`

This repo is based on https://github.com/danbugs/wasi-messaging-demo


- 'guest/' contains source for a component using wit messaging interface
- 'adapter/' source for a component that adapts wit messaging to wasmcloud (in progress)
- 'host/' example host implementation that loads and runs the adapter as a component



## Repository Structure

- `src/`: contains an example guest implementation of the interface.
- `host/`: contains an example host implementation of the interface.

## Run

To run the demo.:

```sh
make build
make componentize
make run
```
