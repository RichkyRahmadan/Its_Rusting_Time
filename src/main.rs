fn main() {
    println!("Hello, world!");
}

#[test]
fn unit_test(){
    // About Unit Test
    println!("Hello, Aku Suaminya Agnes Tachyon");
}
#[test]
fn Immutable_variable(){
    // About Immutable Variable
    let nama = "Richky Rahmadan";
    println!("Hello, Aku {} Suaminya Gentildonna",nama);
}

#[test]
fn mutable_variable(){
    // About mutable variable
    let mut nama = "Richky Rahmadan";
    println!("Hello, Aku {} Suaminya Gentildonna",nama);
    nama = "The King of The Kingless World";
    println!("Hello, Aku Memiliki Title {}",nama);
}
