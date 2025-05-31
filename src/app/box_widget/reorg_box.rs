use std::path::PathBuf;

use super::super::widget_builder::{folder_window, on_select_pages, widget_builder};
use gtk4::{gio::File, prelude::{ButtonExt, TextBufferExt, WidgetExt}, ApplicationWindow, Button, ListBox, ListBoxRow, TextBuffer};

pub fn reorg_box(window:&gtk4::ApplicationWindow) -> gtk4::Box{

    let move_flag = true;
    let pdf_view_flag = false;
    let select_flag = true;
    let del_flag = false;
    let check_visible = false;

    let (main_box,
        file_box,
        add_button,
        do_button,) = widget_builder("Reorganize".to_string(),
                "/usr/share/yapm/ressources/reorganize_icon.png".to_string(),
                move_flag,
                pdf_view_flag,
                select_flag,
                del_flag);

    let win = window.clone();
    let fb = file_box.clone();
    add_button.connect_clicked( move |_e|{
        let file = gtk4::FileDialog::builder().title("Choose your pdf files").build();
        let f_box = file_box.clone();
        file.open(Some(&win), gtk4::gio::Cancellable::NONE,   move |arg0: Result<File, gtk4::glib::Error>| on_select_pages(arg0,f_box,select_flag,check_visible));
    });


    let win = window.clone();
    let file_box = fb.clone();
    do_button.connect_clicked(move |b|{
        let window = win.clone();
        let mut number = 0;
        //let file = FileDialog::builder().title("Choose your saving location").build();

        let mut row: Option<ListBoxRow> = file_box.row_at_index(number);
        while row.is_some() {
            number +=1;
            row = file_box.row_at_index(number );
        } 
        

        if number != 0{
            //should use file.save but it ain't working
            let (accept_button_fwin,path_content_buffer,fwin) = folder_window(b.clone(),".pdf",false);
            accept_button_action(accept_button_fwin, path_content_buffer, number, file_box.clone(), fwin,window);
        
        }
    
        
        
    });
    main_box

}


//Callbacks


fn accept_button_action(button:Button,path_content_buffer:TextBuffer,number:i32,file_box: ListBox,win:ApplicationWindow,main_win:ApplicationWindow){
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

});}
