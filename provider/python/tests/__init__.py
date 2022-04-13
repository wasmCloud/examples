# Python capability provider
from importlib import reload
from . import dispatch

print("Hello from python!")

# pass command to dispatcher
def main(command, arg):
    if command == "reload":
        reload(dispatch)
        return True
    else:
        return dispatch.handle(command, arg["arg"])
