use tomllib::TOMLParser;
use tomllib::types::Value;
use std::fs::File;
use std::path::{Path};
use std::io::*;

static ENDINGS: &'static str = "jpg;png;gif;tiff;bmp;jpg-large;jpeg";
static OPTIONS_FILE_NAME: &'static str = ".rust_gtk_thumbnailer.conf.toml";

pub struct RustThumbnailTilerOptions {
    pub default_folder: String,
    pub endings: String,
}

impl RustThumbnailTilerOptions {
    pub fn new() -> Self {
        RustThumbnailTilerOptions {
            default_folder: "~/Pictures".to_string(), // read from commandline param
            endings: ENDINGS.to_string(),
        }
    }
    pub fn parse(&mut self, mut toml_doc: String) {
        let parser = TOMLParser::new();
        let (mut parser, result) = parser.parse(&toml_doc);

        // parse options provide default value
        parser.get_value("table.SomeKey"); // gets "Some Value"
        parser.set_value("table.\"A Key\"", Value::float(9.876));
        parser.set_value("table.SomeKey", Value::bool(false));

        parser.get_value("");
    }
}

pub fn load_options_toml() -> RustThumbnailTilerOptions {
    let options_file_name: &str = &("~/".to_string() + OPTIONS_FILE_NAME);
    let mut options = RustThumbnailTilerOptions::new();
    match File::open(&Path::new(options_file_name)) {
        Err(e) => options,
        Ok(mut f) => {
            let mut buf = String::new();
            match f.read_to_string(&mut buf) {
                Err(e) => options,
                Ok(_) => {
                    options.parse(buf);
                    options
                }
            }
        }
    }
}
