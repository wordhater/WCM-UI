#![windows_subsystem = "windows"]

use gtk::{Label};
use gtk::{prelude::*};
use clipboard::{ClipboardProvider, ClipboardContext};
use gtk::{Application};
use gtk::{glib, ApplicationWindow, Button, Box};
use std::fs;
use std::io::{self, Read, Write};
use std::{fs::File, rc::Rc, path::Path, collections::LinkedList, collections::linked_list::Iter, cell::RefCell};
use unicode_segmentation::UnicodeSegmentation;
use rand::Rng;

const APP_ID: &str = "org.gtk_rs.WCM_UI";

// other functions

fn loadsettings() -> Vec<i32> {
    println!("loading settings");
    let filepath: &Path = Path::new("./conf/settings");
    let _: Result<(), io::Error> = fs::create_dir_all("./conf");
    let display: std::path::Display<'_> = filepath.display();

    let mut new: bool = false;
    let mut file: File = match File::open(&filepath) {
        Err(why) => {
            println!("config not found, setting settings to default");
            let default: &str = "0_0_80_2";
            let mut newfile: File = fs::File::create(filepath).expect("could not create file");
            match newfile.write_all(default.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            }
            let settings: Vec<i32> = vec![0, 0, 80, 2];
            return settings;
        },
        Ok(file) => file,
    };
    println!("opened file");
    // settings format reference: a number for the settings index for each setting
    // char, inc, str, mod
    let mut settings: Vec<i32> = Vec::new();

    let mut settings_str: String = String::new();
    match file.read_to_string(&mut settings_str){
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, settings_str),
    }
    // parse
    let mut tmp_chars: String = "".to_string();
    for char in settings_str.split("") {
        if char == "_"{
            settings.push(tmp_chars.parse().unwrap());
            tmp_chars = "".to_string();
        }else{
            tmp_chars += char;
        }
    }
    settings.push(tmp_chars.parse().unwrap());
    return settings;
}

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
    if ((a % b) + b) % b == 0{
        return true;
    }else{return false;}
}

fn count_words_inc_increase(input: &str) -> i32 {
    let parts: std::str::Split<'_, &str> = input.split(" ");
    let mut count: i32 = 0;
    for part in parts{
            for s2 in part.split("\u{3164}"){
                count += 1;
            }
    }
    count += input.chars().filter(|c| *c == '\u{3164}').count() as i32;
    return count;
}

fn is_all_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

// Settings readers

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
            if label == "U+205F"{sel = "\u{205f}".to_string();}
            if label == "U+2004"{sel = "\u{2004}".to_string();}
            if label == "Debug Mode"{sel = "_".to_string();}
            break
        }
    }
    return sel;
}

fn getincmode(incbuttons: Rc<RefCell<Vec<gtk::ToggleButton>>>) -> i32{
    let mut sel: i32 = 0;
    for button in incbuttons.borrow().iter(){if button.is_active(){
        let label: &str = match button.label() {
            Some(formatted) => &formatted.clone(),
            None => "null",
        };
        if label == "Increase Mode v2" {sel = 2;
        }else if label == "Before"{
            sel = 0;
        }else if label == "After" {
            sel = 1;
        }
        break
        }
    }
    return sel;
}
fn getAImode(AImode_btns:  Rc<RefCell<Vec<gtk::ToggleButton>>>) -> i8{
    let mut sel: i8 = 0;
    for button in AImode_btns.borrow().iter() {
        if button.is_active(){
            let label: &str = match button.label() {
                Some(formatted) => &formatted.clone(),
                None => "null",
            };
            // add extra modes here
            if label == "Homoglyphs"{sel = 1;}
            if label == "Word Merging"{sel = 2;}
            if label == "Both (Very Effective)"{sel = 3;}
            break
        }
    }
    return sel;
}

// Error prompter

fn popup_error(message: String, window: gtk::ApplicationWindow){
    let dialog = gtk::MessageDialog::new(
            
        Some(&window),
        gtk::DialogFlags::empty(),
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        message
    );

    dialog.set_decorated(false);
    dialog.show();

    dialog.connect_response(|dialog, response| {
        if response == gtk::ResponseType::Ok {
            dialog.close(); 
        }
    });
}

