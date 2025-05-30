use super::widget_builder::{row_file, widget_builder};
use gtk4::{gio::File, prelude::ButtonExt, ListBox};
use gtk4::prelude::FileExt;
pub fn get_box(window:&gtk4::ApplicationWindow) -> gtk4::Box{
    let (main_box,
        file_box,
        add_button,
        do_button) = widget_builder("Get page(s)".to_string(),
                "/usr/share/yapm/ressources/get_icon.png".to_string(),
                false,
                false,
                true);

    let win = window.clone();

    add_button.connect_clicked( move |_e|{
        let file = gtk4::FileDialog::builder().title("Choose your pdf files").build();
        let f_box = file_box.clone();
        file.open(Some(&win), gtk4::gio::Cancellable::NONE,   |arg0: Result<File, gtk4::glib::Error>| on_select(arg0,f_box));
    });

    main_box
}





//Callbacks
//each page -> wich to extract |save to single pdf or multiple ?
fn on_select(arg:Result<File,gtk4::glib::Error>,file_box:ListBox){
    if !arg.is_err(){
        file_box.remove_all();
        let file = &arg.unwrap();
        let path = file.path().unwrap();
        let p = path.clone();
        let splitted_path:Vec<&str>= p.to_str().unwrap().split("/").collect();
        let name = splitted_path[splitted_path.len() -1 ];
        //Ignoring all the format except .pdf
        if ! name.contains(".pdf") { return;}
        //Appending the list
        let row = row_file(path,name,true);
        file_box.append(&row);
        }
}