use gtk4::{ glib::{object::{Cast, CastNone}, Propagation}, prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt}, ApplicationWindow, Button, CenterBox, HeaderBar, Label, Widget, WindowHandle};


pub fn done_window(main_win:&ApplicationWindow){
    let window = ApplicationWindow::builder()
        .resizable(false)
        .modal(true)
        .default_height(200)
        .default_width(200)
        .transient_for(main_win)
        .title("All done ")
        .build();
    window.present();

    //getting the gome button for
    let widget_header = main_win
            .titlebar()
            .unwrap()
            .downcast::<HeaderBar>()
            .unwrap()
            .first_child()
            .and_downcast::<WindowHandle>()
            .unwrap()
            .first_child()
            .and_downcast::<CenterBox>()
            .unwrap()
            .first_child()
            .and_downcast::<gtk4::Box>()
            .unwrap()
            .observe_children();

    let home_button= {
        let mut button:Option<Button>=Some(Button::new());
        for i in &widget_header{
            let widget = i .unwrap();
            let tmp = widget.clone().downcast::<Widget>().unwrap().widget_name();
            if tmp == "button" {
                let tmp =widget.downcast::<Button>().unwrap();
                button=  Some(tmp);
                break;            
            }
        }
        button 
    }.unwrap();



    main_win.set_widget_name("unfocused");
    main_win.set_focusable(false);
    let main_win= main_win.clone();
    window.connect_close_request(move |_w|{
        let main_win = main_win.clone();
        main_win.set_widget_name("main-window"); 
        main_win.set_can_focus(true);
        home_button.emit_clicked();
        Propagation::Proceed
    });    
}

pub fn warning_window(main_win:&ApplicationWindow,msg:String){
    let window = ApplicationWindow::builder()
        .resizable(false)
        .transient_for(main_win)
        .modal(true)
        .default_height(200)
        .default_width(200)
        .name("warning")
        .build();
    
    //vbox
    let vbox = gtk4::Box::builder()
        .vexpand(true)
        .valign(gtk4::Align::Fill)
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .margin_end(20)
        .margin_start(20)
        .orientation(gtk4::Orientation::Vertical)
        .build();
    let erro_title = format!(" <span color=\"red\"font=\"16\">\u{ea6c}  ERROR \u{ea6c}</span>");
    let err_label = Label::builder()
        .use_markup(true)
        .halign(gtk4::Align::Center)
        .hexpand(true)
        .valign(gtk4::Align::Fill)
        .vexpand(true)
        .label(erro_title)
        .build();

    let message = format!("<span font=\"12\">{msg}</span>");
    let label = Label::builder()
        .use_markup(true)
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .valign(gtk4::Align::Center)
        .vexpand(true)
        .label(message)
        .build();

    let ok_button = Button::builder()
        .halign(gtk4::Align::Fill)
        .hexpand(true)
        .valign(gtk4::Align::End)
        .margin_bottom(20)
        .vexpand(true)
        .label("Ok")
        .build();
    let win = window.clone();
    ok_button.connect_clicked(move |_b|{
        let window = win.clone();
        window.close();
    });
    vbox.append(&err_label);
    vbox.append(&label);
    vbox.append(&ok_button);

    window.set_child(Some(&vbox));
    window.present();

    //intelligence to block the main window
    main_win.set_widget_name("unfocused");
    main_win.set_focusable(false);
    let main_win= main_win.clone();
    window.connect_close_request(move |_w|{
        let main_win = main_win.clone();
        main_win.set_widget_name("main-window"); 
        main_win.set_can_focus(true);
        Propagation::Proceed
    });    
}