use std::{path::PathBuf, process::exit};

use gtk4::{  gio::{prelude::{FileExt, InputStreamExt, InputStreamExtManual}, File}, glib::{object::Cast, GString}, prelude::{ButtonExt, GtkWindowExt, TextBufferExt, WidgetExt}, ApplicationWindow, Button, Label, ListBox, ListBoxRow, TextBuffer};
use lopdf::Document;

use super::{cli, widget_builder::{ folder_window, row_file, widget_builder}};


//Move widget on the main box
pub fn merge_box(window:&ApplicationWindow) -> gtk4::Box{
     let (main_box,
        file_box,
        add_button,
        do_button,) = widget_builder("Merge".to_string(),
                "/usr/share/yapm/ressources/merge_icon.png".to_string(),
                true,
                    true);
    
    let fbl = file_box.clone();
    do_button.connect_clicked(move |b|{
        let mut number = 0;
        //let file = FileDialog::builder().title("Choose your saving location").build();

        let mut row: Option<ListBoxRow> = file_box.row_at_index(number);
        while row.is_some() {
            number +=1;
            row = file_box.row_at_index(number );
        } 
        

        if number != 0{
            //should use file.save but it ain't working
            let (accept_button_fwin,path_content_buffer,fwin) = folder_window(b.clone(),".pdf");
            accept_button_action(accept_button_fwin, path_content_buffer, number, file_box.clone(), fwin);
        }
    
        
        
    });
    
    let win = window.clone();
    add_button.connect_clicked( move |_e|{
        let file = gtk4::FileDialog::builder().title("Choose your pdf files").build();
        let f_box = fbl.clone();
        file.open_multiple(Some(&win), gtk4::gio::Cancellable::NONE,   |arg0: Result<gtk4::gio::ListModel, gtk4::glib::Error>| on_select(arg0,f_box));
    });
    
   


    main_box
}


//Overrides
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
            file_box.select_row(Some(&row));
        }
    }
}

fn accept_button_action(button:Button,path_content_buffer:TextBuffer,number:i32,file_box: ListBox,win:ApplicationWindow){
    button.connect_clicked(move |b|{
        b.set_sensitive(false);
        let path = path_content_buffer.text(&path_content_buffer.start_iter(), &path_content_buffer.end_iter(), true);
        //dichotomy if path is a dir or not
        let mut splitted_path:Vec<&str>= path.split("/").collect();
        let name: String;
        let mut flag:bool = false;
        let file_name = splitted_path.remove(splitted_path.len() -1);
        let path_dir:PathBuf ={
            let tmp = splitted_path.join("/");
            if tmp.is_empty() {
                PathBuf::from("/")
            } else{
                if tmp.contains("\n"){
                    return;
                }
                tmp.into()
            }
        };

        let path_buf:PathBuf = path.clone().into();
        if !(path_dir.exists() || path_buf.exists()) || !path.starts_with("/") || file_name.is_empty(){ // the case where the user enter but the path is not valid
           return ;
        } else if path_buf.is_dir(){
            //println!("we are on default mode");
            name = String::from("merged.pdf");
            flag = true;
        }else {//the normal case
            //println!("no : {:?}",file_name);
            if file_name.ends_with(".pdf"){
                 name = file_name.to_owned();
            } else {
                name = format!("{file_name}.pdf");
            }
        }

        let write_path: String = {
            let pds = path_dir.to_str().unwrap();
            let pbs = path_buf.to_str().unwrap();
            if flag{
                format!("{}/{name}",pbs)
            } else {
                if pds == "/"{
                    format!("{}{name}",pds)
                }else {
                    format!("{}/{name}",pds)
                }
                
            }

        };

        let mut pdf_list:Vec<Document> = Vec::new();
        for i in 0..number{
            //access the invisible label the absolute path
            let abs_path = file_box.row_at_index(i)
                .unwrap()
                .first_child()
                .unwrap()
                .first_child()
                .unwrap()
                .downcast::<Label>()
                .unwrap().label();
            let doc = lopdf::Document::load(abs_path).unwrap();
            pdf_list.push(doc);
            
        }
        
        //here
        cli::merge(pdf_list, &write_path);
        b.set_sensitive(true);
        win.close();
    });
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