// struct Point<T> {
//     x: T,
//     y: T
// }
struct Point<T, U> {
    x: T,
    y: U
}

fn main() {
   let number_list = vec![1,2,3,4];
  
   let number_list1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

   println!("{}",find_largest_number(&number_list));
   println!("{}",find_largest_number(&number_list1));
   println!("{}", find_largest_number(&['c', 'b', 'c', 'd', 'a', 'z']));

   // for struct (generic)
   let point1 = Point { x: 10, y: 20.0 };
   let point2 = Point { x: 30.8, y: 40 };
 

   println!("point1.x = {}, point1.y = {}", point1.x, point1.y);
   println!("point2.x = {}, point2.y = {}", point2.x, point2.y);
 

}




fn find_largest_number<T: std::cmp::PartialOrd + std::fmt::Debug>(list: &[T]) -> &T {
   
    let mut largest = &list[0];
    for num in list.iter(){
     if num > largest {
         largest = num;
     }
    }
    println!("The largest number is {:?}", largest);
    largest
 
 }
 
 fn find_largest_char(list : &[char]) -> char {
    let mut largest = &list[0];
    for ch in list.iter(){
        if ch > largest {
            largest = ch;
        }
    }
    println!("The largest char is {}", &largest);
    *largest
 }