//use std::io;
use rand::Rng;
//use std::cmp::Ordering;
use std::mem;

pub fn log_hello() {
    println!("*******************Chapter 1~6***********************");
}

pub fn hello() {
    /**************basic variable define*************************/
    let test = "    ";
    println!("...{}...", test);
    let mut test: usize = test.len();
    test = test + 1;
    println!("len:{}", test);

    /***************array and tuple************************/

    let mut index = 0;

    // array in Rust has a FIXED length, and all elements have the same type (like C/C++)
    let mut months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    months[0] = "Jan"; // this is array indexing method only, cannot be used on tuple
    let array_a: [u32; 5] = [0; 5];
    index = index + 1;
    println!("month: {} {}", months[0], array_a[index]);

    // tuple in Rust has a FIXED length. can have different types, value can be changed(cannot change type)
    let mut tup_v = (500, "test", 'z', array_a);
    tup_v.0 = 9; // this is tuple indexing method only, cannot be used on array
    println!("cat: {} {}", tup_v.0, tup_v.3[2]);

    // array and tuple can be embedded to each other
    let aar = [tup_v];
    println!("aar: {}", aar[0].3[2]);

    /**************function calling*************************/

    let paramt = 198;
    println!("ret val: {}", another_function(paramt)[0]);
    println!("after call, paramt: {}", paramt);

    for it in months.iter() {
        println!("--- {}", it);
    }
    for id in 0..5 {
        println!("index: {}", id);
    }

    ownership_test();
    slice_test();

    println!("#### after struct_test {:?}", struct_test());
    enum_test();
    option_test();
let __val: u32 = rand::thread_rng().gen_range(1, 111);
    return;

    /*You declare constants using the const keyword instead of the let keyword,
      and the type of the value must be annotated. */
/*
    const MAX: u32 = 101;
    let val: u32 = rand::thread_rng().gen_range(1, MAX);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&val) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("win!");
                break;
            },
        }
    }
*/
}

/******************** Chapter 3 *******************/
fn another_function(mut param1: u32) -> [u32; 1] {
    println!("*******************Chapter 3***********************");
    let condition = true;
    let number = if condition {
        10
    } else {
        6
    };

    println!("param: {}", param1);
    param1 = 999;
    println!("param: {}", param1);

    if number == 5 {
        println!("number is 5");
    } else if number > 5 && number <= 10  {
        println!("number is 5 < x <= 10");
    } else if number > 10 || number ==100 {
        println!("number is x > 10 or 100");
    }

    let mut counter = 0;

    let _ret = loop {
        counter += 1 * (1 + 1);
        println!("++ {}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };

    while counter !=0 {
        println!("-- {}", counter);
        counter -= 2;
    };


    [1]
}

/******************** Chapter 4 *******************/
fn ownership_test() {
    println!("*******************Chapter 4***********************");
    /***************move*****************/
    let xx = String::from("90");
    let yy = xx;
    println!("string1 is {}", yy);
    //println!("yy is {}", xx); // xx has been moved to yy, xx cannot be used anymore (already been freed)

    let s1 = String::from("teststring");
    let s2 = s1.clone();
    println!("string2 is {} {}", s2, s1); // s1 has been deep cloned to s2, so can use both

    let ss3 = String::from("for function");
    take_ownership(ss3);
    //println!("after take_ownership {}", ss3); 
    //Passing a variable to a function will do as assignment does. so ss3 has been moved and cannot be used anymore

    let mut s3 = takes_and_gives_back(s2);
    println!("after takes_and_gives_back: {}", s3);
    //Returning values can also transfer ownership.


    /***************references*****************/
    println!("reference test, len: {}, after borrow_test:{}", borrow_test(&mut s3), s3);
    //Reference allows you to refer to some value without taking ownership of it
    //We call having references as Borrowing


    /* At any given time, you can have either one mutable reference or any number of immutable references. */
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        //let r3 = &mut s; // We cannot have a mutable reference while we have an immutable/mutable one
        println!("immutable: {} and {}, s : {}", r1, r2, s); 

        let r3 = &mut s; // no problem
        println!("mutable {}", r3); 
        /*The scopes of the immutable references r1 and r2 end after the println! 
        where they are last used, which is before the mutable reference r3 is created. 
        These scopes don’t overlap, so this code is allowed. */
    }
}

fn take_ownership(input: String) {
    println!("in take_ownership, str: {}", input);
}

fn takes_and_gives_back(input: String) -> String {
    input
}

fn borrow_test(param: &mut String) -> usize {
    param.push_str(" appended str");
    param.len()
}

fn slice_test() {
    let mut ss = String::from("slice string");
    //The type of ss here is &str: it’s a slice pointing to that specific point of the binary

    println!("slice string: {}", ss);
    let r1 = & ss[..3];
    let r2 = & ss[3..];
    //ss.clear(); // mutable borrowing (ss is actually a mutable borrowing to whole string) cannot happen simutaneously with immutable borrowing (r1/r2)
    println!("slice_test r1: {}, r2:{}", r1, r2);
    ss.clear(); // mutable borrowing ok after all immutable borrowing finished.
}


/******************** Chapter 5 *******************/
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    count: u32,
}

