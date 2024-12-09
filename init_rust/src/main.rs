use std::fs;
use std::collections::HashMap;

struct User{
    name: String,
    age: u8,
    active: bool,
}

enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}
fn main() {
    let a =5;
    let b:i8=5;
    let c = 1.5;
    let d:f32 = 1.5;
    let e:u8 = 5;
    let f:u32 = 1000;
    // let g: i8 = 10000000; error out of range
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);
    println!("a:{} b:{} c:{} d:{} e:{} f:{}", a, b, c, d, e, f);
    // char
    let char1 = 'a';
    println!("char1: {}", char1);
    //STRINGS
    let str1 = "Hello";
    let str2 = String::from("World");
    println!("{} {}", str1, str2);
    // nth character
    let char1 = str1.chars().nth(0);
    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character found at index 0"),
    }
    println!("{}", char1.unwrap());
    // conditionals
    let temp = 15;
    if temp > 30 {
        println!("Hot");
    } else if temp < 10 {
        println!("Cold");
    } else {
        println!("Moderate");
    }
    // simple loop
    for i in 1..11 {
        println!("{}", i);
    }
    //iterating over a string
    let sentence = String::from("my name is arsh");
    let first_word = get_first_word(sentence);
    println!("{}", first_word);
    // Borrowing and refernces
    let str1 = String::from("Immutable References");
    let str2 = &str1;
    let str3 = &str1;
    println!("{}", str2);
    println!("{}", str3);
    let mut str4 = String::from("Mutable References");
    let str5 = &mut str4;
    // let str6 = &mut str4; error cannot borrow `str4` as mutable more than once at a time
    println!("{}", str5);
    // println!("{}", str6);
    // structs
    let user1 = User{
        name: String::from("Arsh"),
        age: 25,
        active: true,
    };
    println!("{}", user1.name);
    let sum = add(1,2);
    println!("{}", sum);   
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
    //error handling
    let res = fs::read_to_string("file.txt");
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    s2.push_str(" world");
    let s3 = &s1;
    println!("{}",s3);

    let num1 = 10;
    let num2 = num1;
    println!("{}",num2);
    println!("{}",num1);
    //vectors
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    println!("{:?}",vec1);
    let even_nums = even_nums(&vec1);
    println!("{:?}",vec1);
    println!("{:?}",even_nums);
    // hashmaps
    let mut users = HashMap::new();
    users.insert(String::from("Arsh"),25);
    users.insert(String::from("John"),20);
    let first_user_age = users.get("Arsh");
    match first_user_age{
        Some(age) => println!("{}",age),
        None => println!("No user found"),
    }
    let v1 = vec![1,2,3,4,5];
    let v1_iter = v1.iter();
    let v1_iter2 = v1_iter.map(|x| x+1);
    for i in v1_iter2{
        println!("{}",i);
    }
    println!("{:?}",v1);
    // exercise
    let nums = vec![1,2,3,4,5,6,7,8,9,10];
    let iter = nums.iter();
    let odd_iter = iter.filter(|x| *x%2!=0);
    for i in odd_iter{
        println!("{}",i);
    }
    let double_iter = nums.iter().filter(|x| *x%2!=0).map(|x| x*2);
    let mut new_nums = Vec::new();
    for i in double_iter{
        new_nums.push(i);
    }
    println!("{:?}",new_nums);
    // iterators on hashmaps
    let mut users = HashMap::new();
    users.insert(String::from("Arsh"),25);
    users.insert(String::from("John"),20);
    // iterating over hashmap
    for (key,value) in users.iter(){
        println!("{}: {}",key,value);
    }
    // iterating over mutable references
    for (key,value) in users.iter_mut(){
        *value += 1;
        println!("{}: {}",key,value);
    }
    // Strings
    let mut name = String::from("Arsh");
    name.push_str(" Ali");
    println!("{}",name);
    // delete
    name.replace_range(4..name.len(), "");
    println!("{}",name);
    let mut sentence2 = String::from("Hello World");
    let word2 = find_first_word(&sentence2);
    println!("{}",word2);
    println!("{}",sentence2);

}

 
 fn find_first_word(sentence: &String)-> &str{
    let mut index = 0;
    for i in sentence.chars(){
        if i == ' '{
            break;
        }
        index+=1;
    }
    return &sentence[0..index];
 }

 fn even_nums(nums:&Vec<i32>)->Vec<i32>{
    let mut ans = Vec::new();
    for num in nums{
        if num%2==0{
            ans.push(*num);
        }
    }
    return ans;
 }

 fn get_first_word(sentence: String)->String{
    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push(char);
        if char == ' ' {
            break;
        }
    }
    return ans;
 }

 fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    // print capacity, length and pointer in one statement
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
}

fn add(a:i32,b:i32)->i32{
    let z = a+b;
    z
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    }
}
 