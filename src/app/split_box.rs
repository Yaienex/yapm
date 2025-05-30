use std::path::PathBuf;

use gtk4::{gio::{prelude::FileExt, File}, glib::object::Cast, prelude::{ButtonExt, GtkWindowExt, TextBufferExt, WidgetExt}, ApplicationWindow, Button, Label, ListBox, ListBoxRow, TextBuffer};
use lopdf::Document;

use super::{cli, widget_builder::{folder_window, row_file, widget_builder}};


pub fn split_box(window:&gtk4::ApplicationWindow) -> gtk4::Box{
    let (main_box,
        file_box,
        add_button,
        do_button,) = widget_builder("Split".to_string(),
                "/usr/share/yapm/ressources/split_icon.png".to_string(),
                false,
                true);

    let win = window.clone();

    let f_box = file_box.clone();
    add_button.connect_clicked( move |_e|{
        let file = gtk4::FileDialog::builder().title("Choose your pdf files").build();
        let f_box = f_box.clone();
        file.open_multiple(Some(&win), gtk4::gio::Cancellable::NONE,   |arg0: Result<gtk4::gio::ListModel, gtk4::glib::Error>| on_select(arg0,f_box));
    });

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
            let (faccept_button,path_content_buffer,fwin) = folder_window(b.clone(),".zip");
            accept_button_action(faccept_button, path_content_buffer, number, file_box.clone(), fwin);
        }
    
        
        
    });


    main_box
}




//Callbacks
//if multiple file -> Zip of zips
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
            name = String::from("zipped_doc.zip");
            flag = true;
        }else {//the normal case
            //println!("no : {:?}",file_name);
            if file_name.ends_with(".zip"){
                 name = file_name.to_owned();
            } else {
                name = format!("{file_name}.zip");
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

        let mut pdf_list:Vec<String> = Vec::new();
        for i in 0..number{
            //access the invisible label the absolute path
            let abs_path = file_box.row_at_index(i)
                .unwrap()
                .first_child()
                .unwrap()
                .first_child()
                .unwrap()
                .downcast::<Label>()
                .unwrap().label().to_string();
            pdf_list.push(abs_path);
            
        }
        
        //here
        pdf_list.insert(0, write_path);
        cli::split(&mut pdf_list, true);
        b.set_sensitive(true);
        win.close();
    });
}
