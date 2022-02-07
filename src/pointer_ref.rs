pub fn run() {
    //Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));
    //Vector
    let vec1 = vec![1, 2, 3];
    let mut vec2 = vec1.clone();
    vec2.push(3);
    println!("Values: {:?}", (&vec1, vec2));
}
