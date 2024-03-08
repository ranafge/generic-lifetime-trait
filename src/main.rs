

// struct Point<T> {
//     x: T,
//     y: T
// }
use generic_trait_lifetime::triat_impl::{self, notify, NewsArticle, Summary, Tweet};


// Method definitions
struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T, U> Point<T, U> {
//     fn new(&self) -> Point<T, U> {
//         Point {
//             x: self.x,
//             y: self.y,
//         }
//     }

//     fn x(&self) -> &T {
//         &self.x
//     }
// }
impl<T1, U1> Point<T1, U1> {
    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T1, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    some(T),
    None,
}
// enum can be a multiple generic type like result enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn main() {

    let na = NewsArticle {
        headline: String::from("hello"),
        content: String::from("world"),
        location: String::from("china"),
        author: String::from("rust"),
    };

    let tx = Tweet{
        username: String::from("rust"),
        content: String::from("hello"),
        reply: String::from("world"),
        retweet: false
    };
    println!("{}",na.summarize());
    println!("{}",tx.summarize());
    println!("{:?}", notify(&na));
  
    
    let number_list = vec![1, 2, 3, 4];

    let number_list1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("{}", find_largest_number(&number_list));
    println!("{}", find_largest_number(&number_list1));
    println!("{}", find_largest_number(&['c', 'b', 'c', 'd', 'a', 'z']));

    // for struct (generic)
    // let point1 = Point { x: 10, y: 20.0 };
    // let point2 = Point { x: 30.8, y: 40 };
    // let point3 = Point { x: 50, y: 60.0 };
   

    // let point4 = point1.mixup(point2);

    // println!("point1.x = {}, point1.y = {}", point1.x, point1.y);
 // println!("point2.x = {}, point2.y = {}", point2.x, point2.y);
}

fn find_largest_number<T: std::cmp::PartialOrd + std::fmt::Debug>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for num in list.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is {:?}", largest);
    largest
}

fn find_largest_char(list: &[char]) -> char {
    let mut largest = &list[0];
    for ch in list.iter() {
        if ch > largest {
            largest = ch;
        }
    }
    println!("The largest char is {}", &largest);
    *largest
}
