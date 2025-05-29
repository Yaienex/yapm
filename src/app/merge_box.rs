use std::process::exit;

use gtk4::{  gio::{prelude::{FileExt, InputStreamExt, InputStreamExtManual}, File}, glib::{object::Cast, GString}, prelude::{ BoxExt, ButtonExt, WidgetExt}, ApplicationWindow, Label, ListBox, ListBoxRow};
use lopdf::Document;

use super::widget_builder::{ folder_window, row_file, widget_builder};


//Move widget on the main box
pub fn merge_box(window:&ApplicationWindow) -> gtk4::Box{
     let (main_box,
        file_box,
        add_button,
        do_button,) = widget_builder("Merge".to_string(),
                "/usr/share/yapm/ressources/merge_icon.png".to_string(),
                true,
                    true);
    
    let dbt = do_button.clone();
    let fbl = file_box.clone();
    do_button.connect_clicked(move |_e|{
        let b =dbt.clone();
        let fb = file_box.clone();
        let mut number = 0;
        //let file = FileDialog::builder().title("Choose your saving location").build();

        let mut row: Option<ListBoxRow> = file_box.row_at_index(number);
        while row.is_some() {
            number +=1;
            row = file_box.row_at_index(number );
        } 
        

        if number != 0{
            //should use file.save but it ain't working
            folder_window(b,fb,number);
        }
    
        
        
    });
    
    let win = window.clone();
    let f_box = fbl.clone();
    add_button.connect_clicked( move |_e|{
        let file = gtk4::FileDialog::builder().title("Choose your pdf files").build();
        let f_box = fbl.clone();
        file.open_multiple(Some(&win), gtk4::gio::Cancellable::NONE,   |arg0: Result<gtk4::gio::ListModel, gtk4::glib::Error>| on_select(arg0,f_box));
    });
    
   


    main_box
}


//Callbacks 
fn on_select(arg :Result<gtk4::gio::ListModel, gtk4::glib::Error>,file_box:ListBox){
    if !arg.is_err(){
        let listmodel = &arg.unwrap();
        for object in listmodel{
            let path = object.unwrap().downcast::<File>().unwrap().path().unwrap();
            let p = path.clone();
            let splitted_path:Vec<&str>= p.to_str().unwrap().split("/").collect();
            let name = splitted_path[splitted_path.len() -1 ];
            //Ignoring all the format except .pdf
            if ! name.contains(".pdf") { continue;}

            //Appending the list
            let row = row_file(path,name);
            file_box.append(&row);
        }
    }
}

//TEST DROP ZONE 

fn _on_save(arg : Result<File, gtk4::gdk::glib::Error>,_pdf_list :Vec<Document>){
    if !arg.is_err(){
        let file = arg.unwrap();
        //let tmp = file.path().unwrap();
        //let path = tmp.to_str().unwrap();
        println!("{:?}",file);
    }
    else {
        //do a popup to explain error 
    }
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



/*//Drop zone config 
    let drop_box_controller = DropTarget::new(
        <gtk4::gio::File as gdk::glib::types::StaticType>::static_type(),  
        gdk::DragAction::COPY
    );
    drop_box_controller.connect_accept(|_, drop| {
        println!("{}",drop.formats().to_str());
        return drop.formats().contain_mime_type("text/uri-list") ||drop.formats().contain_mime_type("GFile")  ;
    });
    drop_box_controller.connect_drop( move |_widget, _value, _x, _y| {
        true
    }); */