
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
    let mut v = vec![100, 30, 20]; // v is type of Vec<i32>
    v.push(123);
    v.sort();
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
    str1.push_str(&str2);
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


use std::collections::HashMap;
pub fn hashmap_test() {
    let ks1 = String::from("k1");
    let ks2 = String::from("k2");
    let ks3 = String::from("k3");
    let mut htbl = HashMap::new();
    
    

    //all of the keys must have the same type, and all of the values must have the same type.
    htbl.insert(ks1, 11);
    htbl.insert(ks2, 12);
    // println!("ks1: {}", ks1); // Error. ks1/ks2 have been moved to hashmap, cannot be used

    let val = htbl.get("k2");
    if let Some(_) = val {
        println!("value: {}", val.unwrap());
    }

    htbl.entry(ks3).or_insert(13);
    println!("htbl: {:?}", htbl);



    let tmps1 = String::from("tmpk1");
    let mut tmph = HashMap::new();
    tmph.insert(&tmps1, 0);
    // {
    //     let tmps2 = String::from("tmpk2");
    //     tmph.insert(&tmps2, 1); // Error: borrowed value does not live long enough
    // }
    println!("tmph: {:?}", tmph);




    // let kv = vec!["t1", "t2"];
    // let vv = vec![33,44];

    // let htbl1 :HashMap<_, _> = kv.iter().zip(vv.iter()).collect();
    for (key, val) in &htbl {
        println!("key:{}, val:{}", key, val);
    }
}