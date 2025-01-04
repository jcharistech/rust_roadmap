fn main() {
    
    // Higher Order Fxn in Rust

    // Iterator: Collections, Vectors, Sequence
    let numbers = vec![2,4,3,5,6,20,30];
    println!("{:?}",numbers);

    // Closures: anonymous fxn
    let multiplier = |x| x * 2;
    let res = multiplier(4);
    println!("{}",res);

    // Map: apply a fxn to a each element in a vector/collections and return another collection
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("{:?}",doubled);

    // Filter:involves selecting elements from a collection based on a predicate: if  it is true or false
    let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x %2 == 0).cloned().collect();
    println!("{:?}",even_numbers);

    // Reduce or Folding: reducing a collection to a single value by applying a binary operation
    let sum_numbers = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("{}",sum_numbers);

    // FlatMap: Flat mapping involves applying a function that returns an iterator to each element of a collection and then flattening the results.
    let flattened: Vec<_> = numbers.iter().flat_map(|&x|vec![x,x * 2]).collect();
    println!("{:?}",flattened);
    // dbg!("{:?}",flattened);

    // Find: searching for the first element in a collection that satisfies a predicate.
    let first_even_number = numbers.iter().find(|&x| x % 2 == 0);
    println!("{:?}",first_even_number);

    // Partition: Partitioning involves splitting a collection into two parts based on a predicate.
    let (even, odd): (Vec<i32>, Vec<i32>) = numbers.iter().partition(|&x| x % 2 == 0);
    println!("Even: {:?}, Odd: {:?}", even, odd);

    // Pattern Matching
    match_example(Some(5));





}

// fn multiply(x: i32){
//     x * 2;
// }


fn match_example(value: Option<i32>) {
    match value {
        Some(x) => println!("Received a value: {}", x),
        None => println!("Received None"),
    }
}
 