// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:


fn main() {
// * Use a vector to store 4 numbers
    let my_vector = vec![10, 20, 30, 40];

// * Use the .len() function to print the number of elements in a vector
    println!("Vector Length: {:?}", my_vector.len());

// * Iterate through the vector using a for..in loop
    for num in my_vector {
// * Determine whether to print the number or print "thirty" inside the loop
        match num {
            30 => println!("Thirty"),
            _ => println!("{:?}", num)
        }
    }
}
