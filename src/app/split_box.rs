use gtk4::{gio::{prelude::FileExt, File}, glib::object::Cast, prelude::ButtonExt, ListBox, ListBoxRow};

use super::widget_builder::{folder_window, row_file, widget_builder};


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
            folder_window(b.clone(),number);
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
        }
    }
}