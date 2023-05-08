# cron capability provider

This capability provider implements the "wasmcloud:example:cron" capability. It accepts link definitions with a cron `expression`, or individual values (listed below). The expression/values specify a timed interval to invoke the linked wasmCloud actor on its `timed_invoke` handler.

See [Cron Expressions](https://docs.oracle.com/cd/E12058_01/doc/doc.1014/e12030/cron_expressions.htm) for more about this format. The examples are great inspiration for how to configure this provider. Taking one of the examples, **`0 15 10 * * ? *`: Fire at 10:15 AM every day**, you could either supply the full `expression` or the individual values.

```bash
wash ctl link put <actor> <provider> wasmcloud:example:cron expression="0 15 10 * * ? *"
wash ctl link put <actor> <provider> wasmcloud:example:cron second=0 minute=15 hour=10 day_of_week=?
```

# Link Values

| Key           | Description                               | Example         | Default |
| ------------- | ----------------------------------------- | --------------- | ------- |
| `expression`  | A full cron expression                    | `"* * * * * *"` | N/A     |
| `second`      | A cron statement for the second slot      | `0`             | `*`     |
| `minute`      | A cron statement for the minute slot      | `0`             | `*`     |
| `hour`        | A cron statement for the hour slot        | `0`             | `*`     |
| `day`         | A cron statement for the day slot         | `0`             | `*`     |
| `month`       | A cron statement for the month slot       | `0`             | `*`     |
| `day_of_week` | A cron statement for the day of week slot | `0`             | `*`     |
| `year`        | A cron statement for the yearslot         | `0`             | `*`     |
