// use std::any::type_name;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    // Comments
    // Variable
    // i: signed integer : + or -
    // u: unsigned integer: no sign only positive
    let x: i32 = 5;
    let _y: i32 = 10;
    println!("{x}"); // f string

    // Numbers: integer,float,bool: stored in stack: lifo
    let f: f64 = 30.1;
    let is_active: bool = true;
    println!("{f}{is_active}");

    // How to check for the datatype
    // println!("{}", type_name::<_>());

    // Operations
    let add = x + 4;
    println!("{add}");

    // Char: in single quotes
    let c: char = 'A';
    println!("{}",c);

    // Arrays: fixed size collection
    let arr: [i32; 3] = [3,4,5];
    println!("{:?}",arr);

    // Tuple: finite length sequences
    let tup = (1,'A',"Cool",78,true);
    println!("{:#?}",tup);

    // String `&str` for string slices, `String` for dynamic strings
    // Create a string
    let str_slice = "Hello String";
    let dyn_str = String::from("Hello String");
    println!("{str_slice} =! {dyn_str}");

    // convert string slice to a string
    println!("{}",str_slice.to_string());

    // append to a string
    let mut s = String::from("Rust");// immutable but can be made mutable via mut
    s.push_str(" RoadMap");
    s.push('.');
    println!("{s}");

    // Access an element in the string
    // for i in s.split(" "){
    //     print!("{}",i);
    // }

    for c in s.chars() {
        println!("{}",c);
    }



    // via a range
    println!("{}",s.get(0..2).unwrap());

    // for i in 0..s.len(){
    //     print!("{}",s.chars().nth(1).unwrap());
    // };

    // // Search for a term in string
    // println!("{}",s.contains("Ru"));

    // // Replace a term in a string
    // let r1 = s.replace("Ru","ro");
    // println!("{r1}");

    // println!("find::{}",s.find("Rust").unwrap_or(0));
    //  provide a default value when the Option is None, you can use unwrap_or or unwrap_or_el

    // Transform Text
    let s2 = String::from("Transforming Text");
    println!("{}",s2.to_uppercase());
    println!("{}",s2.to_lowercase());
    
    // Concat Strings
    let s3 = String::from("New Text ");
    println!("{}{}",s2,s3);
    let concat_string = format!("{s2}{s3}");
    println!("{concat_string}");
    println!("trim:: {}",s3.trim());

    // len
    println!("{}",s.len());

    // Vector :  a growable, or dynamically-sized, array-like data structure that can store homogeneous types
    let mut oddvec: Vec<i32> = Vec::new();

    // Add element to a vector
    oddvec.push(1);
    oddvec.extend([3,5,7]);
    println!("{:?}",oddvec);

    // create a vector via the vector macro `vec!`
    let mut evenvec: Vec<i32> = vec![2,4,6,8,10];
    println!("{:?}",evenvec);

    // access a vector elements
    println!("{:?}",evenvec);
    println!("{}",evenvec.first().unwrap()); // may return None/Option hence the unwrap

    // removing vector element
    evenvec.remove(2); // specify index to remove
    evenvec.pop();
    println!("{:?}",evenvec);

    // iterating over elements in a vector
    for i in oddvec.iter(){
        println!("{}",i);
    }


    // Sort
    let bin_search = evenvec.binary_search(&2).unwrap();
    println!("{bin_search}");
    evenvec.sort();
    evenvec.reverse();
    println!("{:?}",evenvec);
    

    // Collections: HashMap => key value pair similar to dictionary
    let mut fruits: HashMap<String,i32> = HashMap::new();
    // Insert data
    fruits.insert("Mango".to_string(),1);
    fruits.insert(String::from("Apple"),3);

    println!("{:?}",fruits);

    // Iterate over key-value pairs
    for (key, value) in &fruits {
        println!("{}: {}", key, value);
    }

    // Iterate over keys
    for key in fruits.keys(){
        println!("{}",key);
    }

     // Iterate over values
     for value in fruits.values() {
        println!("{}", value);
    }

    // entry api handle cases where the key may or maynot exist
    let mut hmap = HashMap::new();
    hmap.entry('a').or_insert(0);
    *hmap.entry('a').or_insert(0) += 4;

    for value in hmap.values() {
        println!("{}", value);
    }

    // Hashset
    let mut odd_set: HashSet<i32> = HashSet::new();
    // add to a set
    odd_set.insert(1);
    odd_set.insert(3);

    println!("{:?}",odd_set);

    for i in odd_set{
        println!("{i}");
    }


    // Control Flow
    // if else
    let x: i32 = 20;
    let y: i32 = 20;

    if x > y {
        println!("x:{x} is greater than y:{y}");
    }
    else if x == y  {
        println!("x:{x} is equal y:{y}");
    } else {
        println!("x:{x} is less than y:{y}");
    }

    // Loop: infinite loop
    // loop {
    //     let mut i:i32 = 2;
    //     println!("i is {}", i);
    //     if i > 100 {
    //         break;
    //     }
    //     i *= 2;
    // }

    // For loop
    // let salary = [6000,6500,5800,12000];
    // for i in salary {
    //     println!("{i}");
    // }

    // // Loop through a range
    // for i in (0..20) {
    //     println!("{i}");
    // }
    
    // enumerate
    // for (v, c) in (0..10).enumerate() {
    //     println!("The {} number loop", c);
    //     if v == 9 {
    //         println!("Here we go continue?");
    //         continue;
    //     }
    //     println!("The value of v is: {}", v);
    // }

    // Match
    let number = 7;
    match number {
        1 => println!("One"),
        7 => println!("Seven"),
        _ => println!("other"),
    }

    // multiple pattern match via the | operator
    let number = 3;
    match number {
        1 | 2 => println!("One or Two"),
        3 | 4 => println!("Three or Four"),
        _ => println!("Other"),
    }

    // matching with binding
    let number = Some(7);
    match number {
        Some(n) => println!("Found a number: {}", n),
        _ => println!("No number found"),
    }

    // if let
    // if let allows you to match a value against a pattern and execute a block of code if the pattern matches. 
    // It is essentially a shorthand for a simple match statement with only one arm.

    let option = Some(5);
    if let Some(x) = option {
        println!("The value is {}", x);
    }

    // Function
    let pf = print_first("this is cool");
    println!("{pf}");

    // Structs
    #[derive(Debug)]
    struct Atom {
        proton: i32,
        electron: i32
    }

    let a = Atom{proton: 6,electron:6};
    println!("{:?}",a);
    println!("{}",a.proton);

    // #[derive(Debug)]
    // struct Human {
    //     name: String,
    //     age: i32,
    //     gender: String,
    // }


    // Add method to structs
    // // Constructor Method
    // impl Human {
    //     fn new(name: String,age: i32, gender: String) -> Human {
    //         Human { name, age, gender }
    //     }
    //     // Method
    //     fn get_name(&self) -> &String {
    //         return &self.name
    //     }

    //     // Setter Method
    //     fn set_age(&mut self, age: i32) {
    //         self.age = age;
    //     }
    // }

    // let mut h1 = Human::new(String::from("Jesse"), 3,String::from("Male"));
    // h1.set_age(31);
    // println!("{:?}",h1);
    // println!("{}",h1.get_name());


    // class Human:
    //     def __init__(self,name,age,gender):
    //         self.name = name
    //         self.age = age
    //         self.gender = gender
    //     def __repr__(self):
    //         return "Human(self,,)"

    //     def get_name(self):
    //         return self.name 

    // Enum: Enumeration Data type
    // #[derive(Debug)]
    // enum Direction {
    //     Up,
    //     Down,
    //     Left,
    //     Right,
    // }

    // let d = Direction::Up;
    // println!("{:?}",d);

    // Trait
    pub trait PersonDetails {
        fn get_name(&self) -> &String;
        fn get_age(&self) -> i32;
        fn get_gender(&self) -> &String;
        fn describe(&self) -> String;
    }

    #[derive(Debug)]
    struct Human {
        name: String,
        age: i32,
        gender: String,
    }

    impl PersonDetails for Human {
        fn get_name(&self) -> &String {
            &self.name
        }
        fn get_age(&self) -> i32 {
            self.age
        }
    
        fn get_gender(&self) -> &String {
            &self.gender
        }
    
        fn describe(&self) -> String {
            format!("Name: {}, Age: {}, Gender: {}", self.name, self.age, self.gender)
        }
    }

    let person = Human {
        name: String::from("John Doe"),
        age: 30,
        gender: String::from("Male"),
    };

    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());
    println!("Gender: {}", person.get_gender());
    println!("{}", person.describe());

    // Trait Inheritance
    #[derive(Debug)]
    struct Angel {
        name: String,
        age: i32,
        gender: String,
    }

    impl PersonDetails for Angel {
        fn get_name(&self) -> &String {
            &self.name
        }
        fn get_age(&self) -> i32 {
            self.age
        }
    
        fn get_gender(&self) -> &String {
            &self.gender
        }
    
        fn describe(&self) -> String {
            format!("Name: {}, Age: {}, Gender: {}", self.name, self.age, self.gender)
        }
    }


   
}


fn print_first(term: &str) -> &str{
    return &term[0..1];
}

// Struct: a struct is a custom data type used for grouping related values together into one entity.
// Creating a struct


// Can have methods 