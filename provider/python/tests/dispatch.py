# compute n factorial
def factorial(n):
    val = 1
    for i in range(2, n + 1):
        val = val * i
    return val


def big_response(n):
    val = 'x' * n
    return val


# say hello
def say_hello(name):
    return "Hello {}!".format(name)


# dispatch wasmbus command
def handle(command, arg):
    if command.endswith("factorial"):
        return factorial(arg)

    if command.endswith("hello"):
        return say_hello(arg)

    if command.endswith("big_response"):
        return big_response(arg)

    return "unknown command {}".format(command)
