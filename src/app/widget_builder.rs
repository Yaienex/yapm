use std::{cell::RefCell, path::PathBuf, process::exit, rc::Rc};

use gtk4::{ cairo::{self, Context}, gdk::{self, prelude::{DeviceExt, DisplayExt, SeatExt}, Key, ModifierType}, gio:: File, glib::{object::Cast, Propagation}, prelude::{ AdjustmentExt, BoxExt, ButtonExt, DrawingAreaExt, DrawingAreaExtManual, EventControllerExt, GestureSingleExt, GtkWindowExt, ListBoxRowExt, TextBufferExt, TextViewExt, WidgetExt}, ApplicationWindow, Button, DrawingArea, EventControllerKey, EventControllerScroll, HeaderBar, Image, Label, ListBox, ListBoxRow, ScrolledWindow, TextBuffer, TextView};
use lopdf::Document;
use poppler::Document as PopDocument;


pub fn widget_builder(action_name:String,
                        icon_path:String,
                        move_flag:bool,
                        pdf_view_flag:bool
 ) -> (gtk4::Box,
        ListBox,
        Button,
        Button){
    //constant
    let margin = 5;
    
    //Main widget - Left Box
    let main_box = gtk4::Box::builder()
        .margin_bottom(margin*3)
        .name("main-box")
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
        .halign(gtk4::Align::Fill)
        .build();

    //Inside decision box
    let (information_box,draw_area,name_content,page_number_content,full_path_content) = information_box_builder();

    let drop_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .name("drop-box")
        .build();
       
    //Manage pdf file from the UI
    let file_box = file_box(name_content,page_number_content,full_path_content,draw_area);
    drop_box.append(&file_box);

    //Action bar 
    let action_bar = gtk4::Box::builder()
        .name("action-bar")
        .margin_top(1)
        .halign(gtk4::Align::Fill)
        .vexpand(false)
        .valign(gtk4::Align::End)
        .hexpand(true)
        .build();

    /*
    manage button
    */
    let add_button = button_builder("/usr/share/yapm/ressources/plus_icon.png".to_string());
    let del_button = button_builder("/usr/share/yapm/ressources/del_icon.png".to_string());
    add_button.set_hexpand(true);
    let fb = file_box.clone();

    

    let file_box = fb.clone();
    del_button.connect_clicked(move |_e|{
        if fb.selected_row().is_some(){
            let row =&fb.selected_row().unwrap();
            fb.remove(row);
            fb.emit_unselect_all();
        }
    });
    

    //Sorting them
    add_button.set_halign(gtk4::Align::Start);
    del_button.set_hexpand(true);
    del_button.set_halign(gtk4::Align::End);

    action_bar.append(&add_button);
    if pdf_view_flag{
        add_button.set_hexpand(false);
        let pdf_button = button_builder("/usr/share/yapm/ressources/loupe_icon.png".to_string());
        pdf_button.set_halign(gtk4::Align::Start);pdf_button.set_hexpand(true);
        let fb = file_box.clone();
        pdf_button.connect_clicked(move |button|{
            let row_option = fb.selected_row();
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

            } else {
                button.set_sensitive(true);
            }
        });
        action_bar.append(&pdf_button);
    }
    if move_flag{
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
        action_bar.append(&move_box);
    }
    action_bar.append(&del_button);



    drop_box.append(&action_bar);
    
    //The action button on the bottom right
    let do_button = do_button_builder(action_name,icon_path);
   
    //Setting up everything 
    decision_box.append(&information_box);
    decision_box.append(&do_button);

    main_box.append(&drop_box);
    main_box.append(&decision_box); 
    
    (main_box,file_box,add_button,do_button)
}



pub fn button_builder(icon_path:String) -> Button{
    let icon = Image::from_file(icon_path);
    let button = Button::builder()
        .margin_start(5)
        .margin_end(5)
        .child(&icon)
        .name("add-button")
        .build(); 
    return button;
}

