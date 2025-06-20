
use gtk4::gdk::Key;
use gtk4::glib::Propagation;
use gtk4::{self as gtk, EventControllerKey, Grid, Image, Label, ScrolledWindow, TextView};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};


use super::box_widget::del_box::del_box;
use super::box_widget::get_box::get_box;
use super::box_widget::merge_box::merge_box;
use super::box_widget::reorg_box::reorg_box;
use super::box_widget::split_box::split_box;


pub fn app(){
    let app = Application::builder()
        .application_id("org.yaienex.yapm")
        .build();
    app.connect_activate(build_ui);
    let mut no_args : [&str; 0] = [];
    app.run_with_args (&mut no_args);();
}


fn build_ui(app :&Application) {
    style( false);
    let margin = 20;
    let inter_spacing = 10;
    let app_wrapper = gtk4::Box::builder()
        .name("app-wrapper")
        .vexpand(true)
        .hexpand(true)
        .orientation(gtk4::Orientation::Vertical)
        .build();
    let window = ApplicationWindow::builder()
        .application(app)
        .name("main-window")
        .title("Yet Another PDF Merger")
        .child(&app_wrapper)
        .resizable(false)
        .default_width(700)
        .default_height(600)
        .build();


    //Grid with the button tiles to choose which mode you'll choose
    let grid = Grid::builder()
        .name("grid")
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
    let merge = tile_button("Merge", "/usr/share/yapm/ressources/merge_icon.png",&window);
    //Split
    let split = tile_button("Split", "/usr/share/yapm/ressources/split_icon.png",&window);
    //Reorganize
    let reorganize = tile_button("Reorganize", "/usr/share/yapm/ressources/reorganize_icon.png",&window);
    //Delete pages
    let del_page = tile_button("Delete pages", "/usr/share/yapm/ressources/del_icon.png",&window);
    //Get page
    let get_page = tile_button("Get page", "/usr/share/yapm/ressources/get_icon.png",&window);

    //Attach the tile which are basically buttons
    grid.attach(&merge, 0, 0,1,1);
    grid.attach(&split, 0, 1,1,1);
    grid.attach(&reorganize, 1, 0,1,1);
    grid.attach(&del_page, 1, 1,1,1);
    grid.attach(&get_page,1,2,1,1,);

    //TO opti
    let help_button = Button::builder().name("button").build();
    let home_button = Button::builder().name("button").build();
    let mode_button = Button::builder().name("button").build();
    let help_icon = Image::from_file("/usr/share/yapm/ressources/help_icon.png");
    let home_icon = Image::from_file("/usr/share/yapm/ressources/home_icon.png");
    let mode_icon = Image::builder()
        .file("/usr/share/yapm/ressources/theme_mode_icon.png")
        .icon_size(gtk4::IconSize::Large)
        .build();
    
    home_button.set_child(Some(&home_icon));
    mode_button.set_child(Some(&mode_icon));
    help_button.set_child(Some(&help_icon));
    let header_bar = gtk4::HeaderBar::builder().name("header-bar").build();
    header_bar.pack_start(&home_button);
    header_bar.pack_start(&mode_button);
    header_bar.pack_start(&help_button);


    let controller = EventControllerKey::new();
    let hb = home_button.clone();
    controller.connect_key_pressed(move |_, key, _, _| {
        if key == Key::Escape {
            home_button.emit_clicked();
            Propagation::Proceed
        } else {
            Propagation::Proceed
        }
    });
    window.add_controller(controller);
    window.set_titlebar(Some(&header_bar));
    window.present();
    hb.connect_clicked(move |_e|{
        window.set_child(Some(&app_wrapper));
    });
    help_button.connect_clicked(|_e|{
        help_button_clicked();
    });
    mode_button.connect_clicked( move |button| {
        let flag = button.widget_name();
        if flag == "button" {
            style(true);
            button.set_widget_name("nbutton");
        } else if flag == "nbutton" {
            style(false);
            button.set_widget_name("button");
        }
    });

    
}

//TO COMPLETE
fn help_button_clicked() {
    let help_wrapper = gtk4::Box::builder()
        .name("app-wrapper")
        .orientation(gtk4::Orientation::Vertical)
        .build();
    
    let text = TextView::builder()
        .editable(false)
        .name("button")
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


    let header_bar = gtk4::HeaderBar::builder().name("header-bar").build();
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
    window.present();

}


fn tile_button(name: &str,image:&str, window: &ApplicationWindow) -> Button{
    let name_formatted = format!("<span font=\"16\">{name}</span>");
    let margin = 10;
    let window = window.clone();
    let button = Button::builder()
        .name("tile-button")
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
        "Merge" => merge_box(window),
        "Split" => split_box(window),
        "Reorganize" => reorg_box(window), 
        "Get page" => get_box(window) ,
        "Delete pages" => del_box(window),
        _ => gtk4::Box::NONE.unwrap().clone(),
    }
}

pub fn style(night_mode : bool){
    let provider = gtk4::CssProvider::new();
    if night_mode{
        let file = gtk4::gio::File::for_path("/usr/share/yapm/ressources/night_style.css");
        provider.load_from_file(&file);
    } else{
        let file = gtk4::gio::File::for_path("/usr/share/yapm/ressources/day_style.css");
        provider.load_from_file(&file);
    }

    let display = gtk4::gdk::Display::default().expect("Could not connect to a display.");
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );


}