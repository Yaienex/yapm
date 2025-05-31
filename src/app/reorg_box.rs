use gtk4::{gio::File, prelude::ButtonExt};
use super::widget_builder::{ on_select_pages, widget_builder};

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
    
    add_button.connect_clicked( move |_e|{
        let file = gtk4::FileDialog::builder().title("Choose your pdf files").build();
        let f_box = file_box.clone();
        file.open(Some(&win), gtk4::gio::Cancellable::NONE,   move |arg0: Result<File, gtk4::glib::Error>| on_select_pages(arg0,f_box,select_flag,check_visible));
    });


    main_box

}


//Callbacks
