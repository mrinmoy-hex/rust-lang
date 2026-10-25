mod garden;

use garden::vegetables::Asparagus;

fn main() {
    // Public child modules and items can be reached through their module path.
    let vegetables = [Asparagus::TypeA, Asparagus::TypeB];

    for vegetable in vegetables {
        match vegetable {
            Asparagus::TypeA => println!("A type A asparagus."),
            Asparagus::TypeB => println!("A type B asparagus."),
        }
    }
}
