// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct StudentLocker {
    name: String,
    locker_number: Option<i32>,
}

fn main() {
    let assignment_1 = StudentLocker {
        name: "Charlie".to_owned(),
        locker_number: None,
    };
    let assignment_2 = StudentLocker {
        name: "Gerl".to_owned(),
        locker_number: Some(666),
    };
    let assignment_3 = StudentLocker {
        name: "Tig".to_owned(),
        locker_number: Some(11),
    };

    let assignments = vec![assignment_1, assignment_2, assignment_3];

    for assignment in assignments {
        match assignment.locker_number {
            Some(locker) => {
                println!(
                    "Name: {:?}, Locker Number: {:?}",
                    assignment.name, locker,
                )
            }
            None => {
                println!(
                    "Name: {:?}, Locker Number not assigned",
                    assignment.name,
                )
            }
        }
    }
}
