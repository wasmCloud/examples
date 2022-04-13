# Python capability provider for wasmCloud

This capability provider allows wasmCloud actors to invoke functions in
Python. It can be used to run Tensorflow, Keras, PyTorch, and other machine
learning models in Python. Because it runs on a native host, it can take
advantage of C libraries, including the [Intel Optimization for Tensorflow](https://www.intel.com/content/www/us/en/developer/articles/guide/optimization-for-tensorflow-installation-guide.html),
GPUs, and any other Python libraries and extensions.

> :warning: This example is for illustrative purposes and is not recommended for production due to security and stability concerns. In addition, the requirement for environment variables and preconfigured python installation limit portability.

This differs from other wasmCloud capability providers in a few
significant ways:

- There is not a smithy-defined interface contract. To invoke this
  provider, an actor passes an operation name and a serialized
  data payload. These are converted to Python objects and passed to a
  dispatch handler function, described below.
- Because of the limitations of the Python Global Interpreter Lock
  (GIL), the provider is (mostly) single threaded. Some attempt is made 
  to increase throughput by using threads and async IO for message handling and
  serialization, but the Python functions are invoked one-at-a-time.
  This has not been tested under load.
- Running this provider requires a Python 3 environment to be already
  configured on the host machine. Although the wasmCloud host could
  download and run the provider from a remote registry, the host
  must be configured as described below. This requirement limits portability
  of the provider.

## Invoking

Here is example of a rust actor invoking the python provider to
compute ten factorial.

```rust
use wasmbus_rpc::{common::AnySender, actor::prelude::*};

async fn run(ctx: &Context) {
    let py = AnySender::new(WasmHost::to_provider("wasmcloud:example:python", "default"));
    let n: i32 = 10;
    let res: i32 = py.send(&ctx, "py.factorial", &n).await?;
    assert_eq!(res, 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2);
}
```

Here's the factorial capability provider in 5 lines of Python code:

```python
def factorial(n):
    val = 1
    for i in range(2, n + 1):
        val = val * i
    return val
```


## Configuration

### Set up python environment

Create a [virtual environment](https://docs.python.org/3/library/venv.html)
and install dependencies

```
python3 -m venv venv
source venv/bin/activate
pip3 install neotasker
```

Install additional dependencies your programs may need.

### Environment variables

Three variables must be set before running the provider. These must be
set in the host's environment before starting wasmCloud host.

- `VENV_PATH` path to the virtual environment.

- `PYTHON_MAIN` a path to a python module (folder) that loads and runs
  the `PYTHON_DISPATCH` program. 

- `PYTHON_DISPATCH` the name of the python program containing 
  the handler function that processes incoming RPC messages. 
  The handler can invoke functions in other libraries in its containing
  folder and in the python path.

See the [`Makefile`](./Makefile) and the [`tests` folder](./tests) 
for an example configuring these variables and the dispatch module.


### Dispatch handler

All messages are received by a `handle` function, which accepts a string
operation name and a Python object. wasmCloud RPC operation names 
typically have the form `Interface.Method`, where `Interface` is the name of the
service interface. Although the Python capability provider does not
require operation names to be in this form, this format is recommended
to retain compatibility with future debugging tools. The example
code below accepts this format and ignores the prefix.

```python
# dispatch RPC command
def handle(command, arg):
    if command.endswith("factorial"):
        return factorial(arg)
    if command.endswith("hello"):
        return say_hello(arg)
    return "unknown command {}".format(command)
```

(A more generic handler could be written to lookup a Python module
from the `Interface` prefix, and avoid the `if` statements entirely 
by letting Python's interpreter invoke the method, e.g., `(*method)(arg)`.)

The function parameter from the actor is serialized to CBOR, and
converted to a native Python value using serde and pyo3. The return
value follows the reverse process to be converted to a value in the
actor's native language. This generlc serialization process takes
advantage of CBOR's self-describing format, and lets us send RPC messages
without code generation used by other capability providers.

Parameter and return Values may be primitive types 
(integer, float, boolean, string), lists of Values, 
and dictionaries (hashmaps with string keys).


## Reloading

The Python code is automatically reloaded if the `PYTHON_DISPATCH` 
program is modified (its modified time is checked before each invocation),
or if a "reload" command is received. This feature is intended to aid
iterative development, as it avoids the need to stop and restart the
capability provider when the code changes. For slightly better security 
in production, the "reload" command can be disabled in `__init__.py`.

