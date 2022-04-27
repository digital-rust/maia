/*
name: maia ['A custom terminal user interface emulator running on crossterm']
version string: 0.0.2
*/

use ansi_term::{Colour};
#[allow(dead_code)]
mod utils;
#[allow(unused_imports)]
use std::io;
use std::usize;
#[allow(unused_imports)]
use std::{
    io::{
        stdout,
        stdin,
        Write,
        Read,
    }
};
#[allow(unused_imports)]
use crossterm::{
    execute,
    cursor,
    terminal,
    event,
    style,

};

pub trait TerminalMenuItem {
    fn enter(&mut self) -> bool; // is ENTER pressed
    fn print(&self);    
}
#[derive(Debug)]
pub struct MenuItem {
    pub val: String,
}
impl MenuItem {
    pub fn new_numeral(default: String) -> MenuItem {
        MenuItem {
            val: default,
        }
    }
    fn enter(&mut self) -> bool {
        true
    }
    fn print(&self) {
        print!("{}", self.val);
    }
    fn modify_value(&mut self, new_value: String) {
         self.val = new_value;
        }
}

// REDEFINE
fn offset_y(x: u16, offset_y: i16) {
    execute!(stdout(), cursor::MoveTo(x, (cursor::position().unwrap().1 as i16 + offset_y) as u16)).unwrap();
}
fn move_to_x(x: u16) {
    execute!(stdout(), cursor::MoveTo(x, cursor::position().unwrap().1)).unwrap();
}
fn move_to_y(y: u16) {
    execute!(stdout(), cursor::MoveTo(cursor::position().unwrap().0, y)).unwrap();
}

pub fn longest_name_calc(menu_items: &mut Vec<(String, MenuItem)>) -> u16 {
        // find longest name to align items in the menu properly
        let mut longest_name: u16 = 0;
        for (name, _) in menu_items {
            if name.len() as u16 > longest_name {
                longest_name = name.len() as u16;
            }
        }
    return longest_name
}

