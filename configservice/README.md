# Example Configuration Service
This is a demonstration of how you can build your own configuration service to supply wasmCloud hosts with a set of startup providers and actors as well as a map of registry
credentials where the key is the registry's root URL (e.g. https://wasmcloud.azurecr.io).

This is a **SAMPLE** only. Choices were deliberately made in this project to make it clear that this is not to be run in production. Each organization is likely going to have different requirements for their configuration service, and this code only illustrates how to satisfy the contract for the configuration service.