
pub fn log_collection() {
    println!("*******************Chapter 8***********************");
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vector_test() {
    // Vectors can only store values of the same type
    //let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3]; // v is type of Vec<i32>
    v.push(123);
    println!("vec: {:?}", v[2]);


    for i in &mut v {
        println!("i: {}", i);
        *i = *i + 1;
    }

    match v.get(3) {
        None => println!("no value in vec"),
        Some(b) => println!("some value: {}", b)
    }
    println!("value unwrap:{}", v.get(3).unwrap());

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row: {:?}", row);

}