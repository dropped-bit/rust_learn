// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
enum TicketType {
    Backstage(f32, String),
    VIP(f32, String),
    Standard(f32),
}


fn main() {
    // * Create one of each ticket and place into a vector

    let ticket_db = vec![
        TicketType::Backstage(32.1, "Charlie".to_owned()),
        TicketType::VIP(22.5, "Martin".to_owned()),
        TicketType::Standard(10.2),
    ];

    // * Use a match expression while iterating the vector to print the ticket info

    for item in ticket_db {
        match item {
            TicketType::Backstage(price, name) => {
                println!(
                    "Ticket Type: Backstage, price: {:?}, ticket holder: {:?}",
                    price,
                    name
                )
            }
            TicketType::VIP(price, name) => {
                println!(
                    "Ticket Type: VIP, price: {:?}, ticket holder: {:?}",
                    price,
                    name
                )
            }
            TicketType::Standard(price) => println!("Ticket Type: Standard, price: {:?}", price),
        };
    }

}
