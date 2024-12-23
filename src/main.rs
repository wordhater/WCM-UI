use gtk::{Label};
use gtk::{prelude::*};
use std::collections::linked_list::Iter;
use std::{collections::LinkedList};
use clipboard::{ClipboardProvider, ClipboardContext};
use adw::{Application};
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
            if label == "_"{sel = "_".to_string();}
            break
        }
    }
    return sel;
}

fn getincmode(incbuttons: Rc<RefCell<Vec<gtk::ToggleButton>>>) -> String{
    let mut sel: String = "".to_string();
    for button in incbuttons.borrow().iter(){if button.is_active(){
        let label: &str = match button.label() {
            Some(formatted) => &formatted.clone(),
            None => "null",
        };
        if label == "Hidden (WIP)" {sel = "Hidden".to_string();}else{
            sel = label.to_string();
        }
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
        popup_error("\nUnable to access Clipboard".to_string(), window);
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
fn increase<'a>(input: &'a str, goal: i32, mode: &'a str) -> String {
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
    if mode == "Hidden"{
        if addition > charlist.len() as i32{return "ERROR_001".to_string();}else{
            let rate: f64 = charlist.len() as f64/addition as f64;
            let mut output: String = String::new();
            index = 0;
            for char in charlist{
                output.push_str(&char);
                if index % rate as i32 == 0{
                    output.push_str(&"\u{2061}");
                }
                index += 1
            }
            return output;
        }
    } else if mode == "Before" {
        let mut output: String = "".to_string();
        for i in 1..addition{
            output.push_str("\u{3164} ");
        }
        output.push_str("\u{000D}");
        output.push_str(&input);
        return output;
    }else if mode == "After"{
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

fn modifywrapper<'a>(input: &'a str, count: i32, replacement: &'a String, incmode: &'a String, window: gtk::ApplicationWindow) -> String {
    let init_count: i32 = count_words(input);
    if init_count > count{
        println!("Decrease requested");
        return decrease(input, count, replacement);
    }else if init_count < count {
        println!("increase requested");
        let output: String = increase(input, count, &incmode);
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
        .build();

    let chargroup = char_btn_0.clone().downcast::<gtk::ToggleButton>().unwrap();
    
    char_btn_1.set_group(Some(&chargroup));
    char_btn_2.set_group(Some(&chargroup));

    let charbuttons: Rc<RefCell<Vec<gtk::ToggleButton>>> = Rc::new(RefCell::new(vec![char_btn_0.clone(), char_btn_1.clone(), char_btn_2.clone()]));
    char_btn_0.set_active(true);

    // Increase mode switch

    let inc_label: Label = Label::builder()
        .label("Increase mode (Applies only when aiming for a higher word count):")
        .build(); 
    
    let inc_btn_0: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("Before")
        .build();

    let inc_btn_1: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("After")
        .build();

    let inc_btn_2: gtk::ToggleButton = gtk::ToggleButton::builder()
        .label("Hidden (WIP)")
        .build();

    let incbuttons: Rc<RefCell<Vec<gtk::ToggleButton>>> = Rc::new(RefCell::new(vec![inc_btn_0.clone(), inc_btn_1.clone(), inc_btn_2.clone()]));
    let incgroup = inc_btn_0.clone().downcast::<gtk::ToggleButton>().unwrap();

    inc_btn_1.set_group(Some(&incgroup));
    inc_btn_2.set_group(Some(&incgroup));

    inc_btn_0.set_active(true);
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
        .label("Settings")
        .build();
    let tab1_content: Box = Box::new(gtk::Orientation::Vertical, 5);
    let tab2_content: Box = Box::new(gtk::Orientation::Vertical, 5);
    let tab3_content: Box = Box::new(gtk::Orientation::Vertical, 5);
    gtk_box.append(&title);
    tab3_content.append(&charrow);
    tab3_content.append(&incrow);

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
    tabs.append_page(&tab3_content, Some(&tab3_label));

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
    // clipboard

    let main_window: ApplicationWindow = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(700)
        .child(&gtk_box)
        .title("WCM UI")
        .build();

    let sucessindicator2: Label = sucessindicator.clone();
    let main_window3: gtk::ApplicationWindow = main_window.clone();
    
    getbtn.connect_clicked(move |_getbtn: &Button| {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let potential_clipboardcontent: Result<String, std::prelude::v1::Box<dyn std::error::Error>> = ctx.get_contents();
        match potential_clipboardcontent {
            Ok(content) => {
                let clipcount: i32 = count_words_inc_increase(&content);
                println!("Clipboard content has {} words.", clipcount);
                if content.to_string() == ""{
                    let _ = handle_error("ERROR_004".to_string(), main_window3.clone());
                }else{
                    sucessindicator.set_label(&format!("Clipboard contains {} words", clipcount));
                    sucessindicator.set_tooltip_text(Some(&content));
                }
            }
            Err(error) => {
                println!("Error getting clipboard content: {}", error);
                let _ = handle_error("ERROR_005".to_string(), main_window3.clone());
                sucessindicator.set_label("Error occurred when reading clipboard contents");
            }
        }
    });


    
    let charbuttons2: Rc<RefCell<Vec<gtk::ToggleButton>>> = charbuttons.clone();
    let incbuttons2: Rc<RefCell<Vec<gtk::ToggleButton>>> = incbuttons.clone();
    let main_window1: gtk::ApplicationWindow = main_window.clone();
    let main_window2: gtk::ApplicationWindow = main_window.clone();

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
                        let result: &String = &modifywrapper(&input_text.text().to_string(), count_input.text().parse::<i32>().unwrap() as i32, &selected, &getincmode(incbuttons.clone()), main_window1.clone());
                        let markup: String = format!("{}", result);
                        output.buffer().set_text(&markup);
                        output_title.set_markup(&format!("<span font=\"15\"><b>Result: {} words</b></span>", count_words(result)));
                    }
                }
            }else{
                handle_error("ERROR_002".to_string(), main_window1.clone()); 
            }
        }else{
            handle_error("ERROR_003".to_string(), main_window1.clone());
        }
        
    });
    let output_title_clip2: Label = output_title_clip.clone();
    apply_button_clip.connect_clicked(move |_button: &Button|{
        if count_input_clip.text().to_string() != ""{
            println!("filled");
            if count_input_clip.text().to_string() != ""{
                if count_input_clip.text().parse::<i32>().unwrap() as i32 == 0{let _ = handle_error("ERROR_002".to_string(), main_window2.clone());}else{
                    let selected: String = getcharmode(charbuttons2.clone());
                    let result: &String = &modifywrapper(&sucessindicator2.tooltip_text().unwrap(), count_input_clip.text().parse::<i32>().unwrap() as i32, &selected, &getincmode(incbuttons2.clone()), main_window2.clone());
                    output_title_clip.set_markup(&format!("<span font=\"15\"><b>Result: {} words</b></span>", count_words(result)));
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
