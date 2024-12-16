use gtk::{Label};
use gtk::{prelude::*};
use std::collections::linked_list::Iter;
use std::{collections::LinkedList};
use clipboard::{ClipboardProvider, ClipboardContext};
use adw::Application;
use gtk::{glib, ApplicationWindow, Button, Box};
use std::rc::Rc;
use std::cell::{RefCell};

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

fn modulus_i32(a: i32, b: i32) -> bool{
    println!("result: {}", (((a % b) + b) % b));
    if ((a % b) + b) % b == 0{
        return true;
    }else{return false;}
}

fn count_words_inc_increase(input: &str) -> i32 {
    let parts: std::str::Split<'_, &str> = input.split(" ");
    let mut count: i32 = 0;
    let mut modlist: LinkedList<String> = LinkedList::new();
    for part in parts{
        for segment in part.split("\u{205f}"){
            count += 1;
        }
    }
    return count;
}

fn is_all_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn getcharmode(charbuttons:  Rc<RefCell<Vec<gtk::ToggleButton>>>) -> String{
    // get selected char
    let mut sel: String = "".to_string();
    for button in charbuttons.borrow().iter() {
        if button.is_active(){
            let label: &str = match button.label() {
                Some(formatted) => &formatted.clone(),
                None => "null",
            };
            // add extra chars here
            if label == "U+205F"{
                sel = "\u{205f}".to_string();
                break
            }
            if label == "U+2004"{
                sel = "\u{2004}".to_string();
                break
            }
            if label == "_"{
                sel = "_".to_string();
                break
            }
        }
    }
    return sel;
}

// Modify word count


fn decrease<'a>(input: &'a  str, goal: i32, replacement: &'a str) -> String{
    let init_count: i32 = count_words(input);
    let parts: std::str::Split<'_, &str> = input.split(" ");
    let mut words:LinkedList<String> = LinkedList::new();
    for part in parts{words.push_back(part.to_string());}
    let totaldiff: i32 = init_count - goal;
    let rate: i32 = ((init_count as f32 -1.0)/totaldiff as f32).ceil() as i32;
    let mut output: String = "".to_string();
    if goal >= init_count/2 {
        for i in 0..init_count{
            output.push_str(&get_item_by_index_str(&words, i as usize));
            if modulus_i32(i, rate){
                output.push_str(replacement);
            }else{
                output.push_str(" ");
            }
        }
        return output;
    }else{
        println!("less than half");
        let spacerate: i32 = (init_count -1)/(init_count - totaldiff);
        for i in 0..init_count{
            output.push_str(&get_item_by_index_str(&words, i as usize));
            println!("rate: {}", spacerate);
            println!("words: {}", init_count);
            if modulus_i32(i, spacerate) {
                output.push_str(" ");
                println!("space")
            }else {
                println!("char");
                output.push_str(replacement);
            }
        }
        return output;
    }
}

// increase
fn increase<'a>(input: &'a str, goal: i32) -> String {
    let mode = "prior";
    let incchar: String = "\u{2061}".to_string();
    let parts: std::str::Split<'_, &str> = input.split("");
    let charcount: i32 = input.len() as i32;
    let mut charlist: LinkedList<String> = LinkedList::new();
    let mut index: i32 = 0;
    for part in parts{
        if index != 0 {if index != charcount+1{
            println!("e:{}", part);
            charlist.push_back(part.to_string());
        }}
        index += 1;

    }
    let addition: i32 = goal-count_words(input);
    // hidden mode
    if mode == "hidden"{
        if addition > charlist.len() as i32{println!("ok buddy that's too much of an increase");}else{
            let rate: f64 = charlist.len()as f64/addition as f64;
            let mut output: String = String::new();
            index = 0;
            for char in charlist{
                println!("yee");
                output.push_str(&char);
                if index % rate as i32 == 0{
                    println!("increase");
                    output.push_str(&"\u{2061}");
                    println!("{}", output)
                }
                index += 1
            }
            return output;
        }
    } else if mode == "prior" {
        let mut output: String = "".to_string();
        for i in 1..addition{
            output.push_str("\u{3164} ");
            println!("char")
        }
        output.push_str("\u{000D}");
        output.push_str(&input);
        return output;
    }
    return input.to_string();
}

fn modifywrapper<'a>(input: &'a str, count: i32, replacement: &'a String) -> String {
    let init_count: i32 = count_words(input);
    if init_count > count{
        println!("Decrease requested");
        return decrease(input, count, replacement);
    }else if init_count < count {
        println!("increase requested very broken");
        return increase(input, count);
    } else{
        println!("NO difference...");
        return input.to_string();
    }
}

fn main()  -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(bootGUI);

    //Run the application
    app.run()
}

