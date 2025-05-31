use super::super::{cli, result_window::{done_window, warning_window}, widget_builder::{folder_window, on_select_pages, widget_builder}};
use gtk4::{gio::File, glib::object::{Cast, CastNone}, prelude::{ButtonExt, CheckButtonExt, GtkWindowExt, TextBufferExt, WidgetExt}, ApplicationWindow, Button, CheckButton, Label, ListBox, ListBoxRow, TextBuffer, Widget};


pub fn get_box(window:&gtk4::ApplicationWindow) -> gtk4::Box{


    let move_flag = false;
    let pdf_view_flag = false;
    let select_flag = true;
    let del_flag = false;
    let check_visible = true;

    let (main_box,
        file_box,
        add_button,
        do_button) = widget_builder("Get page(s)".to_string(),
                "/usr/share/yapm/ressources/get_icon.png".to_string(),
                move_flag,
                pdf_view_flag,
                select_flag,
                del_flag);

    let win = window.clone();
    let fb = file_box.clone(); 
    add_button.connect_clicked( move |_e|{

        let file = gtk4::FileDialog::builder().title("Choose your pdf files").build();
        let f_box = file_box.clone();
        file.open(Some(&win), gtk4::gio::Cancellable::NONE,  move |arg0: Result<File, gtk4::glib::Error>| on_select_pages(arg0,f_box,select_flag,check_visible));
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
            let file_path = file_box.first_child()
                .unwrap()
                .first_child()
                .unwrap()
                .downcast::<gtk4::Box>()
                .unwrap()
                .first_child().unwrap()
                .downcast::<Label>()
                .unwrap()
                .label();
            
            let (accept_button_fwin,path_content_buffer,fwin) = folder_window(b.clone(),".pdf",false);
            let file_path = file_path.replacen(".pdf", "_modified.pdf", 1);
            path_content_buffer.set_text(&file_path);
           
            accept_button_action(accept_button_fwin, path_content_buffer, number, file_box.clone(), fwin,window);
        
        }
    
        
        
    });

    main_box
}


fn accept_button_action(button:Button,path_content_buffer:TextBuffer,number:i32,file_box: ListBox,win:ApplicationWindow,main_win:ApplicationWindow){
    button.connect_clicked(move |b|{
        b.set_sensitive(false);
        let mut pages_list:String= String::new();

        for i in 0..number{
            //access the invisible label the absolute path
            let abs_path = file_box.row_at_index(i)
                .unwrap()
                .first_child()
                .and_downcast::<gtk4::Box>()
                .unwrap()
                .observe_children();

            //get the page number 
            let mut label= abs_path.into_iter().filter_map(|e|{
                let mut tmp = e.iter().cloned();
                if tmp.next().unwrap().downcast::<Widget>().unwrap().widget_name() == "page"
                {return Some(e);} else {None}}
            );

            let binding = label.next()
            .unwrap().unwrap().downcast::<Label>().unwrap().label();
            let page_number = binding.split(" ").collect::<Vec<&str>>()[1];

            //Check if the check button have been clicked
            let check_box = file_box.row_at_index(i)
                .unwrap()
                .first_child()
                .and_downcast::<gtk4::Box>()
                .unwrap()
                .last_child()
                .unwrap()
                .downcast::<CheckButton>()
                .unwrap();

            if !check_box.is_active(){
                pages_list = format!("{page_number} {pages_list}");
            }
        }
        pages_list.pop();
        
        let write_path = path_content_buffer.text(&path_content_buffer.start_iter(),&path_content_buffer.end_iter(), false);
        let file_path = write_path.replace("_modified.pdf", ".pdf");
        
        let mut args = vec![file_path,write_path.to_string(),pages_list];

        let res = cli::get_page(&mut args, true);

        b.set_sensitive(true);
        win.close();
        match res {
            Ok(()) => done_window(&main_win,write_path.to_string()),
            Err(msg) => warning_window(&main_win,msg),
        }
});}