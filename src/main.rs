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
use std::env;

mod options;

use options::*;

// example: https://github.com/gtk-rs/examples/blob/master/src/simple_treeview.rs

fn get_images(path: PathBuf, endings: Vec<&str>) -> Vec<DynamicImage> {
    println!("getting images from {} \n", path.to_str().unwrap());
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

    // todo: read from commandline param
    let images = get_images(options.default_folder_path, endings);

    let builder = Builder::new_from_string(include_str!("./ui.glade"));

    let main_window: Window = builder.get_object("main_window").unwrap();
    let otions_windows: Window = builder.get_object("options_window").unwrap();
    let main_menu_bar: MenuBar = builder.get_object("main_menu_bar").unwrap();
    let places_sidebar: PlacesSidebar = builder.get_object("main_places_sidebar").unwrap();

    // places_sidebar.connect(gtk::signals::GtkPlacesSidebar::open::new(||{
    // if e.get_event_type() == PlacesSidebar::open_location {
    // }
    // gtk::Inhibit(false)
    // });

    // places_sidebar.connect_event(move |e|{
    // });
    // main_window.set_title(".");

    let button_cancel: Button = builder.get_object("options_button_cancel").unwrap();
    let button_ok: Button = builder.get_object("options_button_ok").unwrap();
    let label_test: Label = builder.get_object("label_status").unwrap();
    let main_fixed: Fixed = builder.get_object("main_fixed").unwrap();

    // for i in image { main_fixed.add }
    //
    // mainfirefxed.on_resize ? main_window.onresize?

    //main_fixed.connect_size_allocate(move |_ , _| {

    //});

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
