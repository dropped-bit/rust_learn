// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}
// * Implement a `new` function for the `Adult` structure that returns a Result:

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        // self refers to implementation
        // Adult, whcih is a struct so we will get an Adult struct back
        //   * The Ok variant should contain the initialized structure, but only
        //     if the person is aged 21 or older
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            //   * The Err variant should contain a String (or &str) that explains why
            //     the structure could not be created
            Err("Age must be at least 21")
        }
    }
}

fn main() {
    // * Instantiate two `Adult` structures:
    //   * One should be aged under 21
    //   * One should be 21 or over
    let adult_one = Adult::new(8, "Domi");
    let adult_two = Adult::new(22, "Haarl");
    // * Use `match` to print out a message for each `Adult`:
    //   * For the Ok variant, print any message you want
    //   * For the Err variant, print out the error message
    match adult_one {
        Ok(adult_one) => println!("{} is {} years old", adult_one.name, adult_one.age),
        Err(e) => println!("{e}"),
    }
    match adult_two {
        Ok(adult_two) => println!("{} is {} years old", adult_two.name, adult_two.age),
        Err(e) => println!("{e}"),
    }
}
