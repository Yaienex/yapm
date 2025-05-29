use std::{cell::RefCell, path::PathBuf, process::exit, rc::Rc};

use gtk4::{ cairo::{self, Context}, gdk::{self, Key}, gio::{prelude::{FileExt, InputStreamExt, InputStreamExtManual}, File}, glib::{object::Cast, GString, Propagation},  prelude::{ BoxExt, ButtonExt, DrawingAreaExtManual, GestureSingleExt, GtkWindowExt, ListBoxRowExt, TextBufferExt, TextViewExt, WidgetExt}, ActionBar, ApplicationWindow, Button, DrawingArea, DropTarget,  EventControllerKey, FileDialog, HeaderBar, Image, Label, ListBox, ListBoxRow,  TextView};
use lopdf::Document;
use poppler::Document as PopDocument;

use crate::app::cli;



//Move widget on the main box
pub fn merge_box(window:&ApplicationWindow) -> gtk4::Box{

    //constant
    let margin = 5;
    
    //Main widget - Left Box
    let main_box = gtk4::Box::builder()
        .margin_bottom(margin*3)
        .margin_end(margin*3)
        .margin_start(margin*3)
        .margin_top(margin*3)
        .orientation(gtk4::Orientation::Horizontal)
        .hexpand(true)
        .build();
    //Right Box
    let decision_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_bottom(margin)
        .margin_end(margin)
        .margin_start(margin)
        .margin_top(margin)
        .name("decision-box")
        .halign(gtk4::Align::End)
        .build();

    //Inside decision box
    let (information_box,draw_area,name_content,page_number_content,full_path_content) = information_box_builder();

    //Drop zone config 
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
    });
    let drop_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .name("drop-box")
        .build();
    //drop_box.add_controller(drop_box_controller); Delaying this to focus on a working application 
    
    //Manage pdf file from the UI
    let file_box = file_box(name_content,page_number_content,full_path_content);
    drop_box.append(&file_box);

    let manage_box = gtk4::Box::builder()
        .name("manage-box")
        .halign(gtk4::Align::Fill)
        .vexpand(false)
        .valign(gtk4::Align::Fill)
        .hexpand(true)
        .orientation(gtk4::Orientation::Horizontal)
        .build();
   
    //Action bar 
    let action_bar = ActionBar::builder()
        .halign(gtk4::Align::Fill)
        .vexpand(false)
        .valign(gtk4::Align::Fill)
        .hexpand(true)
        .build();

    //manage button
    let add_button = button_builder("/usr/share/yapm/ressources/plus_icon.png".to_string());
    let del_button = button_builder("/usr/share/yapm/ressources/del_icon.png".to_string());
    
    //Move actions box
    let up_button = move_button_builder("/usr/share/yapm/ressources/up_icon.png".to_string(),true,file_box.clone());
    let down_button = move_button_builder("/usr/share/yapm/ressources/down_icon.png".to_string(),false,file_box.clone());

    let move_box = gtk4::Box::builder()
        .name("move-box")
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .hexpand(true)
        .orientation(gtk4::Orientation::Horizontal)
        .build();
   
    move_box.append(&up_button);move_box.append(&down_button);

    //Pdf view button
    let pdf_button = button_builder("/usr/share/yapm/ressources/loupe_icon.png".to_string());

    let win = window.clone().to_owned();
    let f_box = file_box.clone();
    add_button.connect_clicked(move |_e|{
        let file = FileDialog::builder().title("Choose your pdf files").build();
        let f_box = file_box.clone();
        file.open_multiple(Some(&win), gtk4::gio::Cancellable::NONE,  move |arg0: Result<gtk4::gio::ListModel, gtk4::glib::Error>| on_select(arg0,f_box));
  
    });

    let fb = f_box.clone();
    pdf_button.connect_clicked(move |button|{
        let row_option = f_box.selected_row();
        button.set_sensitive(false);
        if row_option.is_some(){

            //get the full path from the hidden label
            let row = row_option.unwrap();
            let label_hid = row.first_child()
                .unwrap()
                .downcast::<gtk4::Box>()
                .unwrap()
                .first_child()
                .unwrap()
                .downcast::<Label>()
                .unwrap().label();

            pdf_display(label_hid.to_string(),button.clone());

        }
    });

    let file_box = fb.clone();
    del_button.connect_clicked(move |_e|{
        if fb.selected_row().is_some(){
            fb.remove(&fb.selected_row().unwrap());
        }
    });
    


    action_bar.pack_start(&add_button);
    action_bar.pack_start(&pdf_button);
    action_bar.set_center_widget(Some(&move_box));
    action_bar.pack_end(&del_button);

    manage_box.append(&action_bar);


    drop_box.append(&manage_box);
    //The action button on the bottom right
    let do_button = do_button_builder(file_box);
   
    //Setting up everything 
    decision_box.append(&information_box);
    decision_box.append(&do_button);

    main_box.append(&drop_box);
    main_box.append(&decision_box); 
    
    main_box
}


