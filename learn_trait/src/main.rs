trait Summary{
    fn summarize(&self)-> String;
}
trait Fix{
    fn fix(&self)-> String;
}

struct User{
    name: String,
    age: u32,
}

impl Summary for User{
    fn summarize(&self)-> String{
        println!("Read more...{}{}",self.name,self.age);
        String::from("Read more...")
    }
}
impl Fix for User{
    fn fix(&self)-> String{
        println!("Fixing...{}{}",self.name,self.age);
        String::from("Fixing...")
    }
}

fn notify<T: Summary + Fix>(item: T){
    println!("{}",item.summarize());
    println!("{}",item.fix());
}

fn main() {
    let user = User{name: String::from("Arsh"), age: 25};
    notify(user);
}
