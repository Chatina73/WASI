use clap::{App, Arg};
use std::path::Path;
use std::process;
use witx::load;

pub fn main() {
    let app = App::new("witx")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Validate witx file format")
        .arg(
            Arg::with_name("input")
                .required(true)
                .help("path to root of witx document"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("docs")
                .short("d")
                .long("docs")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    match load(Path::new(app.value_of("input").expect("required arg"))) {
        Ok(doc) => {
            if app.is_present("verbose") {
                println!("{:?}", doc)
            }

            if app.is_present("docs") {
                println!("{}", render_markdown(&doc))
            }
        }
        Err(e) => {
            println!("{}", e.report());
            if app.is_present("verbose") {
                println!("{:?}", e);
            }
            process::exit(1)
        }
    }
}

fn render_markdown(doc: &witx::Document) -> String {
    let mut s = format!("# Type definitions\n");
    for d in doc.datatypes() {
        s += &format!("\n## {}\n", d.name.as_str());
        s += &d.docs.join("\n");
    }
    s
}