fn handle_error(code: String, window: gtk::ApplicationWindow) -> bool {
    if code == "ERROR_001"{
        popup_error("\nToo large of an increase requested for hidden mode".to_string(), window);
        return true;
    }else if code == "ERROR_002"{
        popup_error("\nInvalid Number Entered".to_string(), window);
        return true;
    }else if code == "ERROR_003"{
        popup_error("\nNo Text Entered".to_string(), window);
        return true;
    }else if code == "ERROR_004"{
        popup_error("\nInvalid Clipboard Contents".to_string(), window);
        return true;
    }else if code == "ERROR_005"{
        popup_error("\nUnable to Access Clipboard".to_string(), window);
        return true;
    }else if code == "ERROR_006"{
        popup_error("\nNo Processed Text to Copy".to_string(), window);
        return true;
    }else if code == "ERROR_007"{
        popup_error("\nNot Enough Words for Word Merging".to_string(), window);
        return true;
    }
    return false;
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
            if modulus_i32(i, spacerate) {
                output.push_str(" ");
            }else {
                output.push_str(replacement);
            }
        }
        return output;
    }
}

// increase
fn increase<'a>(input: &'a str, goal: i32, mode: i32) -> String {
    let incchar: String = "\u{2061}".to_string();
    let parts: std::str::Split<'_, &str> = input.split("");
    let charcount: i32 = input.len() as i32;
    let mut charlist: LinkedList<String> = LinkedList::new();
    let mut index: i32 = 0;
    for part in parts{
        if index != 0 {if index != charcount+1{
            charlist.push_back(part.to_string());
        }}
        index += 1;
    }
    let addition: i32 = goal-count_words(input);
    // hidden mode
    if mode == 2{
        if addition > count_words(&input) as i32{return "ERROR_001".to_string();}else{
            let rate: i32 = (count_words(&input)/addition) as i32;
            let mut output: String = String::new();
            let parts: std::str::Split<'_, &str> = input.split(" ");
            let mut words:LinkedList<String> = LinkedList::new();
            for part in parts{words.push_back(part.to_string());}
            index = 0;
            for word in words.clone(){
                output.push_str(&word);
                if index == words.len() as i32 -1 {break}
                if modulus_i32(index, rate){
                    println!("char");
                    output.push_str(&"\u{3164}");
                }else{
                    output.push_str(&"  ");
                }
                index += 1
            }
            return output;
        }
    } else if mode == 0 {
        let mut output: String = "".to_string();
        for i in 1..addition{
            output.push_str("\u{3164} ");
        }
        output.push_str("\u{000D}");
        output.push_str(&input);
        return output;
    }else if mode == 1{
        let mut output: String = "".to_string();
        output.push_str(&input);
        output.push_str("\u{000D}");
        for i in 1..addition{
            output.push_str("\u{3164} ");
        }
        return output;
    }
    return input.to_string();
}

fn anti_ai_detection<'a>(input: &'a str, strength: i32, mode: i8) -> String{
    // modes 1=homoglyphs, 2=homoglyphs+word merging, 3=word merging
    let replacement: String = "\u{205f}".to_string();
    let chars_from: Vec<&str> = vec!["a", "c", "d", "e", "h", "i", "j", "o", "p", "x", "y"];
    let chars_to: Vec<&str> = vec!["\u{0430}","\u{0441}","\u{0501}","\u{0435}","\u{04bb}","\u{0456}","\u{0458}","\u{03bf}","\u{0440}","\u{0445}","\u{0443}"];
    let length: i32 = input.graphemes(true).count() as i32;
    println!("char count: {}", length);
    let mut out = "".to_string();

    if (mode == 1)|(mode == 3){
        for segment in input.split(""){
            println!("char: {}", segment);
            if chars_from.contains(&segment){
                if rand::thread_rng().gen_range(1..101) < strength{
                    let index: usize = chars_from.iter().position(|&r| r == segment).unwrap();
                    out += chars_to[index];
                }
            }else{
                out += segment;
            }

        }
    }else{
        out = input.to_string();
    }
    if(mode == 2)|(mode == 3){
        let init_count: i32 = count_words(&out);
        if init_count == 1{
            return "ERROR_007".to_string();
        }
        let target: i32 = ((init_count as f32 /100.0)*(100 - strength) as f32).ceil() as i32;
        let rate: i32 =  ((init_count as f32 -1.0)/(init_count-target) as f32).ceil() as i32;
        let mut output = "".to_string();
        let mut words:LinkedList<String> = LinkedList::new();
        let parts: std::str::Split<'_, &str> = out.split(" ");
        for part in parts{words.push_back(part.to_string());}
        if target >= init_count/2 {
            for i in 0..init_count{
                output.push_str(&get_item_by_index_str(&words, i as usize));
                if modulus_i32(i, rate){
                    output.push_str(&replacement);
                }else{
                    output.push_str(" ");
                }
            }
        }else{
            let spacerate: i32 = (init_count -1)/(init_count - (init_count-target));
            for i in 0..init_count{
                output.push_str(&get_item_by_index_str(&words, i as usize));
                if modulus_i32(i, spacerate) {
                    output.push_str(" ");
                }else {
                    output.push_str(&replacement);
                }
            }
        }
        out = output
    }
    return out;
}

