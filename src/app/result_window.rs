use gtk4::{ glib::object::{Cast, CastNone}, prelude::{ButtonExt, GtkWindowExt, WidgetExt}, ActionBar, ApplicationWindow, Button, CenterBox, HeaderBar, Widget, WindowControls, WindowHandle};


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
        gtk4::glib::Propagation::Proceed
    });    
}

pub fn warning_window(main_win:&ApplicationWindow){
    let window = ApplicationWindow::builder()
        .resizable(false)
        .transient_for(main_win)
        .modal(true)
        .default_height(200)
        .default_width(200)
        .title("All done ")
        .build();

    
    main_win.add_css_class("unfocused");
    window.present();
    let main_win= main_win.clone();
    window.connect_close_request(move |_w|{
        let main_win = main_win.clone();
        main_win.remove_css_class("unfocused");    
        gtk4::glib::Propagation::Proceed
    });
}