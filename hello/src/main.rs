fn main() {
    let mut bunnies = 2;
    let (bunnie, carrots) = (8, 50);
    println!("We have {} bunnies and {} carrots", bunnies, carrots);
    let bunnies = 5;
    println!("Hello, world!");

    let enigma: i32;
    if true {
        enigma = 5;
    } else {
        enigma = 7;
    }

    println!("enigma is {}", enigma);
}