fn modifywrapper<'a>(input: &'a str, count: i32, replacement: &'a String, incmode: i32, window: gtk::ApplicationWindow) -> String {
    let init_count: i32 = count_words(input);
    if init_count > count{
        println!("Decrease requested");
        return decrease(input, count, replacement);
    }else if init_count < count {
        println!("increase requested");
        let output: String = increase(input, count, incmode);
        if handle_error(output.clone(), window){
            return "An Error occured".to_owned()
        }else{
            return output;
        }
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

    // load settings
    let settings: Vec<i32> = loadsettings();

    let gtk_box: Box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // for some reason it errors if I create multiple instances on the same element in the view, there is no better way I can find to fix this mess

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

    // Anti-AI detection

    let strength_slider = gtk::Scale::with_range(gtk::Orientation::Horizontal, 5.0, 100.0, 1.0);
    strength_slider.set_draw_value(true);
    strength_slider.set_width_request(300);
    let strength_label = gtk::Label::builder()
        .label("\nStrength:")
        .build();
    
    let AI_left: Box = Box::new(gtk::Orientation::Vertical, 5);
    AI_left.set_width_request(300);
    let AI_right: Box = Box::new(gtk::Orientation::Vertical, 5);
    AI_right.set_width_request(300);

    let strengthrow: Box = Box::new(gtk::Orientation::Horizontal, 5);
    
    strengthrow.append(&strength_label);
    strengthrow.append(&strength_slider);
    strength_slider.set_value(settings[2] as f64);
    let strength_slider2: gtk::Scale = strength_slider.clone();

    AI_left.append(&strengthrow);
    AI_left.append(&gtk::Separator::builder().margin_top(12).margin_bottom(5).build());
    
    // mode selection
    
    let mode_header = Label::builder().label("Modifications").build();
    mode_header.set_halign(gtk::Align::Center);
    AI_left.append(&mode_header);

    let AI_mode_btn_0: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("Homoglyphs")
        .build();

    let AI_mode_btn_1: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("Word Merging")
        .build();

    let AI_mode_btn_2: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("Both (Very Effective)")
        .build();
    AI_mode_btn_2.set_active(true);
    let AI_mode_layout: Box = Box::new(gtk::Orientation::Vertical, 5);
    AI_mode_layout.append(&AI_mode_btn_0);
    AI_mode_layout.append(&AI_mode_btn_1);
    AI_mode_layout.append(&AI_mode_btn_2);
    
    let AI_mode_group: gtk::ToggleButton = AI_mode_btn_0.clone().downcast::<gtk::ToggleButton>().unwrap();

    AI_mode_btn_1.set_group(Some(&AI_mode_group));
    AI_mode_btn_2.set_group(Some(&AI_mode_group));

    let AI_mode_buttons: Rc<RefCell<Vec<gtk::ToggleButton>>> = Rc::new(RefCell::new(vec![AI_mode_btn_0.clone(), AI_mode_btn_1.clone(), AI_mode_btn_2.clone()]));

    AI_left.append(&AI_mode_layout);
    if settings[3] == 0 {
        AI_mode_btn_0.set_active(true);
    }else if settings[3] == 1 {
        AI_mode_btn_1.set_active(true);
    }else if settings[3] == 2 {
        AI_mode_btn_2.set_active(true);
    }
    // paste button

    let AI_paste_button: Button = Button::builder()
        .label("Get clipboard contents")
        .build();

    AI_right.append(&AI_paste_button);

    let AI_sucessindicator: Label = Label::builder()
        .label("No loaded clipboard contents")
        .tooltip_text("Click Get Clipboard Contents to get text")
        .build();

    AI_right.append(&AI_sucessindicator);
    AI_right.append(&gtk::Separator::builder().margin_top(5).margin_bottom(5).build());

    // apply
    let apply_anti_AI: Button = Button::builder()
        .label("Apply and copy changes")
        .build();

    AI_right.append(&apply_anti_AI);


    // char buttons
    let char_label: Label =  Label::builder()
        .label("Character selection: ")
        .build();
    let char_btn_0: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("U+205F")
        .tooltip_text("The default and most reliable way of modifying text length. Only switch this to 'U+2004' if it not working.")
        .build();

    let char_btn_1: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("U+2004")
        .tooltip_text("For in-case 'U+205F' isn't working in modifying the word count correctly; only use if it is not working.")
        .build();

    let char_btn_2: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("Debug Mode")
        .tooltip_text("Makes use of underscores to more easily be able to identify bugs.")
        .build();

    let chargroup: gtk::ToggleButton = char_btn_0.clone().downcast::<gtk::ToggleButton>().unwrap();
    
    char_btn_1.set_group(Some(&chargroup));
    char_btn_2.set_group(Some(&chargroup));

    let charbuttons: Rc<RefCell<Vec<gtk::ToggleButton>>> = Rc::new(RefCell::new(vec![char_btn_0.clone(), char_btn_1.clone(), char_btn_2.clone()]));
    if settings[0] == 0{
        char_btn_0.set_active(true);
    }else if settings[0] == 1 {
        char_btn_1.set_active(true);
    }else if settings[0] == 2 {
        char_btn_2.set_active(true);
    }

    // Increase mode switch

    let inc_label: Label = Label::builder()
        .label("Increase Mode:")
        .tooltip_text("This setting is automatically turned on when a higher number is entered than what is in your clipboard/the text editor.")
        .build(); 
    
    let inc_btn_0: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("Before")
        .tooltip_text("As 'Increase Mode' is a work in progress, this only changes whether it adds all the extra words at the very beginning or at the end of the text.")
        .build();

    let inc_btn_1: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("After")
        .tooltip_text("As 'Increase Mode' is a work in progress, this only changes whether it adds all the extra words at the very beginning or at the end of the text.")
        .build();

    let inc_btn_2: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("Increase Mode v2")
        .tooltip_text("This setting makes use of a mostly untested character, however keeps the changes within the text content, Check the text before submitting")
        .build();

    let incbuttons: Rc<RefCell<Vec<gtk::ToggleButton>>> = Rc::new(RefCell::new(vec![inc_btn_0.clone(), inc_btn_1.clone(), inc_btn_2.clone()]));
    let incgroup = inc_btn_0.clone().downcast::<gtk::ToggleButton>().unwrap();

    inc_btn_1.set_group(Some(&incgroup));
    inc_btn_2.set_group(Some(&incgroup));

    if settings[1] == 0{
        inc_btn_0.set_active(true);
    }else if settings[1] == 1 {
        inc_btn_1.set_active(true);
    }else if settings[1] == 2 {
        inc_btn_2.set_active(true);
    }
    // save settings button
    let savebtn: Button = Button::builder()
        .label("Save current settings as default")
        .build();
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

    let incrow: Box = Box::new(gtk::Orientation::Horizontal, 5);
    incrow.set_margin_top(5);
    incrow.set_margin_bottom(5);
    
    incrow.append(&inc_label);
    incrow.append(&inc_btn_0);
    incrow.append(&inc_btn_1);
    incrow.append(&inc_btn_2);

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
    let tab3_label: Label = Label::builder()
        .label("Anti-AI Detection Tools")
        .build();
    let tab4_label: Label = Label::builder()
        .label("Settings")
        .build();

    let tab1_content: Box = Box::new(gtk::Orientation::Vertical, 5);
    let tab2_content: Box = Box::new(gtk::Orientation::Vertical, 5);
    let tab3_content: Box = Box::new(gtk::Orientation::Vertical, 5);
    let tab4_content: Box = Box::new(gtk::Orientation::Vertical, 5);

    // anti ai tab builder
    
    let tab4_column_container: Box = Box::new(gtk::Orientation::Horizontal, 5);
    tab4_column_container.append(&AI_left);
    tab4_column_container.append(&gtk::Separator::builder().orientation(gtk::Orientation::Vertical).build());
    tab4_column_container.append(&AI_right);
    
    tab3_content.append(&tab4_column_container);
    

    // other builders
    gtk_box.append(&title);
    tab4_content.append(&gtk::Label::builder().label("<b>Hover over settings to view more details</b>").use_markup(true).build());
    tab4_content.append(&charrow);
    tab4_content.append(&incrow);
    tab4_content.append(&savebtn);

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
        .label("Get Clipboard Contents")
        .margin_top(10)
        .build();
    let sucessindicator: Label = Label::builder()
        .label("No loaded clipboard contents.")
        .tooltip_text("Click Get Clipboard Contents to get text")
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
        .tooltip_text("Nothing yet.")
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
    tabs.append_page(&tab3_content, Some(&tab3_label));
    tabs.append_page(&tab4_content, Some(&tab4_label));

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
                let mut chars: std::str::Chars<'_> = text.chars();
                chars.next_back();
                let output: &str = chars.as_str();
                println!("{}", output);
                _count_input.set_text(&output);
            }
        });
    }
    // clipboard

    let main_window: ApplicationWindow = gtk::ApplicationWindow::builder()
        .application(app)
        // .default_width(1200)
        // .default_height(700)
        .child(&gtk_box)
        .title("WCM UI")
        .build();

    let main_window3: gtk::ApplicationWindow = main_window.clone();
    
    let clip_btn_logic = move |_getbtn: &Button| {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let potential_clipboardcontent: Result<String, std::prelude::v1::Box<dyn std::error::Error>> = ctx.get_contents();
        match potential_clipboardcontent {
            Ok(content) => {
                let clipcount: i32 = count_words_inc_increase(&content);
                println!("Clipboard content has {} words.", clipcount);
                if content.to_string() == ""{
                    let _ = handle_error("ERROR_004".to_string(), main_window3.clone());
                }else{
                    if tabs.current_page() == Some(1){
                        sucessindicator.set_label(&format!("Clipboard contains {} words", clipcount));
                    }else if tabs.current_page() == Some(2){
                        AI_sucessindicator.set_label(&format!("Clipboard contains {} words", clipcount));
                    }
                    _getbtn.set_tooltip_text(Some(&content));
                    
                }
            }
            Err(error) => {
                println!("Error getting clipboard content: {}", error);
                let _ = handle_error("ERROR_005".to_string(), main_window3.clone());
                _getbtn.set_label("Error occurred when reading clipboard contents");
            }
        }
    };
    getbtn.connect_clicked(clip_btn_logic.clone());
    AI_paste_button.connect_clicked(clip_btn_logic.clone());


    // cloning variables so I can reference them    
    let charbuttons2: Rc<RefCell<Vec<gtk::ToggleButton>>> = charbuttons.clone();
    let charbuttons3: Rc<RefCell<Vec<gtk::ToggleButton>>> = charbuttons.clone();
    let incbuttons2: Rc<RefCell<Vec<gtk::ToggleButton>>> = incbuttons.clone();
    let incbuttons3: Rc<RefCell<Vec<gtk::ToggleButton>>> = incbuttons.clone();
    let AI_mode_buttons2: Rc<RefCell<Vec<gtk::ToggleButton>>> = AI_mode_buttons.clone();
    let main_window1: gtk::ApplicationWindow = main_window.clone();
    let main_window2: gtk::ApplicationWindow = main_window.clone();
    let main_window3: gtk::ApplicationWindow = main_window.clone();
    let main_window4: gtk::ApplicationWindow = main_window.clone();


    apply_button.connect_clicked(move |_button: &Button| {
        println!("modify btn clicked");
        if count_input.text().to_string() != ""{
            println!("filled");
            if count_input.text().to_string() != ""{
                if count_input.text().parse::<i32>().unwrap() as i32 == 0{let _ = handle_error("ERROR_002".to_string(), main_window1.clone());}else{
                    let selected: String = getcharmode(charbuttons.clone());

                    if (input_text.text().to_string().len() as i32 == 0) | (count_input.text().to_string().len() as i32 == 0){
                        println!("no text")
                    }else{
                        let result: &String = &modifywrapper(&input_text.text().to_string(), count_input.text().parse::<i32>().unwrap() as i32, &selected, getincmode(incbuttons.clone()), main_window1.clone());
                        let markup: String = format!("{}", result);
                        output.buffer().set_text(&markup);
                        output_title.set_markup(&format!("<span font=\"15\"><b>Result: {} words</b></span>", count_words_inc_increase(result)));
                    }
                }
            }else{
                handle_error("ERROR_002".to_string(), main_window1.clone()); 
            }
        }else{
            handle_error("ERROR_003".to_string(), main_window1.clone());
        }
        
    });

    apply_anti_AI.connect_clicked(move |_button: &Button| {
        println!("modify btn clicked");
        let strength: i16 = strength_slider.value() as i16;
        println!("{}", strength);
        let maybetext: Option<glib::GString> = AI_paste_button.tooltip_text();
        let text: &str = match maybetext {
            Some(formatted) => &formatted.clone(),
            None => "ERROR_003",
        };
        if handle_error(text.to_string(), main_window4.clone()){
            println!("lack of text")
        }else{
            let result = anti_ai_detection(&text, strength as i32-1, getAImode(AI_mode_buttons.clone()));
            if handle_error(result.clone(), main_window4.clone()){
                println!("Too Few words");
            }else{
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(result).unwrap();
            }
        }

    });

    let output_title_clip2: Label = output_title_clip.clone();
    apply_button_clip.connect_clicked(move |_button: &Button|{
        if count_input_clip.text().to_string() != ""{
            println!("filled");
            if count_input_clip.text().to_string() != ""{
                if count_input_clip.text().parse::<i32>().unwrap() as i32 == 0{let _ = handle_error("ERROR_002".to_string(), main_window2.clone());}else{
                    let selected: String = getcharmode(charbuttons2.clone());
                    let result: &String = &modifywrapper(&getbtn.tooltip_text().unwrap(), count_input_clip.text().parse::<i32>().unwrap() as i32, &selected, getincmode(incbuttons2.clone()), main_window2.clone());
                    output_title_clip.set_markup(&format!("<span font=\"15\"><b>Result: {} words</b></span>", count_words_inc_increase(result)));
                    output_title_clip.set_tooltip_text(Some(&result));
                }
            }else{
                handle_error("ERROR_002".to_string(), main_window2.clone());
            }
        }else{
            handle_error("ERROR_003".to_string(), main_window2.clone());
        }
    });

    // copy btn
    copy_result_btn.connect_clicked(move |_btn: &Button|{
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let text: &str = match output_title_clip2.tooltip_text() {
            Some(formatted) => &formatted.clone(),
            None => "Nothing yet.",
        };
        if text == "Nothing yet."{handle_error("ERROR_006".to_string(), main_window3.clone());}else{
            ctx.set_contents(text.to_owned()).unwrap();
        }
    });
    savebtn.connect_clicked(move |_btn: &Button| {
        println!("saving setings");
        // charbtns
        let charmode: String = getcharmode(charbuttons3.clone());
        let mut charindex: i32 = 0;
        if charmode == "\u{205f}".to_string(){
            charindex = 0;
        }else if charmode == "\u{2004}".to_string() {
            charindex = 1;
        }else if charmode == "_".to_string() {
            charindex = 2;
        }

        // increase mode
        let incmode: i32 = getincmode(incbuttons3.clone());
        // strength
        let strength: i32 = strength_slider2.value().ceil() as i32;
        let AI: i32 = getAImode(AI_mode_buttons2.clone()) as i32 -1;
        let content: String = format!("{}_{}_{}_{}", charindex, incmode, strength, AI);
        println!("content: {}", content);
        
        let filepath: &Path = Path::new("./conf/settings");
        let _: Result<(), io::Error> = fs::create_dir_all("./conf");
        let display: std::path::Display<'_> = filepath.display();
        let mut newfile: File = fs::File::create(filepath).expect("could not create file");
        match newfile.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    });

    main_window.present();
}
