extern crate gtk;
extern crate gdk;
extern crate image;

use gtk::prelude::*;
use gtk::{Window, Button, Builder, WindowType, Label, Image, Fixed, EventBox};
use std::path::Path;
use std::ffi::OsStr;
use std::fs;
use gdk::enums::key;

static ALPHABET: &'static str = "jpg;png;gif;tiff;bmp;jpg-large;jpeg";

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    //ensure path is a file? traitbound error 
    let paths: fs::ReadDir = fs::read_dir("./").unwrap();
    let mut images = Vec::<image::DynamicImage>::new();
    for rde in paths { //Result<DirEntry>
        match rde {
            Ok(de) => {
                let path = de.path();
                println!("Name: {}", de.path().display());
                if path.is_file() {
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
        //todo get dimensions from image?

    }

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
