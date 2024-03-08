


fn main() {
   let number_list = vec![1,2,3,4];
  
   let number_list1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

   println!("{}",find_largest_number(&number_list));
   println!("{}",find_largest_number(&number_list1));
   println!("{}", find_largest_number(&['c', 'b', 'c', 'd', 'a', 'z']));


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