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

    for i in &an_vec{
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

    let mut het_vec:Vec<I32OrString> = Vec::new();

    het_vec.push(I32OrString::I32(44));
    het_vec.push(I32OrString::String("hello".to_string()));

    for i in &het_vec{
        let mut data = match i {
            I32OrString::I32(i) => i.to_string(),
            I32OrString::String(i) => i.to_string(),
        };

        println!("data -> {:?}", data);
    }
}

#[derive(Debug)]
enum I32OrString{
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

fn even_filter(vc:Vec<i32>) -> Vec<i32> {
    return vc.into_iter().filter(|&x| x % 2 == 0).collect();
}

// fn even_filter(vc:&Vec<i32>) -> Vec<i32> {
//     return vc.into_iter().filter(|&&x| x % 2 == 0).cloned().collect();
// }

fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b) // Return a tuple
}
