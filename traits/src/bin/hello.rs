trait hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }
    fn say_something(&self) -> String;
}

struct student {}
struct teacher {}

impl hello for student {
    fn say_hi(&self) -> String {
        String::from("hello")
    }
    fn say_something(&self) -> String {
        String::from("i am student")
    }
}

impl hello for teacher {
    fn say_hi(&self) -> String {
        String::from("hello, I'm miss maina")
    }
    fn say_something(&self) -> String {
        String::from("i a good teacher")
    }
}

fn main () {
    let s = student {};
    let t = teacher {};
    println!("{}", s.say_hi());
    println!("{}", s.say_something());
    println!("{}", t.say_hi());
    println!("{}", t.say_something());
}