fn button_builder(icon_path:String) -> Button{
    let icon = Image::from_file(icon_path);
    let button = Button::builder()
        .child(&icon)
        .name("add-button")
        .build(); 
    return button;
}

fn do_button_builder(file_box:ListBox) -> Button{
    let do_button =Button::builder()
        .valign(gtk4::Align::End)
        .vexpand(false)
        .name("do-button")
        .build();
    let do_button_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();
    let do_button_label = Label::new(Some("Merge"));
    let do_button_icon = Image::from_file("ressources/merge_icon.png");
    do_button_icon.set_icon_size(gtk4::IconSize::Large);


    do_button_box.append(&do_button_icon);
    do_button_box.append(&do_button_label);

    do_button.set_child(Some(&do_button_box));
    let b = do_button.clone();
    do_button.connect_clicked(move |_e|{
        let b = b.clone();
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
    do_button
}

fn move_button_builder(icon_path:String,flag:bool,file_box: ListBox) -> Button {
    let button:Button;
    if flag {
        button =Button::builder()
            .valign(gtk4::Align::Start)
            .vexpand(false)
            .name("move-button")
            .build();
        button.connect_clicked(move |_b|{
            let mut row_option = file_box.selected_row();
            let fb = file_box.clone();
            if  row_option.is_some(){
                let row = row_option.clone().unwrap();
                let (_n,row_index) = count(file_box.clone(),row.clone());
                
                if row_index > 0 { //we can't go up at row 0
                    file_box.remove(&row);
                    file_box.insert(&row,row_index-1);
                    row_option = file_box.row_at_index(row_index-1);
                    file_box.unselect_row(&row);
                   
                }
                
                fb.select_row(row_option.as_ref());
            
            }
        });
    }
    else{
         button =Button::builder()
            .valign(gtk4::Align::End)
            .vexpand(false)
            .name("move-button")
            .build();

        button.connect_clicked(move |_b|{
            let mut row_option = file_box.selected_row();
            let fb = file_box.clone();
            if  row_option.is_some(){
                let row = row_option.clone().unwrap();
                let (n,row_index) = count(file_box.clone(),row.clone());
                if row_index < n { //we can't go down at row n
                    file_box.remove(&row);
                    file_box.insert(&row,row_index+1);
                    row_option = file_box.row_at_index(row_index+1);
                    file_box.unselect_row(&row);
                }

                fb.select_row(row_option.as_ref());
            

            }
        });
    }
    let button_icon = Image::from_file(icon_path);

    button.set_child(Some(&button_icon));

    button
}

fn information_box_builder() -> (gtk4::Box,DrawingArea,Label,Label,Label){
    let info_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .vexpand(true)
        .name("info-box")
        .build();
    let pdf_preview = DrawingArea::builder()
        .name("pdf-preview")
        .vexpand(true)
        .width_request(100)
        .height_request(100)
        .valign(gtk4::Align::Fill)
        .build();

    //Every boxes
    let (name_box,name_content) = info_box_subbox_builder("Name :",false);
    let (page_number_box,page_number_content) =info_box_subbox_builder("Number of pages :",false);
    let (full_path_box,full_path_content) = info_box_subbox_builder("Absolute path :",true);

    let global_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .name("global-box")
        .valign(gtk4::Align::End)
        .build();
    info_box.append(&pdf_preview);
    global_box.append(&name_box);
    global_box.append(&full_path_box);
    global_box.append(&page_number_box);
    info_box.append(&global_box);

    (info_box,pdf_preview,name_content,page_number_content,full_path_content)
}

fn info_box_subbox_builder(label: &str,vertical:bool) -> (gtk4::Box,Label){

   let sub_box:gtk4::Box;
   if vertical{
    sub_box =gtk4::Box::builder()
        .margin_top(5)
        .margin_bottom(5)
        .orientation(gtk4::Orientation::Vertical)
        .build();
   } else{
     sub_box =gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();
   }
   let name = Label::builder()
        .label(label)
        .margin_end(10)
        .halign(gtk4::Align::Start)
        .build();

    let content = Label::builder()
        .label("")
        .wrap(true)
        .wrap_mode(gdk::pango::WrapMode::Word)
        .halign(gtk4::Align::Start)
        .build();

    sub_box.append(&name);
    sub_box.append(&content);

    (sub_box,content)
}

fn row_file(path : PathBuf,name:&str) -> ListBoxRow{
    //inital row
    let margin = 5;
    let row = ListBoxRow::builder()
                .name("row")
                .margin_bottom(margin)
                .margin_top(margin)
                .margin_start(margin)
                .margin_end(margin)
                .build();
    let path = path.to_str().unwrap();

    let path_label = Label::builder()
        .label(path)
        .visible(false)
        .build();
    let name_label = Label::new(Some(name));
    let row_child = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    row_child.append(&path_label);
    row_child.append(&name_label);
    row.set_child(Some(&row_child));

    return  row;
}

fn file_box(name_content: Label,page_number_content:Label,full_path_content:Label) -> ListBox{
    let file_box = gtk4::ListBox::builder()
        .name("file-box")
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .valign(gtk4::Align::Fill)
        .vexpand(true)
        .build();

    //Show information on the Decision Box !
    file_box.connect_row_selected(move |file_box,row|{
    if row.is_some(){
        let abs_path = row.unwrap()
            .child() //row child
            .unwrap()//Box
            .downcast::<gtk4::Box>()
            .unwrap()
            .first_child()
            .unwrap()
            .downcast::<Label>()
            .unwrap().label();
        let parse_path:Vec<&str> = abs_path.split("/").collect();
        let name = parse_path[parse_path.len() -1];
        let uri_file = format!("file://{}",&abs_path.to_string());
        let doc = match PopDocument::from_file(&uri_file, Some("")){
                Ok(doc) => doc,
                Err(er) => {println!("{:?}",er);exit(1);}
            };
            
            //Updating the info frame
            name_content.set_label(name);
            page_number_content.set_label(doc.n_pages().to_string().as_str());
            full_path_content.set_label(&abs_path);
        };

        file_box.select_row(row);
    });
    file_box
}

fn folder_window(do_button :Button,file_box:ListBox,number:i32){
    let margin = 10;
    let bar = HeaderBar::builder()
        .build();
    let window = ApplicationWindow::builder()
        .resizable(false)
        .titlebar(&bar)
        .default_height(200)
        .default_width(400)
        .title("Choose a path to save your file !")
        .build();
    
    let boxe = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .vexpand(true)
        .valign(gtk4::Align::Fill)
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .build();

    let path_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .margin_bottom(margin)
        .margin_end(margin)
        .margin_start(margin)
        .margin_top(margin)
        .vexpand(true)
        .build();

    let path_label = Label::builder()
        .label("Path : ")
        .halign(gtk4::Align::Start)
        .build();
    let path_content = TextView::builder()
        .hexpand(true)
        .wrap_mode(gtk4::WrapMode::Word)
        .accepts_tab(false)
        .build();
    path_box.append(&path_label);path_box.append(&path_content);

    let path_check_box = gtk4::Box::builder()
        .margin_start(margin)
        .margin_end(margin)
        .hexpand(true)
        .halign(gtk4::Align::Center)
        .vexpand(true)
        .build();
    let path_check = Label::builder()
        .vexpand(true)
        .build();

    path_check_box.append(&path_check);
    
    
    let cancel_button = Button::builder()
        .hexpand(true)
        .margin_bottom(margin)
        .label("Cancel")
        .halign(gtk4::Align::Center)
        .build();
    let accept_button = Button::builder()
        .hexpand(true)
        .margin_bottom(margin)
        .label("Accept")
        .halign(gtk4::Align::Center)
        .build();
    let button_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .hexpand(true)
        .valign(gtk4::Align::End)
        .vexpand(true)
        .halign(gtk4::Align::Fill)
        .build();


    button_box.append(&accept_button); button_box.append(&cancel_button);
    boxe.append(&path_box); boxe.append(&path_check_box); boxe.append(&button_box);
    window.set_child(Some(&boxe));
    window.present();

    //we blocked the button after one push
    do_button.set_sensitive(false);
    
    window.connect_close_request(move |_p|{
        do_button.set_sensitive(true);
        gdk::glib::Propagation::Proceed
    });
    let win = window.clone();
    cancel_button.connect_clicked(move |_e|{
        window.close();
    });

    let pac = path_content.clone();
    let acb = accept_button.clone();
    //Enter key 
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_, key, _, _| {
        if key == Key::Return {
            accept_button.emit_clicked();
            Propagation::Proceed
        } else {
            Propagation::Proceed
        }
    });
    pac.add_controller(controller);

    //connecting the changed event on the path content buffer
    let path_content_buffer = pac.buffer();
    let pcb = path_check_box.clone();
    let accept_button = acb.clone();
    acb.set_sensitive(false);
    path_content_buffer.connect_changed(move |buff|{
        let path = buff.text(&buff.start_iter(), &buff.end_iter(), true).to_string();
        let mut path_str :String = String::new();
        let mut splitted_path: Vec<&str> = Vec::new();
        let mut message:String = String::new();

        if path.starts_with("/"){
            path_str = String::from("/");
        }
        splitted_path = path.split("/").collect();
        
        
        //taking only the file name 
        let file_name = splitted_path.remove(splitted_path.len() -1);
        let mut tmp =splitted_path.join("/"); 
        if tmp.starts_with("/"){
            tmp.remove(0);
            path_str += &tmp;
        } else {
            path_str += &tmp;
        }
        //we remove the file name so we override path_buf to only contain the link to the directory
        let ps = path_str.clone();
        let path_buf:PathBuf = path_str.into();
        if !path_buf.has_root(){
            message = format!("<span foreground=\"red\">Absolute link</span> is needed");
            accept_button.set_sensitive(false);
        }
        else if !path_buf.exists() {
            message = format!("The <span foreground=\"red\">given</span> path doesn't exist");
            accept_button.set_sensitive(false);
        }
        else if !path_buf.is_dir() && ps !="/"{
            message = format!("An <span foreground=\"red\">existing</span> directory is needed");
            accept_button.set_sensitive(true); //a warning window on popup | it will create a pdf with a default name
        } else if !file_name.ends_with(".pdf"){
            message = format!("A <span foreground=\"red\">pdf extension</span> is needed");
            accept_button.set_sensitive(true);//just place the damn .pdf extension at the end
        }

        let path_check = Label::builder()
            .label(message)
            .use_markup(true)
            .wrap_mode(gdk::pango::WrapMode::Word)
            .margin_start(margin)
            .margin_end(margin)
            .vexpand(true)
            .hexpand(true)
            .halign(gtk4::Align::Center)
            .build();

        let fc = &pcb.first_child().unwrap();
        pcb.insert_child_after(&path_check, Some(fc));
        pcb.remove(fc);
   
    });
    //accept connection
    acb.connect_clicked(move |b|{
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

        let path: String = {
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
        
        let _ = cli::merge(pdf_list, &path);
        b.set_sensitive(true);
        win.close();
    });
    
}

fn count(file_box: ListBox,row_to_find:ListBoxRow) -> (i32,i32){
    let mut number:i32 = 0;
    let mut row_index:i32 =0;
    let mut row: Option<ListBoxRow> = file_box.row_at_index(number );
    while row.is_some() {
        if row.unwrap() == row_to_find{
            row_index = number;
        }
            number +=1;
            row = file_box.row_at_index(number );
            
    } 

    (number -1, row_index)
}

fn pdf_display(filename: String,button:Button){

    let filename = format!("file://{filename}");
     let app_wrapper = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let bottom_bar = gtk4::Box::builder().hexpand_set(true).build();
    let header_bar = gtk4::HeaderBar::builder().build();
    let bt = bottom_bar.clone();
    app_wrapper.append(&bottom_bar);

    let window = ApplicationWindow::builder()
        .child(&app_wrapper)
        .build();
    window.set_titlebar(Some(&header_bar));

    let toggle_fullscreen =  move || {
        if header_bar.is_visible() {
            header_bar.set_visible(false);
            bottom_bar.set_visible(false);
        } else {
            header_bar.set_visible(true);
            bottom_bar.set_visible(true);
        }
    };


    let drawing_area = DrawingArea::builder()
        .width_request(100)
        .height_request(100)
        .hexpand(true)
        .vexpand(true)
        .build();
    let first_child = app_wrapper.first_child().unwrap();
    let last_child = app_wrapper.last_child().unwrap();
    if &first_child != &last_child {
        app_wrapper.remove(&first_child);
    }

    app_wrapper.prepend(&drawing_area);
    let page_indicator = Label::builder().label("Counting").build();
    let old_indicator = bt.last_child();
    if old_indicator.is_some() {
        bt.remove(&old_indicator.unwrap());
    }
    bt.append(&page_indicator);

    let doc = PopDocument::from_file(filename.as_str(), Some("")).unwrap();
    let num_pages = doc.n_pages();

    let current_page = Rc::new(RefCell::new(1));
    let current_page_copy_another = current_page.clone();
    let current_page_view = current_page.clone();

    let surface = cairo::ImageSurface::create(cairo::Format::Rgb24, 0, 0).unwrap();
    let ctx = Context::new(&surface).unwrap();
    let da = drawing_area.clone();
    let update_page_status = move || {
        let page_status: String = format!("{} of {}", *current_page_copy_another.borrow_mut(), num_pages);
        let page_status_s: &str = &page_status[..];
        page_indicator.set_label(page_status_s);
        drawing_area.queue_draw();

    };

    update_page_status();

    let click = gtk4::GestureClick::new();
    click.set_button(0);
    let deo = da.clone();
    click.connect_pressed(move |_count, _, x, y| {
        let center = da.width() / 2;
        if y < (da.height() / 5) as f64 {
            toggle_fullscreen();
        } else if x > center as f64 &&  *current_page.borrow_mut() < num_pages {
            *current_page.borrow_mut() += 1;
        } else if x < center as f64 && *current_page.borrow_mut() > 1 {
            *current_page.borrow_mut()  -= 1;
        }
        update_page_status();
                    
    });
            

    deo.add_controller(click);

    deo.set_draw_func(move |area, context, _a, _b| {
            let current_page_number = &current_page_view.borrow_mut();
            context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            context.paint().unwrap();
            context.fill().expect("uh oh");
            context.paint().unwrap();

            let page = doc.page(**current_page_number - 1).unwrap();
            let (w, h) = page.size();
            let width_diff = area.width() as f64 / w;
            let height_diff = area.height() as f64 / h;
            context.save().unwrap();
            if width_diff > height_diff {
                context.scale(height_diff, height_diff);
            } else {
                context.scale(width_diff, width_diff);
            }
            page.render(&context);

            let r = ctx.paint();
            match r {
                Err(v) => println!("Error painting PDF: {v:?}"),
                Ok(_v) => println!(""),
            }

            ctx.show_page().unwrap();
        });

        window.connect_close_request(move |_w|{
            button.set_sensitive(true);

            Propagation::Proceed
        });

        window.present()
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

fn _on_save(arg : Result<File, gdk::glib::Error>,_pdf_list :Vec<Document>){
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
