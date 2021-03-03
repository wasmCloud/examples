# Ping Interface

While you do not have to create your own actor interface, using `widl`-based code generation just makes things easier. Actor interfaces aren't _just_ for allowing actors and capabilities to communicate--you can create an actor interface that allows actors to communicate with each other as well.

In this case, the `Ping` interface has a single operation called `Ping`. The `ponger` actor registers a ping handler, which is invoked by the `pinger` actor directly.
