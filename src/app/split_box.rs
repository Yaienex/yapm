use gtk4::{gio::{prelude::FileExt, File}, glib::object::Cast, prelude::ButtonExt, ListBox};

use super::widget_builder::{widget_builder,row_file};


pub fn split_box(window:&gtk4::ApplicationWindow) -> gtk4::Box{
    let (main_box,
        file_box,
        add_button,
        do_button,) = widget_builder("Split".to_string(),
                "/usr/share/yapm/ressources/split_icon.png".to_string(),
                false,
                true);

    let win = window.clone();

    add_button.connect_clicked( move |_e|{
        let file = gtk4::FileDialog::builder().title("Choose your pdf files").build();
        let f_box = file_box.clone();
        file.open_multiple(Some(&win), gtk4::gio::Cancellable::NONE,   |arg0: Result<gtk4::gio::ListModel, gtk4::glib::Error>| on_select(arg0,f_box));
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