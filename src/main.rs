use std::io;

fn main() -> bool {
    return game_loop();
}

fn game_loop() -> bool {
    // Game startup
    println!("Welcome to Hangman, with Dan!");

    let mut remainingGuesses: i32 = set_difficulty();
    // Word the player will be guessing
    let phrase = "hello world".to_string();
    let mut displayString = String::new();

    // Create the '___ __ _____' format
    for c in phrase.chars() {
        if c != ' ' {
            displayString = displayString + "_";
        }else{
            displayString = displayString + " ";
        }
    }

    // Game Loop
    let mut isWinner: bool = false;
    while remainingGuesses > 0 || isWinner {
        println!("Only {} guesses remaining!", &remainingGuesses);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Input parse failed!");
        // Convert string input to char
        let letter: char = guess.chars().nth(0);

        let foundMatch: bool = false;
    
        // See if we have a match in the string
        for c in phrase.chars() {
            if letter == c {
                foundMatch = true;
            }
        }

        // Oops, wrong answer
        if !foundMatch {
            println!("No match found!");
            remainingGuesses = remainingGuesses - 1;
        } else {
            // Check for win condition
            let winCheck: bool = true;
            for c in displayString.chars() {
                // if there is an underscore left, there is still room to guess!
                if c == '_' {
                    winCheck = false;
                    break;
                }
            }
            if winCheck {
                isWinner = true;
                break;
            }
        }
    }

    // End Game!
    return end_game(isWinner, phrase);
}

fn set_difficulty() -> i32 {

    let difficulty: u32;

    loop {
        println!("Please select your difficulty: 1 for Easy, 2 for Medium, 3 for Hard");
        
        let mut difficultyInput = String::new();
        io::stdin().read_line(&mut difficultyInput).expect("Input parse failed!");

        difficulty = match difficultyInput.trim().parse() {
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