//name 
fn do_button_builder(action_name:String,icon_path:String) -> Button{
    let do_button =Button::builder()
        .valign(gtk4::Align::End)
        .vexpand(false)
        .halign(gtk4::Align::Fill)
        .name("do-button")
        .build();
    let do_button_box = gtk4::Box::builder()
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .orientation(gtk4::Orientation::Horizontal)
        .build();
    let do_button_label = Label::builder()
        .label(action_name)
        .hexpand(true)
        .halign(gtk4::Align::Center)
        .build();
    let do_button_icon = Image::builder()
        .file(icon_path)
        .halign(gtk4::Align::Start)
        .build();
    do_button_icon.set_icon_size(gtk4::IconSize::Large);
    

    do_button_box.append(&do_button_icon);
    do_button_box.append(&do_button_label);

    do_button.set_child(Some(&do_button_box));
    do_button
}

fn move_button_builder(icon_path:String,flag:bool,file_box: ListBox) -> Button {
    let button:Button;
    if flag {
        button =Button::builder()
            .margin_end(5)
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
            .margin_start(5)
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
    let (name_box,name_content) = info_box_subbox_builder("Name :",true);
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
    let label = format!("<b>{label}</b>");
    let sub_box:gtk4::Box;
    if vertical{
         sub_box =gtk4::Box::builder()
            .name("subinfo-box")
            .margin_top(5)
            .margin_bottom(5)
            .orientation(gtk4::Orientation::Vertical)
            .build();
    } else{
        sub_box =gtk4::Box::builder()
            .name("subinfo-box")
            .orientation(gtk4::Orientation::Horizontal)
            .build();
    }
    let name = Label::builder()
        .use_markup(true)
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

fn file_box(name_content: Label,page_number_content:Label,full_path_content:Label,draw_area:DrawingArea) -> ListBox{
    let file_box = gtk4::ListBox::builder()
        .name("file-box")
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .valign(gtk4::Align::Fill)
        .vexpand(true)
        .build();

    let nc = name_content.clone();
    let pnc = page_number_content.clone();
    let fpc = full_path_content.clone();
    let da = draw_area.clone();
    //Show information on the Decision Box !
    file_box.connect_row_selected(move |file_box,row|{
        draw_area.set_visible(true);
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

            //Drawing 
            let surface = cairo::ImageSurface::create(cairo::Format::Rgb24, 0, 0).unwrap();
            let ctx = Context::new(&surface).unwrap();
          
            draw_area.set_draw_func(move |area, context, _a, _b| {
                let page = doc.page(0).unwrap();
                let (w, h) = page.size();
                let (width,height) = (area.width(),area.height());


                context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                context.paint().unwrap();
                context.fill().expect("nuh uh");
                context.paint().unwrap();


                let scale_x = width as f64 / w;
                let scale_y = height as f64 / h;
                let scale = scale_x.min(scale_y); // Keep proportions

                // Center
                let offset_x = (width as f64 - w * scale) / 2.0;
                let offset_y = (height as f64 - h * scale) / 2.0;
                context.save().unwrap();

                context.translate(offset_x, offset_y);
                context.scale(scale, scale);

                // Dessine la page
                page.render(&context);

                let r = ctx.paint();
                match r {
                    Err(v) => println!("Error painting PDF: {v:?}"),
                    Ok(_v) => _v,
                }

                ctx.show_page().unwrap();
            });
        };

        file_box.select_row(row);
    });

    file_box.connect_unselect_all(move |_file_box|{
        nc.set_label("");
        pnc.set_label("");
        fpc.set_label("");
        da.set_draw_func(|_, cr, _, _| {
            cr.set_source_rgb(1.0, 1.0, 1.0); // white background
            cr.paint().unwrap(); // fill with white
        });

    });
    file_box
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

    let hb = header_bar.clone();
    let toggle_fullscreen =  move || {
        if header_bar.clone().is_visible() {
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
    let doco = doc.clone();
    let num_pages = doc.n_pages();

    let (orig_width,orig_height) = doc.page(0).unwrap().size();
    drawing_area.set_content_height(orig_height as i32);
    drawing_area.set_content_width(orig_width as i32);

    let scale = Rc::new(RefCell::new(1.0));
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
        } else if x > center as f64 &&  *current_page.borrow_mut() < num_pages  {
            *current_page.borrow_mut() += 1;
        } else if x < center as f64 && *current_page.borrow_mut() > 1 {
            *current_page.borrow_mut()  -= 1;
        }
        update_page_status();
                    
    });
            

    deo.add_controller(click);

    let scale_clone = scale.clone();
    deo.set_draw_func(move |_area, context, _a, _b| {
            let current_page_number = &current_page_view.borrow_mut();
            context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            context.paint().unwrap();
            context.fill().expect("uh oh");
            context.paint().unwrap();

            let page = doc.page(**current_page_number - 1).unwrap();

            let scale = *scale.borrow();
            context.save().unwrap();
            context.scale(scale,scale);
            page.render(&context);

            let r = ctx.paint();
            match r {
                Err(v) => println!("Error painting PDF: {v:?}"),
                Ok(_v) => _v,
            }

            ctx.show_page().unwrap();
           

        });

        // GestureScroll for zoom
        let scrolled_window = ScrolledWindow::builder()
            .child(&app_wrapper)
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .build();
        
        let scroll_controller = EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
        let draw_area = deo.clone();
        let h_adj = scrolled_window.hadjustment();
        let v_adj = scrolled_window.vadjustment();
        scroll_controller.connect_scroll(move |controller, _, delta| {
            let control_pressed = controller
                .widget()
                .and_then(|w| Some(w.display()))
                .and_then(|d| d.default_seat())
                .and_then(|s| s.keyboard())
                .map(|k| k.modifier_state().contains(ModifierType::CONTROL_MASK))
                .unwrap_or(false);
            let shift_pressed = controller
                .widget()
                .and_then(|w| Some(w.display()))
                .and_then(|d| d.default_seat())
                .and_then(|s| s.keyboard())
                .map(|k| k.modifier_state().contains(ModifierType::SHIFT_MASK))
                .unwrap_or(false);

            // shift check to block scroll behavior 
            if shift_pressed  {
                    //PDF Zoom
                    let mut scale = scale_clone.borrow_mut();
                    *scale = *scale *{ if delta < 0.0 { 1.1} else { if *scale < 1.0 {1.0} else {0.9 }}};

                    // Adapter la taille du DrawingArea
                    let new_width = (orig_width * *scale) as i32;
                    let new_height = (orig_height * *scale) as i32;
                    deo.set_content_width(new_width);
                    deo.set_content_height(new_height);
                    deo.queue_draw();
            } else if control_pressed { // horizontal scroll
                let new_value = (h_adj.value() + delta * 30.0)
                    .clamp(h_adj.lower(), h_adj.upper() - h_adj.page_size());
                h_adj.set_value(new_value);
            } else {
                let new_value = (v_adj.value() + delta * 30.0)
                    .clamp(v_adj.lower(), v_adj.upper() - v_adj.page_size());
                v_adj.set_value(new_value);
            }
                Propagation::Stop // removing the normal behavior
        });
        draw_area.add_controller(scroll_controller);


        let window = ApplicationWindow::builder()
            .child(&scrolled_window)
            .default_height(doco.page(0).unwrap().size().1 as i32)
            .default_width(doco.page(0).unwrap().size().0 as i32)
            .resizable(false)
            .build();
        window.set_titlebar(Some(&hb));

        window.connect_close_request(move |_w|{
            button.set_sensitive(true);

            Propagation::Proceed
        });
        
        window.present()
}


// Public functions accessible from the other widget 
pub fn row_file(path : PathBuf,name:&str) -> ListBoxRow{
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


#[allow(unused_assignments)]
pub fn folder_window(do_button :Button,extension:&str) -> (Button,
                                                                        TextBuffer,
                                                            ApplicationWindow){
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
        .vexpand(false)
        .valign(gtk4::Align::Fill)
        .halign(gtk4::Align::Fill)
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
    let extension = extension.to_string();
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
        } else if !file_name.ends_with(&extension){
            message = format!("A <span foreground=\"red\"> {extension} extension</span> is needed");
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
    
    //return the necessary objects to connect the action 
    (acb,path_content_buffer,win)
    
}

pub fn count(file_box: ListBox,row_to_find:ListBoxRow) -> (i32,i32){
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



//Callbacks 
//to override

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
