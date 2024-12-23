fn main() {
    let mut x: i8 = 5;
    println!("{}", x);

    for i in 0..10 {
        x = x + i;
    }
    println!("{}", x);
    let is_male = true;
    let is_legal = true;

    if is_male{
        println!{"{}", "Hee is a male!"}
    }
    if is_legal{
        println!{"{}", "He is legal!"}
    }
    if is_male && is_legal{
        println!{"{}", "He is a legal Male!"}
    }

    let mut greeting = String :: from("Hello, World!");
    println!("{}", greeting.clone() + "YoYo");
    println!("{}", greeting);
    let char1 = greeting.chars().nth(0);    
    println!("{}", char1.unwrap());
}
