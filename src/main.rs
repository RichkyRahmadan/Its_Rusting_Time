fn main() {
    println!("Hello, world!");
}

#[test]
fn fungsi_test_1(){
    // About Unit Test
    println!("Hello, Aku Suaminya Agnes Tachyon");
}
#[test]
fn fungsi_test_2(){
    // About Immutable Variable
    let nama = "Richky Rahmadan";
    println!("Hello, Aku {} Suaminya Gentildonna",nama);
}

#[test]
fn fungsi_test_3(){
    // About mutable variable
    let mut nama = "Richky Rahmadan";
    println!("Hello, Aku {} Suaminya Gentildonna",nama);
    nama = "The King of The Kingless World";
    println!("Hello, Aku Memiliki Title {}",nama);
}