pub fn print_menu(max_namelen: u16, selection: usize, menu_items: &mut Vec<(String, MenuItem)>, initial_print: bool) {

    if initial_print == true {
        // clear term screen at initial print and move to cursor position
        execute!(stdout(),
        terminal::Clear(terminal::ClearType::FromCursorUp)).unwrap();
        move_to_y(0); // move to top of terminal screen

        // print logo
        utils::pr_logo();

        // display starting prompt -> change to a custom cursor tick & color -> create the cursor in a function and initialize starting screen in main.
        print!("{}", menu_items[0].0);
        move_to_x(8);
        menu_items[0].1.print();
        println!();
        println!();
        print!("+ {}", menu_items[1].0);

        move_to_x(max_namelen + 7);
        menu_items[1].1.print();
        for i in 2..menu_items.len() {
            println!();
            move_to_x(2);
            print!("{}", menu_items[i].0);
            move_to_x(max_namelen + 7);
            //if items[i].0 != "Exit" {
            //    items[i].1.print();
            //}
            menu_items[i].1.print();
        }
        move_to_x(0);

    } else {

        // TODO: all cursor move cmds into utils
        execute!(stdout(),
        cursor::MoveUp(menu_items.len() as u16), //menu_items.len() as u16 + 1
        terminal::Clear(terminal::ClearType::FromCursorDown),
        terminal::Clear(terminal::ClearType::FromCursorUp)
        ).unwrap();

        // display starting prompt -> change to a custom cursor tick & color -> create the cursor in a function and initialize starting screen in main.
        print!("{}", menu_items[0].0);
        move_to_x(8);
        menu_items[0].1.print();
        println!();

        for i in 1..menu_items.len() {
            println!();

            if i == selection { 
                print!("+ {}", menu_items[selection].0);
                move_to_x(max_namelen + 7);
                menu_items[selection].1.print();
            } else {
                move_to_x(2);
                print!("{}", menu_items[i].0);
                move_to_x(max_namelen + 7);
                menu_items[i].1.print();
            }
            move_to_x(0);
        }
        
    }
}
pub fn engage(items: &mut Vec<(String, MenuItem)>,selection: &mut usize, _initial_print: bool) -> usize {
    execute!(stdout(), cursor::Hide).unwrap();
    // handle 0 items in the menu - this is for debugging only, no need for it on release
    if items.len() == 0 {
        panic!("Cannot have 0 items.");}

    let mut selection:usize = *selection;
    let longest_name: u16 = longest_name_calc(items);
    // Main Loop
    loop {
        terminal::enable_raw_mode().unwrap();
        // Handle Enter
        let _no_modifiers = event::KeyModifiers::empty();
        match event::read().unwrap() {

            // Handle ESCAPE - TODO

            // Handle Up
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Up,
                modifiers: _no_modifiers,
            }) => {
                offset_y(0, (selection as i16) - (items.len() as i16) + 1);
                print!(" ");
                if selection == 1 { // first to last element |'0' if no status prompt
                    selection = items.len() - 1;
                    offset_y(0, selection as i16 -1);
                }
                else {              // any other case
                    selection -= 1;
                    offset_y(0, -1);
                }
                print!("+");
                offset_y(0, (items.len() as i16) - (selection as i16) - 1);
            }

            // Handle Down
            event::Event::Key(event::KeyEvent {
                
                code: event::KeyCode::Down,
                modifiers: _no_modifiers,
            }) => {
                
                offset_y(0, (selection as i16) - (items.len() as i16) + 1);
                print!(" ");
                if selection == items.len() - 1 { // last to first element
                    selection = 1;
                    offset_y(0, -(items.len() as i16) + 2);
                }
                else { // any other case
                    selection += 1;
                    offset_y(0, 1);
                }
                print!("+");
                offset_y(0, (items.len() as i16) - (selection as i16) - 1);    
            }
            // redraw in case of resize
            event::Event::Resize(_, term_height) => {
                execute!(stdout(), cursor::Hide).unwrap();
                //execute!(stdout(), cursor::DisableBlinking).unwrap();
                if items.len() > (term_height - 1).into() {
                    print!("{}", items[0].0);
                    move_to_x(8);
                    items[0].1.print();
                    println!();
                    println!();
                    print!("+ {}", items[1].0);
                
                    move_to_x(longest_name + 7);
                    items[1].1.print();
                    for i in 2..items.len() {
                        println!();
                        move_to_x(2);
                        print!("{}", items[i].0);
                        move_to_x(longest_name + 7);
                        //if items[i].0 != "Exit" {
                        //    items[i].1.print();
                        //}
                        items[i].1.print();
                    }
                    move_to_x(0);
                    execute!(stdout(), cursor::Hide).unwrap();
                }
            }
            
            // Handle ENTER=SELECT VALUE
            event::Event::Key(event::KeyEvent { //exit out of loop and end program
                code: event::KeyCode::Enter,
                modifiers: _no_modifiers,
            }) => {
                terminal::disable_raw_mode().unwrap();
                //execute!(stdout(), cursor::Hide).unwrap();
                if items[selection].1.enter() {
                    if items[selection].0 == "Exit" {
                        break;
                    }
                    else if items[selection].0 == "status: " {} // ignore status bar
                    else {
                        execute!(stdout(), cursor::Hide).unwrap();
                        //let mut err_flag = true; - error flag can be used with while loop
                        //while err_flag {

                        // reallocate 8 bytes for this buffer.
                        let mut buffer: String = String::new(); // 'reset' buffer

                        // Enter alternate screen here to select value for that parameter
                        //execute!(stdout(), terminal::EnterAlternateScreen).unwrap();

                        // handle resize here as well, because it breaks!


                        // hide cursor for this screen for coolness
                        //execute!(stdout(), cursor::Hide).unwrap();

                        // assign prompt
                        let mut assign_prompt: String = String::new();
                        assign_prompt.push_str("Assign value [f64]: ");
                        let assign_prompt = utils::trim_newline(&mut assign_prompt);
                        
                        execute!(stdout(), cursor::SavePosition).unwrap();
                        offset_y(0, items.len() as i16);
                        print!("{}",assign_prompt);
                        let _=stdout().flush();

                        // need to point to a function that performs 
                        // a position check based on the terminal size
                        // and outputs a reasonable position where the
                        // assign prompt should appear and ask the user.
                        
                        execute!(stdout(), cursor::Hide).unwrap();
                        stdin()
                            .read_line(&mut buffer)
                            .expect("Failed to read from stdin.");

                        let trimmed = buffer.trim();
                        let _input_parse_match: () = match trimmed.parse::<f64>() {
                            Ok(i) => {
                                //err_flag = false;
                                //print!("{}", Style::default().on(Colour::RGB(r, g, b)).paint(" "));

                                let success_modify_status: String = (Colour::RGB(0, 175, 0)).paint("all looks good").to_string();
                                items[selection].1.modify_value(i.to_string());
                                items[0].1.modify_value(success_modify_status);
                                //execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
                            }
                            Err(_e) => {
                                let error_modify_status: String = (Colour::RGB(175, 0, 0)).paint("invalid float literal").to_string();
                                //let error_modify_status_edited = trim_newline_str(&mut error_modify_status);
                                items[0].1.modify_value(error_modify_status);
                            }
                        };
                        //}
                        terminal::enable_raw_mode().unwrap();
                        execute!(stdout(), cursor::RestorePosition).unwrap();
                        offset_y(0, selection as i16);
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    //
    execute!(stdout(),
        cursor::MoveUp(items.len() as u16),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    ).unwrap();
    //offset_y(0, selection as i16);
    return selection
}

fn main() {

    let mut menu_ver_2: Vec<(String, MenuItem)> = vec![
        ("status: ".to_owned(), MenuItem::new_numeral(String::new())),
        ("orbital distance".to_owned(), MenuItem::new_numeral(400.0.to_string())),
        ("required data rate".to_owned(), MenuItem::new_numeral(401.0.to_string())),
        ("receiver frequency".to_owned(), MenuItem::new_numeral(401.0.to_string())), //user input when selected
        ("Exit".to_owned(), MenuItem::new_numeral(String::new()))
    ];
    
    let items: &mut Vec<(String, MenuItem)> = &mut menu_ver_2;
    // selection handler
    let mut selection: usize = 1; // compensate for status prompt
    let initial_print: bool = true;
    let longest_name: u16 = longest_name_calc(items);

    // print initial menu
    print_menu(longest_name, selection, items, true);
    loop {

        // output user selection 
        let result: usize = engage(items,&mut selection, initial_print);

        // adjust TUI based on said user selection
        print_menu(longest_name, result, items, false);

        selection = result;

        if selection != 4 {
            continue;
        }
        break;
    }
    execute!(stdout(), cursor::Show).unwrap();
    // access values for export 
    //println!("{}", menu_ver_2[1].1.val);
    // implement export logic here
}