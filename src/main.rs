#[macro_use]
extern crate log;
extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod app {
    use std::io;
    use std::error::Error;
    use gtk::prelude::*;
    use gtk::{Builder, Window};
    use gtk;
    use std::io::prelude::{Read};
    use std::net::{TcpListener, TcpStream, Shutdown};

    fn process(s: TcpStream) -> Result<(), Box<Error>> {
        let mut stream = s;
        let mut buf = Vec::new();

        if let Err(err) = stream.read_to_end(&mut buf) {
            error!("read from connection {}", err);
            stream.shutdown(Shutdown::Both).unwrap();
        }
        Ok(())
    }

    fn listen() {
        let listener = TcpListener::bind("127.0.0.1:4812").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(err) = process(stream) {
                        error!("process connection {}", err);
                    }
                }
                Err(e) => {
                    error!("accept connection {}", e);
                }
            }
        }
    }

    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize gtk!");
            return;
        }

        let glide_src = include_str!("./gui/main.ui");
        let builder = Builder::new_from_string(glide_src);
        let window: Window = builder.get_object("mainWindow").unwrap();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.connect_destroy(|_| {
            println!(">>>>>");
        });

        window.show_all();
        listen();
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
