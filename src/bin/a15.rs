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
#[derive(Debug, Clone, Copy)]
enum TicketType {
    Backstage,
    VIP,
    Standard,
}

struct Ticket {
    event_name: String,
    ticket_holder: String,
    price: f32,
    ticket_type: TicketType,
}

fn main() {
    // * Create one of each ticket and place into a vector
    let ticket_1 = Ticket {
        event_name: "Football".to_owned(),
        ticket_holder: "Marcel Finke".to_owned(),
        price: 87.22,
        ticket_type: TicketType::Backstage,
    };

    let ticket_2 = Ticket {
        event_name: "Slipknot".to_owned(),
        ticket_holder: "Christian Hense".to_owned(),
        price: 50.10,
        ticket_type: TicketType::VIP,
    };

    let ticket_3 = Ticket {
        event_name: "Abba".to_owned(),
        ticket_holder: "Knut Sonnabend".to_owned(),
        price: 2.0,
        ticket_type: TicketType::Standard,
    };

    let ticket_db = vec![ticket_1, ticket_2, ticket_3];

    // * Use a match expression while iterating the vector to print the ticket info

    for item in ticket_db {
        match item {
            Ticket { ticket_type: TicketType::Backstage, .. } => {
                println!(
                    "For the event: {:?}, Ticket Type: Backstage, Ticket Holder {:?}",
                    item.event_name,
                    item.ticket_holder
                )
            }
            Ticket { ticket_type: TicketType::VIP, .. } => {
                println!(
                    "For the event: {:?}, Ticket Type: VIP, Ticket Holder {:?}",
                    item.event_name,
                    item.ticket_holder
                )
            }
            Ticket { ticket_type: TicketType::Standard, .. } => {
                println!(
                    "For the event: {:?}. Ticket Type: Standard",
                    item.event_name
                )
            }
        }
    }

}
