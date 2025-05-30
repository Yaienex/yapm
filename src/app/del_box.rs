use super::widget_builder::{row_file, widget_builder};
use gtk4::{gio::File, prelude::{ButtonExt,FileExt}, ListBox};
use poppler::Document;

pub fn del_box(window:&gtk4::ApplicationWindow) -> gtk4::Box{
    let (main_box,
        file_box,
        add_button,
        do_button) = widget_builder("Delete page(s)".to_string(),
                "/usr/share/yapm/ressources/del_icon.png".to_string(),
                false,
                false,
                true);

    let win = window.clone();

    //Connect buttons to right actions 
    add_button.connect_clicked( move |_e|{
        let file = gtk4::FileDialog::builder().title("Choose your pdf file").build();
        let f_box = file_box.clone();
        file.open(Some(&win), gtk4::gio::Cancellable::NONE,   |arg0: Result<File, gtk4::glib::Error>| on_select(arg0,f_box));
    });



    main_box
}


//Callbacks
//like get but save in one pdf  after remove
fn on_select(arg:Result<File,gtk4::glib::Error>,file_box:ListBox){
    if !arg.is_err(){
        file_box.remove_all();
        let file = &arg.unwrap();
        let path = file.path().unwrap();
        //Ignoring all the format except .pdf
        if ! &path.to_str().unwrap().ends_with(".pdf") { return;}

        //reprensenting every page by a row 
        //Appending the list
        let document = Document::from_file(&format!("file://{}",path.to_str().unwrap()), Some("")).unwrap();
        let n = document.n_pages();
        for i in 1..=n{
            let row = row_file(path.clone(),&format!("Page_{i}"),true);
            file_box.append(&row);
        }
        
        }
}