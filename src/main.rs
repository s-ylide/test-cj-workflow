use std::{fs::File, io::Write, path::Path};

use clap::Parser;

#[derive(Parser)]
struct Arg {
    #[clap(short, value_name = "FILE")]
    /// Path to the input file
    input: Vec<String>,
    #[clap(short, value_name = "FILE")]
    /// Path to the output file
    output: String,
}

fn main() {
    let Arg { output, .. } = Arg::parse();
    let output = output.trim_end_matches(".s");
    let operation = ["flatten", "TACE"];
    let phase = ["before", "after"];
    fn write<P: AsRef<Path>>(path: P, content: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
    for operation in operation {
        for phase in phase {
            write(format!("{output}.{phase}_{operation}"), phase);
        }
    }
}
