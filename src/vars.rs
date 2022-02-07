pub fn run() {
    let name = "Archi";
    let mut age = 29;
    println!("My name is {} and I am {}", name, age);
    age = 30;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
