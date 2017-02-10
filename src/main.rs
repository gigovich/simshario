extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod app {
    use gtk;
    use gtk::prelude::*;
    use gtk::{Builder, Window};

    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize gtk!");
            return;
        }

        let glide_src = include_str!("./gui/main.ui");
        let builder = Builder::new_from_string(glide_src);
        let window: Window = builder.get_object("mainWindow").unwrap();

        window.show_all();
        gtk::main();
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    app::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This version of GTK not supported");
}
