trait Animal {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat {
    name: String,
}
impl Animal for Cat {
    fn speak(&self) {
        println!("meow!");
    }
}

#[derive(Debug)]
struct Dog {
    name: String,
}
impl Animal for Dog {
    fn speak(&self) {
        println!("woof!");
    }
}

fn main() {
    let bdd = String::from("bdd");
    let cdd = String::from("cdd");
    let cat = Box::new(Cat { name: bdd });
    print_animal_name(&cat.name);
    let dog = Box::new(Dog { name: cdd });
    print_dog(&dog);
    let animals: Vec<Box<dyn Animal>> = vec![cat, dog];
    animals_sounds(&animals);
}

fn print_animal_name(name: &str) {
    println!("{name}");
}

fn print_dog(dog: &Dog) {
    println!("{:?}", dog)
}

fn animals_sounds(animals: &[Box<dyn Animal>]) {
    for a in animals {
        a.speak();
    }
}
