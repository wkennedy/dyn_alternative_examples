fn main() {
    let trainer_enum_dog = TrainerEum {
        animal: Animal::Dog(Dog)
    };
    trainer_enum_dog.ask_animal_to_speak();

    let trainer_enum_cat = TrainerEum {
        animal: Animal::Cat(Cat)
    };
    trainer_enum_cat.ask_animal_to_speak();

    let cat: Cat = Cat {};
    let trainer_generic = TrainerGeneric {
        animal: cat,
    };
    trainer_generic.ask_animal_to_speak();

    let trainer_func_pointer_dog = TrainerFunctionalPointer { speak_fn: dog_speak };
    trainer_func_pointer_dog.ask_animal_to_speak();

    let trainer_func_pointer_cat = TrainerFunctionalPointer { speak_fn: cat_speak };
    trainer_func_pointer_cat.ask_animal_to_speak();

    let trainer_constant: TrainerConstant<1> = TrainerConstant {};
    trainer_constant.ask_animal_to_speak();
}

struct Dog;

struct Cat;

pub trait Speak {
    fn speak(&self) -> String;
}

impl Speak for Dog {
    fn speak(&self) -> String {
        "Woof!".into()
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        "Meow!".into()
    }
}

// Generic with trait bounds
fn make_animal_speak<T: Speak>(animal: &T) {
    println!("{}", animal.speak());
}

// Enum dispatch
// Create an Enum to encapsulate the types
enum Animal {
    Dog(Dog),
    Cat(Cat),
}

impl Speak for Animal {
    fn speak(&self) -> String {
        match self {
            Animal::Dog(dog) => dog.speak(),
            Animal::Cat(cat) => cat.speak(),
        }
    }
}

// Functional pointers example
fn dog_speak() -> String {
    "Woof!".into()
}

fn cat_speak() -> String {
    "Meow!".into()
}

fn make_animal_speak_fn(speak: fn() -> String) {
    println!("{}", speak());
}

// Compile-time polymorphism with const generics
trait AnimalTrait {
    fn speak_animal_trait() -> String;
}

impl AnimalTrait for Cat {
    fn speak_animal_trait() -> String {
        "Meow!".into()
    }
}

impl AnimalTrait for Dog {
    fn speak_animal_trait() -> String {
        "Woof!".into()
    }
}

fn animal_behavior<const ANIMAL_TYPE: usize>() -> String {
    match ANIMAL_TYPE {
        1 => Cat::speak_animal_trait(),
        2 => Dog::speak_animal_trait(),
        _ => "Unknown animal".into(),
    }
}

//Examples incorporating the animals

//Enum Example
pub struct TrainerEum {
    animal: Animal,
}

impl TrainerEum {
    fn ask_animal_to_speak(&self) {
        println!("{}", self.animal.speak());
    }
}

// Generic example
pub struct TrainerGeneric<T: Speak> {
    animal: T,
}

impl<T: Speak> TrainerGeneric<T> {
    fn ask_animal_to_speak(&self) {
        make_animal_speak(&self.animal)
    }
}

// Functional pointer example
pub struct TrainerFunctionalPointer {
    speak_fn: fn() -> String,
}

impl TrainerFunctionalPointer {
    fn ask_animal_to_speak(&self) {
        make_animal_speak_fn(self.speak_fn);
    }
}

// Compile-time polymorphism with const generics
pub struct TrainerConstant<const ANIMAL_TYPE: usize> {}

impl<const ANIMAL_TYPE: usize> TrainerConstant<ANIMAL_TYPE> {
    fn ask_animal_to_speak(&self) {
        println!("{}", animal_behavior::<ANIMAL_TYPE>());
    }
}