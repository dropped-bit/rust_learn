// From demo video
#[derive(Debug)] // added so we can easily print this data
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}
fn get_choice(input: &str) -> Result<MenuChoice, String> {
    // we expect a string input, but will
    // return a result so we can do proper error handling
    match input {
        // we return for each item an Ok status on expected items but have "inner data"
        // mapped to the actual enum
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()), // so anything else returns Err
    }
}

// we can prepare a function to print the MenuChoice.
// But remember: the get_choice returns a result (Ok, Err) so that's why we have to do an
// additional match int eh code below (see match choice with "inner_choice")
fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

//NOTE: Iteration 2
//We do not need all the addition match choice stuff below.
//We can just have a specific function for returning our inner data

fn pick_choice(input: &str) -> Result<(), String> {
    // we have a () because this is a unit type,
    // and as it is a result we need to specify it in this case. It returns nothing
    let choice: MenuChoice = get_choice(input)?; // the ? checks if the MenuChoice is Ok or an Err
    print_choice(&choice);
    Ok(()) // so here we are returning Ok and in this  case nothing hence Ok(())
}

fn main() {
    // so here we create a variable which has a type Result,
    // with either the enum or any other data type (it knows it is a string from get_choice method)
    // then we call the function get_choice
    //let choice: Result<MenuChoice, _> = get_choice("test");
    // we have to do this matching here with get_choice we are returning either Ok or Err
    // BUT once we have the match, we can then access the inner data, noted by "inner_choice"
    //match choice {
    //    Ok(inner_choice) => print_choice(&inner_choice),
    //    Err(e) => println!("error = {:?}", e),
    //}

    //NOTE: With Interation 2 we save all the code we originally wrote above in main()
    let choice = pick_choice("end");
    //let choice = pick_choice("start");
    println!("choice value: {:?}", choice);
}
