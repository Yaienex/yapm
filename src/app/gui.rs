
use std::ffi::OsStr;

use gtk4::gio::File;
use gtk4::glib::property::PropertyGet;
use gtk4::glib::translate::FromGlibPtrBorrow;
use gtk4::glib::{clone, GString, RustClosure, Type, Value};
use gtk4::subclass::window;
use gtk4::{self as gtk, DrawingArea, DropTarget, FileChooser, FileDialog, Grid, GridLayout, IconView, Image, Label, ScrolledWindow, TextBuffer, TextView, Widget};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use poppler::PopplerDocument;

use super::del_box::del_box;
use super::get_box::get_box;
use super::merge_box::merge_box;
use super::reorg_box::reorg_box;
use super::split_box::split_box;


pub fn app(){
    let app = Application::builder()
        .application_id("org.yapm")
        .build();


    app.connect_activate(build_ui);

    
    let mut no_args : [&str; 0] = [];
    app.run_with_args (&mut no_args);();

}

/*
TODO
- Menu bar -> header bar 
- drag drop zone
- Preview
- right menu preview
- Home menu with the selection of the option 
*/

fn build_ui(app: &Application) {
    let margin = 20;
    let inter_spacing = 10;
    let app_wrapper = gtk4::Box::builder()
        .vexpand(true)
        .hexpand(true)
        .orientation(gtk4::Orientation::Vertical)
        .build();
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Yet Another PDF Merger")
        .child(&app_wrapper)
        .default_width(800)
        .default_height(400)
        .build();

    //Grid with the button tiles to choose which mode you'll choose
    let grid = Grid::builder()
        .margin_bottom(margin)
        .margin_top(margin)
        .margin_start(margin)
        .margin_end(margin)
        .hexpand(true)
        .vexpand(true)
        .column_homogeneous(true)
        .row_homogeneous(true)
        .row_spacing(inter_spacing)
        .column_spacing(inter_spacing)
        .build();

    app_wrapper.append(&grid);
    //Merge
    let merge = tile_button("Merge", "ressources/merge_icon.png",&window);
    //Split
    let split = tile_button("Split", "ressources/split_icon.png",&window);
    //Reorganize
    let reorganize = tile_button("Reorganize", "ressources/reorganize_icon.png",&window);
    //Delete pages
    let del_page = tile_button("Delete pages", "ressources/del_icon.png",&window);
    //Get page
    let get_page = tile_button("Get page", "ressources/get_icon.png",&window);

    //Attach the tile which are basically buttons
    grid.attach(&merge, 0, 0,1,1);
    grid.attach(&split, 0, 1,1,1);
    grid.attach(&reorganize, 1, 0,1,1);
    grid.attach(&del_page, 1, 1,1,1);
    grid.attach(&get_page,1,2,1,1,);

    let help_button = Button::builder().build();
    let home_button = Button::builder().build();
    let help_icon = Image::from_file("ressources/help_icon.png");
    let home_icon = Image::from_file("ressources/home_icon.png");
    home_button.set_child(Some(&home_icon));
    help_button.set_child(Some(&help_icon));
    let header_bar = gtk4::HeaderBar::builder().build();
    let e = gtk4::ActionBar::builder().build();
    e.set_revealed(true);
    
    header_bar.pack_start(&home_button);
    header_bar.pack_start(&help_button);

    window.set_titlebar(Some(&header_bar));
    window.present();

    home_button.connect_clicked(move |_e|{
        window.set_child(Some(&app_wrapper));
    });
    help_button.connect_clicked(|_e|{
        help_button_clicked();
    });

    
 
    
}

//We need to create a new window for the corresponding mode 
//But first let's do the Main Menu 
fn on_select(file : Result<gtk4::gio::ListModel, gtk4::glib::Error>){
    let filemodel = file.unwrap();
    for i in 0..filemodel.n_items(){
        let file  =    filemodel.item(i).unwrap().downcast::<File>().unwrap();
        let path = file.path().unwrap();
        if path.extension() == Some(OsStr::new("pdf")){
            println!("{:?} : ce fichier est bien un pdf",path);
        }
        else {
            println!("{:?} : ce fichier n'est pas un pdf",path);
        }

    }
    

}

fn help_button_clicked() {
    let help_wrapper = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();
    
    let text = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .build();
    let buff = text.buffer();
    //TODO THE DOC 
    buff.set_text("ca earea <\\span> aaaaaaa\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n ca");
   
    let scroll = ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&text)
        .build();

    help_wrapper.append(&scroll);


    let header_bar = gtk4::HeaderBar::builder().build();
    let window:ApplicationWindow= ApplicationWindow::builder()
        .title("Yet Another PDF Merger - Help")
        .titlebar(&header_bar)
        .child(&help_wrapper)
        .default_width(400)
        .default_height(400)
        .resizable(false)
        .destroy_with_parent(true)
        .visible(true)
        .build();
    println!("{:?}",window.parent());
    window.present();

}


fn tile_button(name: &str,image:&str, window: &ApplicationWindow) -> Button{
    let name_formatted = format!("<span font=\"16\">{name}</span>");
    let margin = 10;
    let window = window.clone();
    let button = Button::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
    let inside = gtk4::Box::builder()
        .valign(gtk4::Align::Fill)
        .halign(gtk4::Align::Fill)
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let label = Label::builder()
        .label(name_formatted)
        .use_markup(true)
        .halign(gtk4::Align::Fill)
        .build();
    let icon = Image::builder()
        .file(image)
        .margin_bottom(margin)
        .margin_end(margin)
        .margin_start(margin)
        .margin_top(margin)
        .vexpand(true)
        .hexpand(true)
        //.icon_size(gtk4::IconSize::Large)
        .build();

    inside.append(&icon);
    inside.append(&label);
    button.set_child(Some(&inside));
    
    let vname = name.to_owned();
    button.connect_clicked( move |_e|{
       
        //swap to the corresponding window
        let widget = get_widget(&vname,&window);
        window.set_child(Some(&widget));

    });


    button
}

fn get_widget(name: &String,window: &ApplicationWindow) -> gtk4::Box{
    match name.as_str(){
        "Merge" => merge_box(),
        "Split" => split_box(),
        "Reorganize" => reorg_box(), 
        "Get page" => get_box() ,
        "Delete pages" => del_box(),
        _ => warning_box(),
    }

}

fn warning_box() -> gtk4::Box{
    let boxe = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    boxe
}

/*
let ebutton = Button::from_icon_name("document-open");
//let file = FileDialog::builder().build();
 ebutton.connect_clicked( move |_button|{
        file.open_multiple(Some(&window), gtk4::gio::Cancellable::NONE, move |arg0: Result<gtk4::gio::ListModel, gtk4::glib::Error>| on_select(arg0));
    });
    
    file.set_title("Choose a PDF file");
    
    */