use std::{path::PathBuf, process::exit};

use gtk4::{ gdk::{self, Display, DragAction}, gio::{prelude::{FileExt, InputStreamExt, InputStreamExtManual}, File, Icon, Menu, MenuItem}, glib::{object::{Cast, ObjectExt}, property::PropertyGet, GString, SignalHandlerId, Value}, prelude::{BoxExt, ButtonExt, ListBoxRowExt, WidgetExt}, ActionBar, ApplicationWindow, Button, CssProvider, DropTarget, DropTargetAsync, FileDialog, GestureClick, Image, Label, ListBox, ListBoxRow};




//Move widget on the main box
pub fn merge_box(window:&ApplicationWindow) -> gtk4::Box{

    //constant
    let margin = 5;
    
    //Main widget
    let main_box = gtk4::Box::builder()
        .margin_bottom(margin*3)
        .margin_end(margin*3)
        .margin_start(margin*3)
        .margin_top(margin*3)
        .orientation(gtk4::Orientation::Horizontal)
        .hexpand(true)
        .build();

    let decision_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_bottom(margin)
        .margin_end(margin)
        .margin_start(margin)
        .margin_top(margin)
        .name("decision-box")
        .halign(gtk4::Align::End)
        .build();

    //Drop zone config 
    let drop_box_controller = DropTarget::new(
        <gtk4::gio::File as gdk::glib::types::StaticType>::static_type(),  
        gdk::DragAction::COPY
    );
    drop_box_controller.connect_accept(|_, drop| {
        println!("{}",drop.formats().to_str());
        return drop.formats().contain_mime_type("text/uri-list") ||drop.formats().contain_mime_type("GFile")  ;
    });
    drop_box_controller.connect_drop( move |_widget, value, _x, _y| {
        true
    });
    let drop_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .name("drop-box")
        .build();
    //drop_box.add_controller(drop_box_controller); Delaying this to focus on a working application 
    
    //Manage pdf file from the UI
    let file_box = gtk4::ListBox::builder()
        .name("file-box")
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .valign(gtk4::Align::Fill)
        .vexpand(true)
        .build();
    
    
    //TO remove
    let label = Label::new(Some("aerer"));

    let row = ListBoxRow::builder()
        .name("row")
        .child(&label)
        .build();
    file_box.append(&row);
    drop_box.append(&file_box);

    let manage_box = gtk4::Box::builder()
        .name("manage-box")
        .halign(gtk4::Align::Fill)
        .vexpand(false)
        .valign(gtk4::Align::Fill)
        .hexpand(true)
        .orientation(gtk4::Orientation::Horizontal)
        .build();
   
    //Action bar 
    let action_bar = ActionBar::builder()
        .halign(gtk4::Align::Fill)
        .vexpand(false)
        .valign(gtk4::Align::Fill)
        .hexpand(true)
        .build();

    
    let add_icon = Image::from_file("ressources/plus_icon.png");
    let del_icon = Image::from_file("ressources/del_icon.png");
    let add_button = Button::builder()
        .child(&add_icon)
        .name("add-button")
        .build(); 
    let del_button = Button::builder()
        .child(&del_icon)
        .name("del-button")
        .build();
    let win = window.clone().to_owned();

     let f_box = file_box.clone();
    add_button.connect_clicked(move |_e|{
        let file = FileDialog::builder().title("Choose your pdf files").build();
        let f_box = file_box.clone();
        file.open_multiple(Some(&win), gtk4::gio::Cancellable::NONE,  move |arg0: Result<gtk4::gio::ListModel, gtk4::glib::Error>| on_select(arg0,f_box));
  
    });
   
    del_button.connect_clicked(move |_e|{
        f_box.remove(&f_box.selected_row().unwrap());
    });


    action_bar.pack_start(&add_button);
    action_bar.pack_end(&del_button);

    manage_box.append(&action_bar);


    drop_box.append(&manage_box);

    //The action button on the bottom right
    let do_button = do_button();
   
    //Setting up everything 
    decision_box.append(&do_button);

    main_box.append(&drop_box);
    main_box.append(&decision_box); 
    
    main_box
}

fn on_select(arg :Result<gtk4::gio::ListModel, gtk4::glib::Error>,file_box:ListBox){
    if !arg.is_err(){
        let listmodel = &arg.unwrap();
        let mut label = Label::builder().build();
        for object in listmodel{
            let path = object.unwrap().downcast::<File>().unwrap().path().unwrap();
            let splitted_path:Vec<&str>= path.to_str().unwrap().split("/").collect();
            let name = splitted_path[splitted_path.len() -1 ];
            //Ignoring all the format except .pdf
            if ! name.contains(".pdf") { continue;}
            println!("{:?}",name );

            //Appending the list
            label = Label::new(Some(name));
            let row = ListBoxRow::builder()
                .name("row")
                .build();
            row.set_child(Some(&label));
            file_box.append(&row);
        }
        
    }
}

fn do_button() -> Button{
    let do_button =Button::builder()
        .valign(gtk4::Align::End)
        .vexpand(true)
        .name("do-button")
        .build();
    let do_button_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();
    let do_button_label = Label::new(Some("Merge"));
    let do_button_icon = Image::from_file("ressources/merge_icon.png");
    do_button_icon.set_icon_size(gtk4::IconSize::Large);


    do_button_box.append(&do_button_icon);
    do_button_box.append(&do_button_label);

    do_button.set_child(Some(&do_button_box));
    do_button
}



fn _drop_handler(e : Result<(gtk4::gio::InputStream, GString), gtk4::glib::Error>){
    let (c,_uri_type) = e.unwrap();
    let buffer: [u8;64496] = [0;64496];
    match c.read_all(buffer,gtk4::gio::Cancellable::NONE){
        Ok(ok) => ok,
        Err(er) => {println!("{}",er); exit(0);}
    };
    println!("Buffer {:?}",buffer);
    let _ =c.close(gtk4::gio::Cancellable::NONE);
}