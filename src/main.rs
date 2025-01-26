pub mod lab;

#[cfg(test)]
#[path = "./lab_test.rs"]
pub mod lab_test;

fn main() {
    println!("This program doesn't actually do anything.\n");
    println!("Run 'make check' to see if the list is implemented properly.");
}
