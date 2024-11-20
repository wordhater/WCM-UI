use gtk::glib::Char;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.WCM_UI";

// other functions

fn count_words(input: &str) -> i32 {
    let parts = input.split(" ");
    let mut count: i32 = 0;
    for part in parts{
        count += 1;
    }
    return count;
}


// Modify word count


fn modifywrapper<'a>(input: &'a str, count: i32, replacement: &'a str) -> &'a str {
    
    return "null";
}

fn main() -> glib::ExitCode {
    const TESTING: bool = true;
    if TESTING{
        println!("{}", count_words("1 2 3 4 5 6 7 8 9 10"));
        let out: &str = modifywrapper("dfkjhf dfdshfkjjalks fhas ld k ja k j sl fh lskafhasjf hasdkjf haskdhfkasd fasdh fasdjhf jhsdfk ljsah as fk lj sa hfk asdf kh asd fkja shd fasdkjhfasdk ", 10, "u");
    }

    // Create a new application

    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button
    let button = Button::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}