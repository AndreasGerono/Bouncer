#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug, PartialEq)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: u8,
}

impl Visitor {
    fn new(name: &str, age: u8, action: VisitorAction) -> Self {
        Self {
            name: (name.to_lowercase()),
            /*  If the function variable is the same as
             *  struct field you can omit the ()
             */
            age: (age),
            action: (action),
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => {
                println!("Welcome to the tree house {}", self.name);
            }
            VisitorAction::AcceptWithNote { note } => println!("{}", note),
            VisitorAction::Probation => {
                println!("{} is now a probationary member", self.name);
            }
            VisitorAction::Refuse => println!("Do not allow in!"),
        }
        if (self.action != VisitorAction::Refuse) && (self.age < 18) {
            println!("{} is not allowed to drink alcohol!", self.name);
        }
    }
}

fn what_is_your_name() -> String {
    println!("Hello, what's your name? (Leave empty and press enter to quit)");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}

fn read_age() -> u8 {
    let mut your_age: String = String::new();
    stdin()
        .read_line(&mut your_age)
        .expect("Failed to read line");

    if let Ok(num) = your_age.trim().parse() {
        return num;
    }

    println!("Please enter a valid number: ");
    read_age()
}

fn main() {
    let mut visitors_list = vec![
        Visitor::new("bert", 45, VisitorAction::Accept),
        Visitor::new(
            "steve",
            15,
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose free milk in the fridge"),
            },
        ),
        Visitor::new("fred", 30, VisitorAction::Refuse),
    ];

    loop {
        let name = what_is_your_name();

        let known_visitor =
            visitors_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(v) => v.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                }

                println!("{} is not on the visitor list.", name);
                println!("as a new guest you need to enter your age");
                let age = read_age();
                visitors_list.push(Visitor::new(
                    &name,
                    age,
                    VisitorAction::Probation,
                ));
            }
        }
    }

    println!("The ginal list of visitors:");
    println!("{:#?}", visitors_list);
}
