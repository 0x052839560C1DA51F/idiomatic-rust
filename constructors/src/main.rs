use constructors::{Post, User};

fn main() {
    let user1 = User::new("cdd".to_owned()).unwrap_or_default();
    println!("{:?}", user1);
    let post1 = Post::default();
    let post2 = Post::new("example content".to_owned());
    println!("{:?}", post1);
    println!("{:?}", post2);
}
