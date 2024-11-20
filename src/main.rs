use gtk::glib::Char;
use gtk::prelude::*;
use std::collections::LinkedList;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.WCM_UI";



// other functions

fn get_item_by_index_str(list: &LinkedList<String>, index: usize) -> String {
    let mut iter = list.iter();
    for _ in 0..index {
        if iter.next().is_none() {
            return "Index out of bounds".to_string();
        }
    }
    iter.next().map(|s| s.clone()).unwrap_or_else(|| "Index out of bounds".to_string())
}

fn count_words(input: &str) -> i32 {
    let parts = input.split(" ");
    let mut count: i32 = 0;
    for part in parts{
        count += 1;
    }
    return count;
}


// Modify word count

fn decrease<'a>(input: &'a str, goal: i32, replacement: &'a str) -> &'a str{
    let init_count: i32 = count_words(input);

    return input;
}

fn modifywrapper<'a>(input: &'a str, count: i32, replacement: &'a String) -> &'a str {
    let init_count: i32 = count_words(input);
    if init_count > count{
        println!("Decrease requested");
        return decrease(input, count, replacement);
    }else if init_count < count {
        println!("increase requested - unsupported");
        return "null";
    } else{
        println!("NO difference...");
        return input;
    }
}

fn main() -> glib::ExitCode {
    const TESTING: bool = true;
    // Setup chars
    let mut addCHARS:LinkedList<String> = LinkedList::new();
    addCHARS.push_back("\u{2000}".to_string());
    addCHARS.push_back("\u{2002}".to_string());
    addCHARS.push_back("\u{205f}".to_string());//best
    addCHARS.push_back("\u{2004}".to_string());//potential inconsistency
    let selected: String = get_item_by_index_str(&addCHARS, 2);
    if TESTING{
        println!("{}", count_words("1 2 3 4 5 6 7 8 9 10"));
        let out: &str = modifywrapper("dfkjhf dfdshfkjjalks fhas ld k ja k j sl fh lskafhasjf hasdkjf haskdhfkasd fasdh fasdjhf jhsdfk ljsah as fk lj sa hfk asdf kh asd fkja shd fasdkjhfasdk ", 10, &selected);
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
        .title("WCM UI")
        .child(&button)
        .build();

    // Present window
    window.present();
}