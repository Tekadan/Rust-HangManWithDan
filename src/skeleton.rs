fn my_game() -> bool {
    // Setup the player, set their number of
    // guesses and select their word
    init_game();

    while true {
        // Display the users current state
        output_game_state();

        // Prompt the user to guess, collect and validate guess
        collect_guess();

        // Compared guess against phrase
        evaluate_guess();

        // Did they win?
        // Did they lose?
        // Is the game over?
        is_game_over();
    }

    // Output to the user their results
    display_end_results();
}

fn init_game() -> u32 {
    return 42;
}

fn output_game_state() -> String {
    return "Doing Great!".to_string();
}

fn collect_guess() -> char {
    return '?';
}

fn evaluate_guess() -> bool {
    return false;
}

fn is_game_over() -> bool{
    return true;
}

fn display_end_results() -> String {
    return "Better Luck Next Time!".to_string();
}