impl User {
    fn inc_cnt(&mut self, val: u32) {
        self.count += val;
    }

    // Associated Function
    fn create_user(email: String, username: String, count: u32) -> User {
        User {
            email,
            username,
            count
        }
    }
}

fn inc_cnt(user: &mut User) {
    user.count += 1;
}

#[derive(Debug, Clone, Copy)]
struct PointCopy(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn struct_test() -> Point {
    println!("*******************Chapter 5***********************");

    let point = Point(1,2,3);
    // by default, Rust's assignment moves the object, making the old location invalid
    //let movp = point; // will move point
    println!("point: {:?}, size: {}", point, mem::size_of_val(&point));
    println!("point: {} {} {}", point.0, point.1, point.2);

    // But if add Clone and Copy trait for struct (adding #[derive(Clone, Copy)])
    // the assignment will copy, rather than move.
    let point_copy1 = PointCopy(4,5,6);
    let point_copy2 = point_copy1; // will do copy
    println!("point_copy1 : {:?}, sizeof: {}", point_copy1, mem::size_of_val(&point_copy1));
    println!("point_copy2 : {:?}", point_copy2);

    let mut stru = User {
        email: String::from("aaa0@bbb.com"),
        username: String::from("Testname"),
        count: 0,
    };

    let aaa0 = stru.email;
    stru.email = String::from("aaa1@bbb.com");
    let user_ref = &stru;
    println!("user_ref.email: {}", user_ref.email); // immutable borrow struct, use and release
    println!("uaaa0: {}", aaa0);

    let myname = stru.username; // partial move
    //let mystru = stru; // cannot move whole struct now
    println!("struct: {} {} {}", myname, stru.email, stru.count); // can partial access non-moved value
    stru.username = myname; // recover stru


    let stru2 = User {
        username: String::from("stru2_name"),
        ..stru // will do partial move, email has been moved
    };
    //println!("...... {}", stru.email); // stru partial moved, cannot use email
    println!("stru2 test {:#?}", stru2);
    stru.email = stru2.email; // recover stru



    inc_cnt(&mut stru);
    println!("{:?}", stru);
    stru.inc_cnt(10);    
    println!("{:?}", stru);


    let new_user = User::create_user(String::from("new@new.com"), String::from("newname"), 0);
    println!("{:?}", new_user);

    // struct Point point is on heap
    return point;

}


/******************** Chapter 6 *******************/
// can associate data to each variant of the enum, so there is no need for an extra struct.
// and each variant can have different types and amounts of associated data.
enum _IpAddr {
    V4(u8, u8, u8, u8), // associate four u8 data to V4
    V6(String), // associate one String to V6
    Empty,  // associate nothing
}

#[derive(Debug)]
enum MyEnum {
    AA(u32),
    BB(String),
    CC,
}

impl MyEnum {
    fn print_val(& self) {
        match self {
            //MyEnum::CC      => println!("enum is CC"),
            MyEnum::AA(val)   => println!("enum is AA {}", val),
            MyEnum::BB(_)   => println!("enum is BB String"),
            _ => println!("enum is other"),
        }
    }
}

fn enum_test() {
    println!("*******************Chapter 6***********************");
    let en1 = MyEnum::AA(33);
    let en2 = MyEnum::BB(String::from("enum2"));
    let en3 = MyEnum::CC;
    let en4 = &MyEnum::CC;
    println!("enum1: {:#?}, enum2: {:#?}, {:#?}, {:#?}", en1, en2, en3, en4);

    en1.print_val();
    en2.print_val();
    //println!("is AA: {}", en1 == MyEnum::AA);
}

fn option_test() {
    let a = Some(5);
    let b: Option<u32> = None;

    //if let Some(3) = b { // this equals to:  if b == Some(3)
    if let Some(_) = b {
        println!("b is value {:?}", b);
    } else {
        println!("b is None");
    }
    // if let equals to below: 
    match b {
        None => println!("is none"),
        //Some(x) => println!("is value: {}", x),
        //_ => println!("rest arms"),
        Some(_) => println!("rest Some"),
    }

    let c = Some(10);

    option_test1(a);
    option_test1(c);
    //option_test1(_b); // will compile error cause None

    let sa = Some(String::from("straa"));
    //_option_test11(sa); // will move ownership here
    //println!("before option_test2: {}", sa.unwrap()); // unwrap will move ownership
    option_test2(&sa);
    println!("after option_test2: {}", sa.unwrap());
}

fn option_test1(param: Option<u32>) {
    println!("option test 1: {}", param.unwrap());
}

fn _option_test11(param: Option<String>) {
    println!("option test 11: {:#?}", param);
}

fn option_test2(param: &Option<String>) {
    for i in param.iter() {
        println!("option test2: {}", i);
    }
}