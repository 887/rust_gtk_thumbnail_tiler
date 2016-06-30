extern crate gtk;
extern crate gdk;
extern crate image;
extern crate tomllib;

use gtk::prelude::*;
use gtk::{Window, Button, Builder, WindowType, Label, Image, Fixed, EventBox, MenuBar,
          PlacesSidebar};
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use gdk::enums::key;
use gdk::Event;
use image::DynamicImage;
use std::io::*;
use tomllib::TOMLParser;
use tomllib::types::Value;

//example: https://github.com/gtk-rs/examples/blob/master/src/simple_treeview.rs

static ENDINGS: &'static str = "jpg;png;gif;tiff;bmp;jpg-large;jpeg";
static OPTIONS_FILE_NAME: &'static str = ".rust_gtk_thumbnailer.conf.toml";

struct RustThumbnailTilerOptions {
    default_folder: String,
    endings: String,
}

impl RustThumbnailTilerOptions {
    fn new() -> Self {
        RustThumbnailTilerOptions {
            default_folder: "~/Pictures".to_string(), // read from commandline param
            endings: ENDINGS.to_string(),
        }
    }
    fn parse(&mut self, mut toml_doc: String) {
        let parser = TOMLParser::new();
        let (mut parser, result) = parser.parse(&toml_doc);
 
        //parse options provide default value
        parser.get_value("table.SomeKey"); // gets "Some Value"
        parser.set_value("table.\"A Key\"", Value::float(9.876));
        parser.set_value("table.SomeKey", Value::bool(false));
            
        parser.get_value("");
    }
}

// toml parser?!?
fn load_options_toml() -> RustThumbnailTilerOptions {
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

// ensure path is a file? traitbound error
fn get_images(path: &str, endings: Vec<&str>) -> Vec<DynamicImage> {
    let paths = fs::read_dir(path).unwrap();
    paths.fold(Vec::<DynamicImage>::new(), |mut images, rde| {
        // Result<DirEntry>
        match rde {
            Ok(de) => {
                let path: PathBuf = de.path();
                println!("Name: {}", de.path().display());
                if path.is_file() && endings.iter().any(|e| path.ends_with(e)) {
                    let ir = image::open(de.path());
                    match ir { //ImageResult<DynamicImage>
                        Ok(dr) => {
                            // todo get dimensions from image?
                            images.push(dr);
                        }
                        _ => {}
                    }
                }
            }
            Err(_) => {}
        }
        images
    })
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let options = load_options_toml();

    let endings: Vec<&str> = options.endings.split(";").collect();
    let images = get_images(&options.default_folder, endings);

    let builder = Builder::new_from_string(include_str!("./ui.glade"));

    let main_window: Window = builder.get_object("main_window").unwrap();
    let otions_windows: Window = builder.get_object("options_window").unwrap();
    let main_menu_bar: MenuBar = builder.get_object("main_menu_bar").unwrap();
    let places_sidebar: PlacesSidebar = builder.get_object("main_places_sidebar").unwrap();

    //places_sidebar.connect(gtk::signals::GtkPlacesSidebar::open::new(||{
        //if e.get_event_type() == PlacesSidebar::open_location {
        //}
        //gtk::Inhibit(false)
    //});

    //places_sidebar.connect_event(move |e|{
    //});
    // main_window.set_title("First GTK+ Program");

    let button_cancel: Button = builder.get_object("options_button_cancel").unwrap();
    let button_ok: Button = builder.get_object("options_button_ok").unwrap();
    let label_test: Label = builder.get_object("label_status").unwrap();
    let main_fixed: Fixed = builder.get_object("main_fixed").unwrap();

    // for i in image { main_fixed.add }
    //
    // mainfirefxed.on_resize ? main_window.onresize?

    button_ok.connect_clicked(move |_| {
        label_test.set_text("test");
        println!("Clicked!");
    });

    button_cancel.connect_clicked(move |_| {
        std::process::exit(0);
    });

    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    main_window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        gtk::Inhibit(false)
    });

    main_window.show_all();

    gtk::main();
}
