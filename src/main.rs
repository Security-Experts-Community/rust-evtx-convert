use clap::Parser;
use evtx::{EvtxParser, ParserSettings};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use xml2json_rs::JsonConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    evtx: PathBuf,
    #[arg(short, long)]
    json: PathBuf,
}

fn main() {
    let args = Args::parse();

    let configuration = ParserSettings::default();
    let mut parser = EvtxParser::from_path(args.evtx)
        .unwrap()
        .with_configuration(configuration);

    let outf = File::create(args.json).expect("Unable to open json file");
    let mut f = BufWriter::new(outf);
    f.write(b"\xef\xbb\xbf").unwrap(); // utf-8 BOM

    let json_builder = JsonConfig::new()
        .ignore_attrs(false)
        .merge_attrs(true)
        .explicit_array(false)
        .charkey("text")
        .finalize();
    for record in parser.records() {
        match record {
            Ok(r) => {
                f.write_all(
                    json_builder
                        .build_string_from_xml(&r.data)
                        .unwrap()
                        .replace("\\r\\n", "\\n")
                        .replace("\"\\n    \"", "null")
                        .as_bytes(),
                )
                .unwrap();
                f.write(b"\r\n").unwrap();
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
