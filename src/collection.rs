
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

pub fn string_test() {
    //let str1 = "fdsa".to_string();
    let mut str1 = String::from("fdsa");
    str1.push_str(" jkl;");
    str1 += " aaa";

    let mut str2 = String::from(" str2");
    str1 += &str2;
    str2 += " bbb";

    println!("str1: {}", str1);
    println!("str2: {}", str2);

    // format
    let format_str = format!("{} --{}", str1, str2);
    println!("format str: {}", format_str);


    // indexing/slicing...
    let str3 = String::from("学习");
    println!("str3 len:{}", str3.len());
    //println!("slice string: {}", &str3[0]); // String indexing not permitted
    //println!("slice string: {}", &str3[0..3]); // String slicing not always good

    // iterating
    for c in str3.chars() {
        println!("{}", c);
    }
    for c in str3.bytes() {
        println!("{}", c);
    }
}
