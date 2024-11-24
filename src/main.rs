use gtk::{glib::Char, Label};
use gtk::{prelude::*, Separator};
use gio::{glib::clone, prelude::*};
use std::collections::linked_list::Iter;
use std::{collections::LinkedList};
use clipboard::{ClipboardProvider, ClipboardContext};
use adw::Application;
use gtk::{glib, ApplicationWindow, Button, SpinButton, Box};
use std::rc::Rc;
use std::cell::RefCell;

const APP_ID: &str = "org.gtk_rs.WCM_UI";

// other functions

fn get_item_by_index_str(list: &LinkedList<String>, index: usize) -> String {
    let mut iter: Iter<String> = list.iter();
    for _ in 0..index {
        if iter.next().is_none() {
            return "Index out of bounds".to_string();
        }
    }
    iter.next().map(|s: &String| s.clone()).unwrap_or_else(|| "Index out of bounds".to_string())
}

fn count_words(input: &str) -> i32 {
    let parts: std::str::Split<'_, &str> = input.split(" ");
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
    let parts: std::str::Split<'_, &str> = input.split(" ");
    let mut words:LinkedList<String> = LinkedList::new();
    for part in parts{
        words.push_back(part.to_string());
    }
    let mut output: String = "".to_string();
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
    // Setup chars
    let mut addCHARS:LinkedList<String> = LinkedList::new();
    addCHARS.push_back("\u{2000}".to_string());
    addCHARS.push_back("\u{2002}".to_string());
    addCHARS.push_back("\u{205f}".to_string());//best
    addCHARS.push_back("\u{2004}".to_string());//potential inconsistency
    addCHARS.push_back("TEST".to_string());//testing
    // Create a new application

    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(bootGUI);

    //Run the application
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

    // mode selection


    // char buttons
    let char_label =  Label::builder()
        .label("Char selection (SELECT ONLY ONE AT A TIME): ")
        .build();
    let char_btn_0 = gtk::ToggleButton::builder()
        .label("U+205F")
        .build();

    let char_btn_1 = gtk::ToggleButton::builder()
        .label("U+2004")
        .build();

    let char_btn_2 = gtk::ToggleButton::builder()
        .label("_")
        // .group("yee")
        .build();

    let charbuttons: Rc<RefCell<Vec<gtk::ToggleButton>>> = Rc::new(RefCell::new(vec![char_btn_0.clone(), char_btn_1.clone(), char_btn_2.clone()]));
    char_btn_0.set_active(true);
    // layouts

    let row1: Box = Box::new(gtk::Orientation::Horizontal, 5);
    row1.set_margin_top(5);
    row1.set_margin_bottom(5);

    let charrow: Box = Box::new(gtk::Orientation::Horizontal, 5);
    charrow.set_margin_top(5);
    charrow.set_margin_bottom(5);

    charrow.append(&char_label);
    charrow.append(&char_btn_0);
    charrow.append(&char_btn_1);
    charrow.append(&char_btn_2);

    row1.append(&count_input_label);
    row1.append(&count_input);

    // tabs

    let tabs: gtk::Notebook = gtk::Notebook::builder().build();
    
    let tab1_label: Label = Label::builder()
        .label("Text")
        .build();
    let tab2_label: Label = Label::builder()
        .label("Clipboard")
        .build();
    let tab1_content: Box = Box::new(gtk::Orientation::Vertical, 5);
    let tab2_content: Box = Box::new(gtk::Orientation::Vertical, 5);

    gtk_box.append(&title);
    gtk_box.append(&charrow);

    tab1_content.append(&input_text);
    tab1_content.append(&horizontal_separator_0);
    tab1_content.append(&word_count);
    tab1_content.append(&horizontal_separator_1);
    tab1_content.append(&row1);
    tab1_content.append(&horizontal_separator_2);
    tab1_content.append(&apply_button);
    tab1_content.append(&horizontal_separator_3);
    tab1_content.append(&output_title);
    tab1_content.append(&output);
    
    // Clipboard menu contents

    let getbtn = Button::builder()
        .label("Get clipboard contents")
        .build();
    let sucessindicator = Label::builder()
        .label("No loaded clipboard contents")
        .tooltip_text("yee")
        // .has_tooltip(false)
        .build();
    let count_input_clip: gtk::Entry = gtk::Entry::builder()
        .build();
    let count_input_label_clip: Label = gtk::Label::builder()
        .label("Word Count Goal:")
        .build();

    let row2: Box = Box::new(gtk::Orientation::Horizontal, 5);
    row2.set_margin_top(5);
    row2.set_margin_bottom(5);

    row2.append(&count_input_label_clip);
    row2.append(&count_input_clip);

    tab2_content.append(&getbtn);
    tab2_content.append(&sucessindicator);
    tab2_content.append(&horizontal_separator_4);
    tab2_content.append(&row2);

    tabs.append_page(&tab1_content, Some(&tab1_label));
    tabs.append_page(&tab2_content, Some(&tab2_label));

    gtk_box.append(&tabs);

    // Interaction
    input_text.connect_changed(move |_text: &gtk::Text| {
        if _text.text_length() == 0 {word_count.set_label("Current Word Count: 0");}else{
            let mut lable: String = "Current Word Count: ".to_string();
            lable.push_str(&count_words(&_text.text()).to_string());

            word_count.set_label(&lable);
        }
    });

    // jank method of ensuring count input is a number
    let num_inputs: Rc<RefCell<Vec<gtk::Entry>>> = Rc::new(RefCell::new(vec![count_input.clone(), count_input_clip.clone()]));
    for _input in num_inputs.borrow().iter()  {
        _input.connect_changed(|_count_input: &gtk::Entry| {
            println!("update");
            let mut text: &str = &_count_input.text().to_string();
            if text.chars().all(char::is_numeric){
                println!("all numbers")
            }else{
                println!("letters detected");
                let mut output: String = text.to_string();
                output.pop();
                println!("{}", output);
                _count_input.set_text(&output);
            }
        });
    }
    // toggle controller - this never worked
    // for button in charbuttons.borrow().iter() {
    //     let charbuttons: Rc<RefCell<Vec<gtk::ToggleButton>>> = Rc::clone(&charbuttons);
    //     button.connect_toggled(move |btn| {
    //         if btn.is_active() {
    //             // Deactivate all other buttons when this one is activated
    //             for other_button in charbuttons.borrow().iter() {
    //                 other_button.set_active(false);
    //             }
    //             btn.activate();
    //         } else{println!("deactivate")}
    //     });
    // }

    // clipboard
    getbtn.connect_clicked(move |_getbtn| {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let potential_clipboardcontent: Result<String, std::prelude::v1::Box<dyn std::error::Error>> = ctx.get_contents();
        match potential_clipboardcontent {
            Ok(content) => {
                let clipcount: i32 = count_words(&content);
                println!("Clipboard content has {} words.", clipcount);
                sucessindicator.set_label(&format!("Clipboard contains {} words", clipcount));
                sucessindicator.set_tooltip_text(Some(&content));

            }
            Err(error) => {
                println!("Error getting clipboard content: {}", error);
                sucessindicator.set_label("Error occurred when reading clipboard contents");
            }
        }
    });

    let main_window: ApplicationWindow = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(700)
        .child(&gtk_box)
        .title("WCM UI")
        .build();

    apply_button.connect_clicked(move |_button| {
        println!("modify btn clicked");
        if count_input.text().parse::<i32>().unwrap() as i32 == 0{println!("no number input")}else{
            let mut char = "".to_string();
            // get selected char
            for button in charbuttons.borrow().iter() {
                if button.is_active(){
                    let label: &str = match button.label() {
                        Some(formatted) => &formatted.clone(),
                        None => "null",
                    };
                    // add extra chars here
                    if label == "U+205F"{
                        char = "\u{205f}".to_string();
                        break
                    }
                    if label == "U+2004"{
                        char = "\u{2004}".to_string();
                        break
                    }
                    if label == "_"{
                        char = "_".to_string();
                        break
                    }
                }
            }
            let result: &String = &modifywrapper(&input_text.text().to_string(), count_input.text().parse::<i32>().unwrap() as i32, &char);
            let markup: String = format!("{}", result);
            output.buffer().set_text(&markup);
            output_title.set_markup(&format!("<span font=\"15\"><b>Result: {} words</b></span>", count_words(result)));
        }
    });
    main_window.present();
}