
use std::mem;

#[derive(Debug)]
enum User {
    Reader { name: String },
    Writer { name: String },
    Admin { name: String },
}

fn main() {
    let mut user = User::Reader {name:"cdd".to_owned()};
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");
}

fn promote(u: &mut User) {
    // *u = match u {
    //     Reader { name } => Writer { name: name.clone() },
    //     Writer { name } => Admin { name: name.clone() },
    //     Admin { name: _ } => return,
    // }

    *u = match u {
        User::Reader { name } => User::Writer {
            name: mem::take(name),
        },
        User::Writer { name } => User::Admin {
            name: mem::take(name),
        },
        User::Admin { name: _ } => return,
    }
}
