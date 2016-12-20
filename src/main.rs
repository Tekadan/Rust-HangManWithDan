use std::io;

fn main() {
    let game_result: bool = game_loop();
}

fn game_loop() -> bool {
    // Game startup
    println!("Welcome to Hangman, with Dan!");

    let mut remaining_guesses: u32 = set_difficulty();
    // Word the player will be guessing
    let phrase = "hello world".to_string();

    // Reformats phrase to look like "_____ _____"
    let mut display_string = create_hangman_string(phrase.clone());

    // Game Loop
    let mut is_winner: bool = false;
    while remaining_guesses > 0 || is_winner {
        // Display to the Player the current state of the game
        output_game_state(remaining_guesses.clone(), display_string.clone());

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Input parse failed!");
        // Convert string input to char
        let mut letter: char = guess.chars().nth(0).unwrap();

        if letter.is_alphabetic() {
            letter = letter.to_lowercase().collect::<String>().chars().nth(0).unwrap();;
        } else {
            println!("Hey, that wasn't a letter!!");
            println!("\n");
            continue;
        }

        let mut found_match: bool = false;
    
        // See if we have a match in the string
        for (i, c) in phrase.chars().enumerate() {
            if letter == c {
                found_match = true;
                // TODO: Replace char in 'display_string' here.
                display_string.remove(i);
                display_string.insert(i, c);
            }
        }

        // Oops, wrong answer
        if !found_match {
            println!("Wrong Guess!");
            remaining_guesses = remaining_guesses - 1;
        } else {
            // Check for win condition
            let win_check = winner_check(display_string.clone());
            if win_check {
                is_winner = true;
                break;
            }
        }
        println!("\n");
    }

    // End Game!
    return end_game(is_winner, phrase);
}

// Allows the player to select their difficulty
fn set_difficulty() -> u32 {

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
            break;
        }
    }

    if difficulty == 1 {
        println!("Easy Selected! Total Guesses: 6");
        return 6;
    }else if difficulty == 2 {
        println!("Easy Selected! Total Guesses: 4");
        return 4;
    } else {
        println!("Easy Selected! Total Guesses: 2");
        return 2;
    }
}

// Converts a regular alphabetic string to a hangman string
fn create_hangman_string(phrase: String) -> String {
    let phrase_clone = phrase;
    let mut display_string = String::new();

    for c in phrase_clone.chars() {
        if c != ' ' {
            display_string = display_string + "_";
        }else{
            display_string = display_string + " ";
        }
    }

    return display_string;
}

// Displays the current state of the hangman game
fn output_game_state(remaining_guesses: u32, display_string: String) {
    println!("Only {} guesses remaining!", &remaining_guesses);
    println!("{}", &display_string);
}

// Checks the display string to see if there are any unknown characters left
fn winner_check(display_string_clone: String) -> bool {
    let mut win_check: bool = true;
    for c in display_string_clone.chars() {
        // if there is an underscore left, there is still room to guess!
        if c == '_' {
            win_check = false;
            break;
        }
    }
    return win_check;
}

// Used to display the end results of the game - winner or loser
fn end_game(result: bool, phrase: String) -> bool {
    if result {
        println!("You win!");
        return true;
    } else{
        println!("You lose!");
        println!("The phrase was: {}", &phrase);
        return false;
    }
}