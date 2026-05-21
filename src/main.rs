use std::io;
use rand::Rng;
use colored::*;

fn main() {
    let mut user_mode = String::new();
    let mut str_number_one = String::new();    
    let mut str_number_two = String::new();

    println!("{}", "Welcome to MyFirstCalculator!".blue());
    println!("Mode: 1)Calculator, 2)match gym");
    
    io::stdin()
        .read_line(&mut user_mode)
        .expect("Calculator crash, eror code: 1");

    if user_mode.trim() == "1" {
    
    println!("Your first number?");

    io::stdin()
        .read_line(&mut str_number_one)
        .expect("Calculator crash, eror code: 2");
        
    let number_one: f64 = str_number_one.trim().parse()
    .expect("Calculator crash, eror code: 3");

    println!("Your two number?");
    
    io::stdin() 
        .read_line(&mut str_number_two)
        .expect("Calculator crash, eror code: 4");
   
    let number_two: f64 = str_number_two.trim().parse()
    .expect("Calculator crash, eror code: 5");

    println!("Your action 1)+, 2)-, 3)*, 4)/ ");
    let mut action = String::new();

    io::stdin()
        .read_line(&mut action)
        .expect("Calculator crash, eror code: 6");

    match action.trim() {
        "1" => {let result = number_one + number_two;
            println!("Result: {}", result)},
        "2" => {let result = number_one - number_two;
            println!("Result: {}", result)},
        "3" => {let result = number_one * number_two;
            println!("Result: {}", result)},
        "4" => {let result = number_one / number_two;
        println!("Result: {}", result)},
        _ => println!("Calculator crash, eror code: 7"),
    }

    fn_press_to_end();  

} 
    
    else if user_mode.trim() == "2" {
        let random_number_one = rand::thread_rng().gen_range(0..=100);
        let random_number_two = rand::thread_rng().gen_range(0..=100);
        let mut str_user_anser_gym = String::new();
        let rand_action = rand::thread_rng().gen_range(1..=3);
        //let mut streak = 0;

        let gym_answear = match rand_action {
            1 => {
                println!("{random_number_one} + {random_number_two}=?");
                io::stdin()
                .read_line(&mut str_user_anser_gym)
                .expect("Calculator crash, eror code: 9"); 
                random_number_one + random_number_two
                },           
           
            2 => {
                println!("{random_number_one} - {random_number_two}=?");
                io::stdin()
                .read_line(&mut str_user_anser_gym)
                .expect("Calculator crash, eror code: 9");                
                random_number_one - random_number_two
                }, 

            3 => {
                println!("{random_number_one} * {random_number_two}=?");
                
                io::stdin()
                .read_line(&mut str_user_anser_gym)
                .expect("Calculator crash, eror code: 9");              
                random_number_one * random_number_two                
                },
            _ => {println!("Calculator crash, eror code: 13"); 0}
            };
            
            let user_anser_gym: i32 = str_user_anser_gym.trim().parse().expect("Calculator crash, eror code: 8");
            if user_anser_gym == gym_answear {
                //streak = streak + 1;
                println!("{}","Correct!".green());
                //if streak>=2 {
                //    if streak <= 4 {
                //       println!("{}", "Streak: {streak}".yellow());
                //    }
                //    else if streak >= 5 {
                //        println!("{}", "Streak: {streak}".red());
                //    }
                //    else {
                //        println!("Calculator crash, eror code: 11");
                //    } 
                //}
               
                fn_press_to_end();       
            } 
            else {
                println!("{}", "Wrong!".red());
                //streak = 0;
                //    println!("{}", "Streak: {streak}".red());
                
                fn_press_to_end();
            
            }
            
    }
    else {
        println!("Calculator crash, eror code: 10");

        fn_press_to_end();

    }    
}

fn fn_press_to_end() {   
     let mut press_to_end = String::new();

    println!("Press enter to close program");
    io::stdin()
        .read_line(&mut press_to_end)
        .expect("Calculator crash, eror code: 12");     
}

//1.0 Base work calculator
//1.1 rework if-else system to match and add gym mode
//1.2 update gym mode (+, -, *) with use match and add colors for better UX
//1.3 code optimization and add streak mehanick (dont work at this time but for future) to gym mode also fix bug when consol close)