fn bootGUI(app: &Application){

    let gtk_box: Box = gtk::Box::builder()
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
    let horizontal_separator_5 = gtk::Separator::builder()
        .margin_top(5)
        .margin_bottom(5)
        .build();
    let horizontal_separator_6 = gtk::Separator::builder()
        .margin_top(5)
        .margin_bottom(5)
        .build();
    
    let title: Label = gtk::Label::builder()
        .label("Word Count Modifier UI")
        .use_markup(true)
        .build();
    title.set_markup("<span font=\"15\"><b>Word Count Modifier UI</b></span>");

    let input_text: gtk::Text = gtk::Text::builder()
        .placeholder_text("Enter input text here")
        .margin_bottom(30)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let word_count: Label = gtk::Label::builder()
        .label("Current Word Count: 0")
        .margin_bottom(5)
        .margin_top(5)
        .build();

    let count_input_label: Label = gtk::Label::builder()
        .label("Word Count Goal:")
        .build();
    let count_input: gtk::Entry = gtk::Entry::builder()
        .build();

    let apply_button: Button = gtk::Button::builder()
        .label("Apply")
        .build();
    let output: gtk::TextView = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::Word)
        .build();
    let output_title: Label = Label::builder()
        .label("Result:")
        .use_markup(true)
        .build();
    output_title.set_markup("<span font=\"15\"><b> </b></span>");

    // mode selection


    // char buttons
    let char_label: Label =  Label::builder()
        .label("Char selection (SELECT ONLY ONE AT A TIME): ")
        .build();
    let char_btn_0: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("U+205F")
        .build();

    let char_btn_1: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("U+2004")
        .build();

    let char_btn_2: gtk::ToggleButton = gtk::ToggleButton::builder()
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

    let getbtn: Button = Button::builder()
        .label("Get clipboard contents")
        .margin_top(10)
        .build();
    let sucessindicator: Label = Label::builder()
        .label("No loaded clipboard contents")
        .tooltip_text("yee")
        // .has_tooltip(false)
        .build();
    let count_input_clip: gtk::Entry = gtk::Entry::builder()
        .build();
    let count_input_label_clip: Label = gtk::Label::builder()
        .label("Word Count Goal:")
        .build();
    let apply_button_clip: Button = gtk::Button::builder()
        .label("Apply")
        .build();
    let output_title_clip: Label = Label::builder()
        .label("Result:")
        .use_markup(true)
        .tooltip_text("nothing yet")
        .build();

    let copy_result_btn: gtk::Button = gtk::Button::builder()
        .label("Copy result to clipboard")
        .build();

    output_title_clip.set_markup("<span font=\"15\"><b> </b></span>");
    let row2: Box = Box::new(gtk::Orientation::Horizontal, 5);
    row2.set_margin_top(5);
    row2.set_margin_bottom(5);

    row2.append(&count_input_label_clip);
    row2.append(&count_input_clip);

    tab2_content.append(&getbtn);
    tab2_content.append(&sucessindicator);
    tab2_content.append(&horizontal_separator_4);
    tab2_content.append(&row2);
    tab2_content.append(&horizontal_separator_5);
    tab2_content.append(&apply_button_clip);
    tab2_content.append(&output_title_clip);
    tab2_content.append(&horizontal_separator_6);
    tab2_content.append(&copy_result_btn);
    

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

    let sucessindicator2: Label = sucessindicator.clone();
    
    getbtn.connect_clicked(move |_getbtn: &Button| {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let potential_clipboardcontent: Result<String, std::prelude::v1::Box<dyn std::error::Error>> = ctx.get_contents();
        match potential_clipboardcontent {
            Ok(content) => {
                let clipcount: i32 = count_words_inc_increase(&content);
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
    
    let charbuttons2: Rc<RefCell<Vec<gtk::ToggleButton>>> = charbuttons.clone();

    apply_button.connect_clicked(move |_button: &Button| {
        println!("modify btn clicked");
        if count_input.text().parse::<i32>().unwrap() as i32 == 0{println!("no number input")}else{
            let selected: String = getcharmode(charbuttons.clone());
            let result: &String = &modifywrapper(&input_text.text().to_string(), count_input.text().parse::<i32>().unwrap() as i32, &selected);
            let markup: String = format!("{}", result);
            output.buffer().set_text(&markup);
            output_title.set_markup(&format!("<span font=\"15\"><b>Result: {} words</b></span>", count_words(result)));
        }
    });
    let output_title_clip2: Label = output_title_clip.clone();
    apply_button_clip.connect_clicked(move |_button: &Button|{
        println!("mod clip");
        if count_input_clip.text().parse::<i32>().unwrap() as i32 == 0{println!("no number")}else{
            let selected: String = getcharmode(charbuttons2.clone());
            let result: &String = &modifywrapper(&sucessindicator2.tooltip_text().unwrap(), count_input_clip.text().parse::<i32>().unwrap() as i32, &selected);
            output_title_clip.set_markup(&format!("<span font=\"15\"><b>Result: {} words</b></span>", count_words(result)));
            output_title_clip.set_tooltip_text(Some(&result));
        }
    });

    // copy btn
    copy_result_btn.connect_clicked(move |_btn|{
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let text: &str = match output_title_clip2.tooltip_text() {
            Some(formatted) => &formatted.clone(),
            None => "null",
        };
        ctx.set_contents(text.to_owned()).unwrap();
    });
    main_window.present();
}