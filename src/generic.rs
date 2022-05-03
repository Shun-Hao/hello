

pub fn log_generic() {
        println!("*******************Chapter 10***********************");
}

struct _Point<T> {
        x: T,
        y: T,
}

// use impl<T> to indicate generic in implementation
impl<T> _Point<T> {
        fn _fun_x(&self) -> &T {
                &self.x
        }
}

struct Papers {
        pages: u32,
        name:  String
}

struct Twitter {
        url: String,
        name:  String
}

struct NewMedia {
        _tbd: u8
}


trait Summary {
        fn summarize(&self) -> String {
                format!("summary empty")
        }
}

trait Publish {
        fn publish(&self) -> String {
                format!("publishing")
        }
}

impl Publish for Papers {}
impl Summary for Papers {
        fn summarize(&self) -> String {
            format!("name: {}, pages: {}", self.name, self.pages)
        }
}

impl Summary for Twitter {
        fn summarize(&self) -> String {
            format!("url: {}, name: {}", self.url, self.name)
        }
}

impl Summary for NewMedia {}
impl Publish for NewMedia {}

pub fn test_trait()
{
        let twi = Twitter {
                url: String::from("www.test.com"),
                name: String::from("Twitter")
        };

        test_trait_param(&twi);

        let nm = NewMedia {
                _tbd: 1
        };
        test_generic_trait(&nm);
}

fn test_trait_param (param: &impl Summary) {
        println!("output summary: {}", param.summarize());
}

fn test_generic_trait<T>(param: &T) 
        where T: Summary + Publish {
        println!("OUTPUT SUMMARY:{}", param.summarize());
        println!("OUTPUT PUBLISH:{}", param.publish());
}

// PartialOrd is the crait who implements compare
fn largest<T: PartialOrd> (list: &[T]) -> &T{
        let mut largest = &list[0];
        for i in list {
                if i > largest {
                        largest = i;
                }
        }
        return largest;
}

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

pub fn test_generic () {
        let li = [1,5,39,9,20,5];
        println!("li largest:{}", largest(&li));
        let ch = ['c', 't', '2', ' '];
        println!("ch largest:{}", largest(&ch));
        let vc = vec![35,67,994,43543,34,33,22];
        println!("vc largest:{}", largest(&vc));
        let mut vcs = vec!["aa", "etrw", "33", "22"];
        let new_str = String::from("new string");
        vcs.push(&new_str);
        println!("vcs largest:{}", largest(&vcs));

        let str1 = String::from("str1111");
        {
                let str2 = String::from("str22");
                println!("lifetime test:{}", longest_str(str1.as_str(), &str2));
        }

}