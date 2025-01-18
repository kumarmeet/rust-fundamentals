use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::zip;

fn main() {
    println!("Hello, world!");

    //tuple
    let tuple = (1, "hello", 3.14);
    println!("tuple: {:?}", tuple.2);
    let (x, y, z) = tuple; // Destructure the tuple
    println!("x = {}, y = {}, z = {}", x, y, z);

    // vectors
    let mut vec1 = Vec::new();

    vec1.push(1);
    vec1.push(2);
    vec1.push(3);

    println!("{:?}", vec1);

    let mut an_vec = vec!["this", "is", "a", "test", "string"];

    println!("{:?}", an_vec.join(" "));

    let popped_data = an_vec.pop();

    let result = match popped_data {
        Some(x) => x.to_string(),
        None => "nothing".to_string(),
    };

    println!("{:?}", result);

    an_vec.extend(["added", "removed", "modified"]);

    println!("{:?}", an_vec);

    let removed_ele = an_vec.remove(0);

    println!("{:?}", removed_ele);

    println!("{:?}", an_vec[2]);

    for i in &an_vec {
        println!("{}", i);
    }

    println!("{}", an_vec.len());

    println!("{}", an_vec.capacity()); // 10

    an_vec.shrink_to_fit();

    println!("{}", an_vec.capacity());

    an_vec.sort();

    println!("{:?}", an_vec);

    println!("{:?}", an_vec.drain(2..5)); // removed from original vec

    println!("{:?}", an_vec.contains(&"has"));

    println!("before reverse -> {:?}", an_vec);

    println!("{:?}", an_vec.reverse());

    println!("after reverse -> {:?}", an_vec);

    an_vec.clear();

    println!("{:?}", an_vec.is_empty());

    let an_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Get a slice of the vector from index 2 to 5 (exclusive)
    let slice = &an_vec[2..5];

    println!("{:?}", slice); // Prints [3, 4, 5]
    println!("{:?}", an_vec); // Original vector is unchanged: [1, 2, 3, 4, 5, 6, 7, 8, 9]

    // Create a new vector from the range
    let new_vec = Vec::from(&an_vec[2..5]);

    println!("{:?}", new_vec); // Prints [3, 4, 5]
    println!("{:?}", an_vec); // Original vector is unchanged: [1, 2, 3, 4, 5, 6, 7, 8, 9]

    println!("{:?}", even_filter(an_vec));
    // println!("{:?}", even_filter(&an_vec));

    //heterogeneous vector

    let mut het_vec: Vec<I32OrString> = Vec::new();

    het_vec.push(I32OrString::I32(44));
    het_vec.push(I32OrString::String("hello".to_string()));

    for i in &het_vec {
        let mut data = match i {
            I32OrString::I32(i) => i.to_string(),
            I32OrString::String(i) => i.to_string(),
        };

        println!("data -> {:?}", data);
    }

    //hashmaps

    let mut hashmap = HashMap::new();

    hashmap.insert("john", 29);
    hashmap.insert("ketty", 18);
    hashmap.insert("warn", 21);
    hashmap.insert("meet", 33);

    println!("{:?}", hashmap);

    let age = hashmap.get("john");

    match age {
        Some(&age) => println!("john: {}", age),
        None => println!("no john"),
    }

    if let Some(age) = hashmap.get_mut("ketty") {
        *age += 1; // Increment Alice's age
    }

    println!("{:?}", hashmap);

    if hashmap.contains_key("john") {
        println!("john: {}", hashmap["john"]);
    }

    let removed = hashmap.remove("ketty");

    match removed {
        Some(x) => println!("ketty: {}", x),
        None => println!("ketty not found"),
    }

    println!("{:?}", hashmap);

    for (k, v) in &hashmap {
        println!("{}: {}", k, v);
    }

    for (_, age) in hashmap.iter_mut() {
        *age += 1; // Increment everyone's age
    }

    println!("{:?}", hashmap);

    println!("{:?}", hashmap.len());

    println!("{:?}", hashmap.keys());

    println!("{:?}", hashmap.values());

    //updating a values based on key

    match hashmap.entry("meet") {
        // if key is exist
        Entry::Occupied(mut entry) => {
            *entry.get_mut() += 10;
        }
        // if key is not exist then make a key and insert value
        Entry::Vacant(entry) => {
            entry.insert(24);
        }
    }

    // shorthand of above match arm code

    hashmap.entry("mark").or_insert(28);

    println!("{:?}", hashmap);

    println!("{:?}", hashmap);

    let mut another_map = HashMap::new();

    another_map.insert("penny", 12);
    another_map.insert("jolly", 16);

    hashmap.extend(another_map);

    println!("{:?}", hashmap);

    //collecting into a hashmap

    let names = vec!["tory", "jenny", "castle"];
    let ages = vec![12, 16, 19, 25];

    let zipped: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    hashmap.extend(zipped);

    println!("{:?}", hashmap);

    hashmap.clear();

    println!("{:?}", hashmap.is_empty());

    //another way to define a hashmap

    let map = HashMap::from([("Alice", 30), ("Bob", 25), ("Charlie", 35)]);

    println!("{:?}", map);

    //closure

    let add = |a: i32, b: i32| -> i32 { a + b };

    println!("add of closure fun -> {}", add(10, 40));

    //lambda
    let square = |a| a * a;
    println!("square of lambda -> {:?}", square(4));

    // unnamed function
    let multiply = |a, b| a * b;
    println!("multiply of unnamed function -> {:?}", multiply(5, 4));

    let res = apply_closure(square, 10);

    println!("closure function as param -> {:?}", res);

    let mut y = 14;

    // Closure captures `y` by mutable reference
    let add_and_increment_y = |x| {
        y += 1;
        x + y
    };

    let result = apply_closure_mut(add_and_increment_y, 5);

    println!("Result: {}", result); // Prints "Result: 16"
    println!("y: {}", y); // Prints "y: 11"
}

//Closure Capturing by Mutable Reference (FnMut)
fn apply_closure_mut<F>(mut f: F, x: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(x)
}

//passing closure func as args
fn apply_closure<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

#[derive(Debug)]
enum I32OrString {
    I32(i32),
    String(String),
}

impl I32OrString {
    fn to_string_heter(&self) -> String {
        match self {
            I32OrString::I32(i) => i.to_string(),
            I32OrString::String(s) => s.clone(),
        }
    }
}

fn even_filter(vc: Vec<i32>) -> Vec<i32> {
    return vc.into_iter().filter(|&x| x % 2 == 0).collect();
}

// fn even_filter(vc:&Vec<i32>) -> Vec<i32> {
//     return vc.into_iter().filter(|&&x| x % 2 == 0).cloned().collect();
// }

fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b) // Return a tuple
}
