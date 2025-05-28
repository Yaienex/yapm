pub fn get_box() -> gtk4::Box{
    let margin = 10;
    let main_box = gtk4::Box::builder()
        .margin_bottom(margin)
        .margin_end(margin)
        .margin_start(margin)
        .margin_top(margin)
        .orientation(gtk4::Orientation::Horizontal)
        .build();




    main_box
}