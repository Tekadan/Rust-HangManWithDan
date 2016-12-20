use std::io;

fn main() {
    let game_result: bool = game_loop();
}

fn game_loop() -> bool {
    // Game startup
    println!("Welcome to Hangman, with Dan!");

    let mut remaining_guesses: i32 = set_difficulty();
    // Word the player will be guessing
    let phrase = "hello world".to_string();
    let mut display_string = String::new();

    // Create the '___ __ _____' format
    for c in phrase.chars() {
        if c != ' ' {
            display_string = display_string + "_";
        }else{
            display_string = display_string + " ";
        }
    }

    // Game Loop
    let mut is_winner: bool = false;
    while remaining_guesses > 0 || is_winner {
        println!("Only {} guesses remaining!", &remaining_guesses);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Input parse failed!");
        // Convert string input to char
        let letter: char = guess.chars().nth(0).unwrap();

        let mut found_match: bool = false;
    
        // See if we have a match in the string
        for c in phrase.chars() {
            if letter == c {
                found_match = true;
                // TODO: Replace char in 'displayString' here.
            }
        }

        // Oops, wrong answer
        if !found_match {
            println!("No match found!");
            remaining_guesses = remaining_guesses - 1;
        } else {
            // Check for win condition
            let mut win_check: bool = true;
            for c in display_string.chars() {
                // if there is an underscore left, there is still room to guess!
                if c == '_' {
                    win_check = false;
                    break;
                }
            }
            if win_check {
                is_winner = true;
                break;
            }
        }
    }

    // End Game!
    return end_game(is_winner, phrase);
}

fn set_difficulty() -> i32 {

    let mut difficulty: u32;

    loop {
        println!("Please select your difficulty: 1 for Easy, 2 for Medium, 3 for Hard");
        
        let mut difficulty_input = String::new();
        io::stdin().read_line(&mut difficulty_input).expect("Input parse failed!");

        difficulty = match difficulty_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if difficulty > 3 {
            println!("Too big!");
        } else if difficulty < 1 {
            println!("Too small!");
        }else{
            continue;
        }
    }

    if difficulty == 1 {
        return 6;
    }else if difficulty == 2 {
        return 4;
    } else {
        return 2;
    }
}

fn end_game(result: bool, phrase: String) -> bool{
    
    if result {
        println!("You win!");
        return true;
    } else{
        println!("You lose!");
        println!("The phrase was: {}.", &phrase);
        return false;
    }
}