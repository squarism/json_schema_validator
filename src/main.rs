use clap::{crate_version, Parser};
use jsonschema::{Draft, JSONSchema};
use std::{fs, process::exit};

#[derive(Parser, Debug)]
#[clap(
    version(crate_version!()),
    about,
    long_about = "json_schema_validator is a CLI program that verifies a JSON document against a schema file.  It will only output on error and set an exit code to non-zero."
)]
struct Cli {
    #[arg(long("document"), help("JSON document to validate"), required(true))]
    document: String,
    #[arg(
        long("schema"),
        help("JSON schema to use for validation"),
        required(true)
    )]
    schema: String,
}

fn main() {
    let args = Cli::parse();

    let schema_file = fs::read_to_string(args.schema).expect("Cannot read schema file");
    let schema = serde_json::from_str(&schema_file).expect("Schema file is not valid?");

    let document_file = fs::read_to_string(args.document).expect("Cannot read document file");
    let document = serde_json::from_str(&document_file).expect("Document file is not valid?");

    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("A valid schema");

    let result = compiled.validate(&document);
    match result {
        Err(errors) => {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
            exit(1);
        }
        Ok(()) => exit(0),
    }
}
