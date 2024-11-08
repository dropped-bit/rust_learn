// This is a program that accepts some input string
// It checks that the input string is valid agains a specific set of options
// Error handling is done as the input might not return a match

// create an enum with three choices (main menu, start, quit)
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Exit,
}

// create a function that gets an input and returns a result.
// The result should match inputs to the enum
fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "Start" => Ok(MenuChoice::Start),
        "Exit" => Ok(MenuChoice::Exit),
        _ => Err("You did not choose a valid menu choice".to_string()),
    }
}

// create a function that prints the users choice
fn print_choice(choice: &MenuChoice) {
    println!("You have chosen: {:?}", &choice)
}

// create a function that picks the choice?
fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    // in main program create a variable using the pick_choice function and print data. If it choice is
    // not valid it should handle the error
    let choice = pick_choice("mainmenu");
}
