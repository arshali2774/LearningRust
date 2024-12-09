fn longest_string_function<'a>(a:&'a str,b:&'a str) -> &'a str{
    if a.len()>b.len(){
        a
    }else{
        b
    }
}

fn main() {
    let longest_string;
    let string1 = String::from("Hello");
    {
        let string2 = String::from("Worldzz");
        longest_string = longest_string_function(&string1,&string2);
    }
    println!("{}",longest_string);
}
