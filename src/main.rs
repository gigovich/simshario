extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod main {
    use gtk::prelude::*;
    use gtk::{Button, Window, WindowType};

    fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize gtk!");
            return;
        }

        let window = Window::new(WindowType::Toplevel);
        window.set_title("First GTK program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        window.add(&button);
        window.show_all();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        button.connect_clicked(|_| {
            println!("Clicked");
        });

        gtk::main();
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    prog::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This version of GTK not supported");
}
