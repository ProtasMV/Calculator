use std::io::{self, Write};
use colored::*;

const SYS_COLOR: Color = Color::TrueColor { r: 255, g: 255, b: 0 };
const AUTOR_COLOR: Color = Color::TrueColor {r: 255, g: 165, b: 0};

fn main() {
    println!("{}", "Welcome to calculator".color(SYS_COLOR));
    println!("{}", "Made by: @ProtasMV".color(AUTOR_COLOR));
    
    println!();
    println!("{}", "1)Calculator, 2)Math gym".color(SYS_COLOR));
    let user_mode = input();

    match user_mode.trim().to_lowercase().as_str() {
        "1"|"calculator"|"cal" => {
            loop {
                println!();
                
                let user_first_num = input_number();
                let user_two_num = input_number();
                
                println!();
                calc(user_first_num, user_two_num);

                if user_want_continue() {continue}
                else if !user_want_continue() {break}
            }
        },
        "2"|"math gym"|"gym" => {
            loop {
                println!();

                let rand_num_1 = rand::random_range(1..5000);
                let rand_num_2 = rand::random_range(1..5000);
                
                let rand_num_1 = rand_num_1 as f64;
                let rand_num_2 = rand_num_2 as f64;

                let rand_action = rand::random_range(1..3);
                let answear_list = match rand_action {
                    1 => (rand_num_1+rand_num_2, "+"),
                    2 => (rand_num_1-rand_num_2, "-"),
                    3 => (rand_num_1*rand_num_2, "*"),
                    _ => {
                        println!("Error 3"); (0.0, "?")
                    }
                };
                
                let (rand_answer, act) = answear_list;

                println!("{rand_num_1}{act}{rand_num_2}=?");
                loop {
                    let user_try = loop {
                        print!("Your guess: ");
                        flush();

                        let user_try = input();
                        let user_try = parse_to_f64(&user_try.trim());
                        let (user_try, error) = user_try;
                        
                        if error {continue}
                        else if !error {break user_try}
                    };

                    println!();
                    if user_try == rand_answer {
                        println!("Corect!"); break;
                    } else {
                        println!("Incorect!")
                    }
                }

                if user_want_continue() {continue}
                else {break}
            }
        },
        _ => {
            
        }
    }
}

fn calc(user_first_num: f64, user_two_num: f64) {
    loop {
        println!("{}", "Action: 1)+, 2)-, 3)*, 4)/".color(SYS_COLOR));
        let user_action = input();

       match user_action.trim().to_lowercase().as_str() {
            "1"|"+" => {
                println!("Result: {}", user_first_num+user_two_num)
            },
           "2"|"-" => {
                println!("Result: {}", user_first_num-user_two_num)                    
            },
            "3"|"*" => {
                println!("Result: {}", user_first_num*user_two_num)                    
            },
            "4"|"/" => {
                if user_first_num == 0.0 || user_two_num == 0.0 {
                    println!("{}", "Can't divide by 0")
                } else {
                    println!("Result: {}", user_first_num/user_two_num)                
                }                    
            },
            _=> {
                println!("{}", "Invalid input, try again");
                continue;
            }
        }
    }    
}

fn input_number() -> f64 {
loop {
        print!("{}", "Number: ".color(SYS_COLOR));
        flush();
        let user_first_num = input();
        let user_first_num = parse_to_f64(&user_first_num);
        let (user_first_num, error) = user_first_num;
                        
        if error {continue} else if !error {break user_first_num}
    }    
}

fn input() -> String {
    let mut data = String::new();
    io::stdin()
        .read_line(&mut data)
        .expect("Error 1");
    data
}

fn flush() {
    io::stdout()
        .flush()
        .expect("Error 2");
}

fn parse_to_f64(data: &str) -> (f64, bool) {
    match data.trim().parse() {
        Ok(num) => {(num, false)},
        Err(er) => {
            println!("Error: {er}, please try again");
            (0.0, true)
        }
    }
}

fn user_want_continue() -> bool {
    loop {
        println!("{}", "Continue? 1)Yes, 2)No".color(SYS_COLOR));
        let user_input = input();
        
        match user_input.trim().to_lowercase().as_str() {
            "1"|"yes"|"y" => return true,
            "2"|"no"|"n" => return false,
            _ => {
                println!("Invalid input, please try again");
                println!();
                continue
            }
        }
    }
}