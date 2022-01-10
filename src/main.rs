use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}.", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}.", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve whiskey to {}.", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now A probationary member.", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn get_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = get_name();
    let mut visitor_list = vec![
        Visitor::new("Ben", VisitorAction::Accept, 32),
        Visitor::new(
            "Rick",
            VisitorAction::AcceptWithNote {
                note: String::from("This is Rick"),
            },
            20,
        ),
        Visitor::new("Luke", VisitorAction::Refuse, 55),
    ];

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => {
            if !name.is_empty() {
                println!("{} is not on the visitor list.", name);
                visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
