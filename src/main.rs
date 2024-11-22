use gtk::{glib::Char, Label};
use gtk::{prelude::*, Separator};
use gio::{glib::clone, prelude::*};
use std::{collections::LinkedList};
use adw::Application;
use gtk::{glib, Application as GTKApplication, ApplicationWindow, Button, SpinButton, Box};

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

fn is_all_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

// Modify word count


// decrease
fn decrease<'a>(input: &'a str, goal: i32, replacement: &'a str) -> String{
    let init_count: i32 = count_words(input);
    let parts = input.split(" ");
    let mut words:LinkedList<String> = LinkedList::new();
    for part in parts{
        words.push_back(part.to_string());
    }
    let mut output: String = get_item_by_index_str(&words, 0 as usize);
    let diff: i32 = words.len() as i32 - goal;
    if goal >= init_count/2 {
        let rate: f32 = (words.len() as f32 -1.0)/diff as f32;
        for i in 1..init_count{
            output.push_str(&get_item_by_index_str(&words, i as usize));
            if i % rate as i32 == 0 {
                output.push_str(replacement);
            }else {
                output.push_str(" ");
            }
        }
    }else{
        println!("less than half");
        let spacerate: f32 = ((init_count -1)/(init_count - diff)) as f32 /4.0;
        //+ ((init_count -1) % (init_count - diff)) as f32 /4.0)
        for i in 1..init_count{
            output.push_str(&get_item_by_index_str(&words, i as usize));
            println!("rate: {}", spacerate);
            println!("words: {}", init_count);
            if i as f32 % spacerate == 0.0 {
                output.push_str(" ");
                println!("space")
            }else {
                println!("char");
                output.push_str(replacement);
            }
        }
    }
    return output;
}

fn modifywrapper<'a>(input: &'a str, count: i32, replacement: &'a String) -> String {
    let init_count: i32 = count_words(input);
    if init_count > count{
        println!("Decrease requested");
        return decrease(input, count, replacement);
    }else if init_count < count {
        println!("increase requested - unsupported");
        return "null".to_string();
    } else{
        println!("NO difference...");
        return input.to_string();
    }
}

fn main()  -> glib::ExitCode {
    const TESTING: bool = true;
    // Setup chars
    let mut addCHARS:LinkedList<String> = LinkedList::new();
    addCHARS.push_back("\u{2000}".to_string());
    addCHARS.push_back("\u{2002}".to_string());
    addCHARS.push_back("\u{205f}".to_string());//best
    addCHARS.push_back("\u{2004}".to_string());//potential inconsistency
    addCHARS.push_back("TEST".to_string());//testing

    let selected: String = get_item_by_index_str(&addCHARS, 2);
    if TESTING{
        println!("{}", count_words("1 2 3 4 5 6 7 8 9 10"));
        let out: &str = &modifywrapper("df k j h f df ds h f k j j a l k s f h a s ld k ja k j sl fh l s k a f h a s j f h a s d k j f h a s k d h f k a s d fasdh fasdjhf jhsdfk ljsah as fk lj sa hfk asdf kh asd fkja shd fasdkjhfasdk ", 40, &selected);
        println!("result: {}", count_words(out));
        println!("text: {}", out);
    }
    // Create a new application

    // bootGUI();
    let app = Application::builder().application_id(APP_ID).build();

    // // Connect to "activate" signal of `app`
    app.connect_activate(bootGUI);

    // // Run the application
    app.run()

}

fn bootGUI(app: &Application){
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // for some reason it errors if I create multiple instances on the same element in the view

    let horizontal_separator_0 = gtk::Separator::builder()
        .margin_top(5)
        .margin_bottom(5)
        .build();

    let horizontal_separator_1 = gtk::Separator::builder()
        .margin_top(5)
        .margin_bottom(5)
        .build();
    let horizontal_separator_2 = gtk::Separator::builder()
        .margin_top(5)
        .margin_bottom(5)
        .build();
    let horizontal_separator_3 = gtk::Separator::builder()
        .margin_top(5)
        .margin_bottom(5)
        .build();
    let horizontal_separator_4 = gtk::Separator::builder()
        .margin_top(5)
        .margin_bottom(5)
        .build();
    
    let title = gtk::Label::builder()
        .label("Word Count Modifier UI")
        .use_markup(true)
        .build();
    title.set_markup("<span font=\"15\"><b>Word Count Modifier UI</b></span>");

    let input_text = gtk::Text::builder()
        .placeholder_text("Enter input text here")
        .margin_bottom(30)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let word_count = gtk::Label::builder()
        .label("Current Word Count: 0")
        .margin_bottom(5)
        .margin_top(5)
        .build();

    let count_input_label = gtk::Label::builder()
        .label("Word Count Goal:")
        .build();
    let count_input = gtk::Entry::builder()
        .build();

    let apply_button = gtk::Button::builder()
        .label("Apply")
        .build();
    let output = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::Word)
        .build();
    let output_title = Label::builder()
        .label("Result:")
        .use_markup(true)
        .build();
    output_title.set_markup("<span font=\"15\"><b> </b></span>");
    // layouts
    let row1 = Box::new(gtk::Orientation::Horizontal, 5);
    row1.set_margin_top(5);
    row1.set_margin_bottom(5);

    row1.append(&count_input_label);
    row1.append(&count_input);

    gtk_box.append(&title);
    gtk_box.append(&input_text);
    gtk_box.append(&horizontal_separator_0);
    gtk_box.append(&word_count);
    gtk_box.append(&horizontal_separator_1);
    gtk_box.append(&row1);
    gtk_box.append(&horizontal_separator_2);
    gtk_box.append(&apply_button);
    gtk_box.append(&horizontal_separator_3);
    gtk_box.append(&output_title);
    gtk_box.append(&output);

    // Interaction
    input_text.connect_changed(move |_text| {
        if _text.text_length() == 0 {word_count.set_label("Current Word Count: 0");}else{
            let mut lable: String = "Current Word Count: ".to_string();
            lable.push_str(&count_words(&_text.text()).to_string());
            word_count.set_label(&lable);
        }
    });

    count_input.connect_changed(|_count_input| {
        println!("update")
    });


    

    let main_window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(700)
        .child(&gtk_box)
        .title("WCM UI")
        .build();

    apply_button.connect_clicked(move |_button| {
        println!("modify btn clicked");
        if count_input.text().parse::<i32>().unwrap() as i32 == 0{println!("no number input")}else{
            let result = &modifywrapper(&input_text.text().to_string(), count_input.text().parse::<i32>().unwrap() as i32, &"\u{2004}".to_string());
            let markup = format!("{}", result);
            output.buffer().set_text(&markup);
            output_title.set_markup(&format!("<span font=\"15\"><b>Result: {} words</b></span>", count_words(result)));

            // output.set_markup(&markup);
            // gtk_box.append(&output);
        }
    });
    main_window.present();
}