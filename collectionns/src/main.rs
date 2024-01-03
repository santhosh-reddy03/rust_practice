fn main() {
    println!("Hello, Vectors!");
    // collections are stored in heap instead of stack and can grow or shrink dynamically
    // to create a new vector we can use below, it is used to store integers
    let mut v: Vec<i32> = Vec::new();
    // type annotation is needed when we are creating a empty vector to descibe what values are allowed to store in vector
    // apart from this rust can infer the type , and its more common to create a vec<T> with initial values using vec! macro
    {
        // drops the v2 as it goes out of scope
        let _v2 = vec![1, 2, 3];
    }
    // to add values in vector
    v.push(1);
    v.push(2);
    v.push(3);
    {
        // different ways to access the elements of a vector
        let v2 = &v[2];
        println!("{v2}");
        // the below return the None when there is no element and the above errors out
        let v2 = &v.get(2);
        // let v2 = v2.expect("a number");
        match v2 {
            Some(num) => println!("{}", num),
            None => println!("number dont exist"),
        }

        println!("{}", v2.expect(" a num"));
    }
    v.push(4);
    for i in &mut v {
        // * is used to dereference the value and get actual value from refernce
        *i += 50;
    }
    {
        // immutable reference iterating
        for i in &v {
            println!("element {}", i);
        }
    }

    // we can store different data types in a vector using enum variants as shown below

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
    // knowing runtime dataype helps us in allocating memory in the right way and also handling errors and logic in a
    // correct way
    // but if we dont know run time data types we can still use vector with trait
}
