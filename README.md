# JSON Schema Validator

A simple binary utility that verifies a JSON document against a schema.  Exits with `1` and error messages if there's a problem, exits `0` and silence if everything is ok.  Designed for CI pipelines and/or other automation.

```
Usage: json_schema_validator --document <DOCUMENT> --schema <SCHEMA>

Options:
      --document <DOCUMENT>
          JSON document to validate

      --schema <SCHEMA>
          JSON schema to use for validation

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Example documnent included in this repo.
```
$ cargo run -- --schema ./example.schema.json --document ./example.json
$ echo $?
0
```
