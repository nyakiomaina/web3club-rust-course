trait Animal {
    fn noise(&self) -> String;
}

fn make_animal_noise<T: Animal>(animal: &T){
    println!("{}", animal.noise());
}

struct Cow;
struct Sheep;

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaa".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooo".to_string()
    }
}

fn main() {
    let cow = Cow;
    let sheep = Sheep;

    make_animal_noise(&cow);
    make_animal_noise(&sheep);
}