use std::io;

pub mod bcm2835;
pub mod epd;

use epd::doodle;
use epd::doodle::Color;

pub fn interactive() {
    epd::epd_init();
    let mut buff = epd::Frame::new();
    println!("Welcome to doodle interactive!");
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("welp input is broken?");
        let mut command = command.split_whitespace();
        match command.next().unwrap_or("help") {
            "exit" => break,
            "clear" => buff.clear(),
            "draw" => epd::display_color_frame(&buff),
            "new" => interactive_new(&mut command, &mut buff),
            _ =>  println!("Help:\nCommands {{exit;clear;draw;new {{point(x, y, color);rect(x, y, width, height, color)}}}}"),
        };
    }
    epd::sleep();
}

//don't look its ugly
fn interactive_new(mut command: &mut std::str::SplitWhitespace, mut buff: &mut epd::Frame) {
    let print_usage = |msg| println!("{}\nUsage: new point {{x, y, color}}|rect {{x, y, width, height, color}} ", msg);
    let parse_color = |command: &mut std::str::SplitWhitespace| match command.next() {
        Some(color) => match color {
            "white" => {Some(Color::White)},
            "red" => {Some(Color::Red)},
            "black" => {Some(Color::Black)},
            _ => None
        },
        None => None, 
        };
    let parse_coord = |command: &mut std::str::SplitWhitespace| match command.next() {
        Some(number) => number.parse::<isize>().ok(),
        None => None
        };
    if let Some(word) = command.next() {
        match word {
            "point" => {
                match parse_coord(&mut command) {
                    Some(x) => {
                        match parse_coord(&mut command) {
                            Some(y) => match parse_color(&mut command) {
                                Some(color) => doodle::draw_point(&mut buff, doodle::Point{x: x, y: y}, color),
                                None => print_usage("Can not parse/find the color."),
                            }
                            None => print_usage("Can not parse/find the y coordinate."),
                        }
                    },
                    None => print_usage("Can not parse/find the x coordinate."),
                }
            },
            "rect" => {
                match parse_coord(&mut command) {
                    Some(x) => {
                        match parse_coord(&mut command) {
                            Some(y) => match parse_coord(&mut command) {
                                Some(width) => {
                                    match parse_coord(&mut command) {
                                        Some(height) => {match parse_color(&mut command) {
                                            Some(color) => doodle::draw_rect(&mut buff, doodle::Point{x: x, y: y}, width, height, color),
                                            None => print_usage("Can not parse/find the color.")
                                            }
                                        },
                                        None => print_usage("Can not parse/find the height."),
                                    }
                                }
                                None => print_usage("Can not parse/find the width."),
                            }
                            None => print_usage("Can not parse/find the y coordinate."),
                        }
                    },
                    None => print_usage("Can not parse/find the x coordinate."),
                }
            },
            _ => print_usage("No such type."),
        }
    } else {
        print_usage("Type not found");
    }
}

pub fn wake_draw_white_sleep() {
    epd::epd_init();
    let mut buff = epd::Frame::new();
    doodle::draw_rect(&mut buff, doodle::Point{x:20, y: 70}, 84, 60, Color::Red);
    epd::display_color_frame(&buff);
    epd::sleep();
}
