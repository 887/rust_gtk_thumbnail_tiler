extern crate gtk;
extern crate gdk;
extern crate image;
extern crate tomllib;
extern crate threadpool;

//use threadpool::ThreadPool;
use gtk::prelude::*;
use gtk::{Window, Button, Builder,MenuBar, Label, Fixed, PlacesSidebar};
//use gtk::{Image, EventBox, };
use std::path::{PathBuf};
//use std::fs::File;
use std::ffi::OsStr;
use std::fs;
use gdk::enums::key;
//use gdk::Event;
use image::*;
//use std::env;
//use std::sync::{Arc, Mutex};
//use std::sync::mpsc;

mod options;

use options::*;

//let pool = ThreadPool::new(8);

//pool.execute(move || {
//count_frequency_for_word(&sequence, &mut worker_hash_map.lock().unwrap());
//match tx.send(current_index) {
//Err(mpsc::SendError(index)) => { println!("Error on worker {}: {}", index, sequence) },
//_ => {}
//}

fn get_images(path: PathBuf, endings: Vec<&str>) -> Vec<PathBuf> {
    println!("getting images from {} \n", path.to_str().unwrap());
    let paths = fs::read_dir(path).unwrap();
    paths.fold(Vec::<PathBuf>::new(), |mut images, rde| {
        // Result<DirEntry>
        match rde {
            Ok(de) => {
                let path: PathBuf = de.path();
                println!("Name: {}", de.path().display());
                if path.is_file() {
                    println!("..is a file.."); 
                    let extension: &OsStr = path.extension().unwrap();
                    let extension_str: &str = extension.to_str().unwrap();
                    if endings.iter().any(|e| extension_str.ends_with(e)) { 
                        println!("..and has correct extenson"); 
                        images.push(path.clone());
                    }
                }
            }
            Err(_) => {}
        }
        images
    })
}

//http://bit.ly/29Jcf0A
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let options = load_options_toml();

    let endings: Vec<&str> = options.endings.split(";").collect();
    //let file_endings = endings.iter().fold(String::new(),|mut res, e|{
        //res = res + e;
        //res
    //});
    let file_endings = endings.join(",");
    println!("allowed endings: {}", file_endings);

    //get image sin seperate thread and join them later through a
    // todo: read from commandline param
    let images: Vec<PathBuf> = get_images(options.default_folder_path, endings);
    println!("found {} images!", images.len());
    for pb in images {

        let ir = image::open(pb.clone());
        match ir { //ImageResult<DynamicImage>
            Ok(di) => {
                let (width, height) = di.dimensions();
                let str_pb = pb.to_str().unwrap();
                println!("found image: {}, width: {}, height: {}", str_pb, width, height);
            }
            _ => {}
        }
    }
    //let mut handles = Vec::<JoinHandle<HashMap<char, usize>>>::new();


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

     //places_sidebar.connect_event(move |e|{
     //});
     main_window.set_title(".");

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
