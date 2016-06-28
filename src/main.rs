extern crate gtk;
extern crate gdk;
extern crate image;

use gtk::prelude::*;
use gtk::{Window, Button, Builder, WindowType, Label, Image, Fixed, EventBox};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::*;
use gdk::enums::key;
use image::{DynamicImage};
//use std::io::*;

//should not be static but a parsed vec
static ENDINGS: &'static str = "jpg;png;gif;tiff;bmp;jpg-large;jpeg";

fn get_images(path: &str, endings: Vec<&str>) -> Vec<DynamicImage> {
    //ensure path is a file? traitbound error
    let paths = read_dir(path).unwrap();
    paths.fold(Vec::<DynamicImage>::new(), |mut images, rde| { //Result<DirEntry>
        match rde {
            Ok(de) => {
                let path: PathBuf = de.path();
                println!("Name: {}", de.path().display());
                if path.is_file() && endings.iter().any(|e| path.ends_with(e))  {
                    let ir = image::open(de.path());
                    match ir { //ImageResult<DynamicImage>
                        Ok(dr) => {
                            images.push(dr);
                        }
                        _ => {}
                    }
                }
            }
            Err(_) => {}
        }
        images
        //todo get dimensions from image?
    })
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let endings: Vec<&str> = ENDINGS.split(";").collect();

    let images = get_images("./", endings);
    // todo
    // let default_folder = env::

    let builder = Builder::new_from_string(include_str!("./TestApp.glade"));

    let window: Window = builder.get_object("main").unwrap();
    window.set_title("First GTK+ Program");

    let button_cancel: Button = builder.get_object("button_cancel").unwrap();
    let button_ok: Button = builder.get_object("button_ok").unwrap();
    let label_test: Label = builder.get_object("label_status").unwrap();
    let main_fixed: Fixed = builder.get_object("main_fixed").unwrap();

    //for i in image { main_fixed.add }
    //
    //mainfirefxed.on_resize ? window.onresize?

    button_ok.connect_clicked(move |_| {
        label_test.set_text("test");
        println!("Clicked!");
    });

    button_cancel.connect_clicked(move |_| {
        std::process::exit(0);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() { gtk::main_quit() }
        gtk::Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
