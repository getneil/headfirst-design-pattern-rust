/**
0 - determine which things stay constant and those that changes, then group them
1 - program to an interface over implementation, Run Time vs Compile Time, Program to Supertype
*/

trait SoundBehaviour {
    fn make_sound(&self, duck: &Duck);
}

struct Duck {
    name: String,
    // description: String, // display
    sound_behaviour: Box<dyn SoundBehaviour>,
}

struct DefaultQuack;
impl SoundBehaviour for DefaultQuack {
    fn make_sound(&self, duck: &Duck) {
        println!("{}: quack", duck.name);
    }
}

struct SqueakQuack;
impl SoundBehaviour for SqueakQuack {
    fn make_sound(&self, duck: &Duck) {
        println!("{}: squeak", duck.name);
    }
}

impl Default for Duck {
    fn default() -> Self {
        Duck {
            name: "nameless".to_string(),
            // description: "a generic duck".to_string(),
            sound_behaviour: Box::new(DefaultQuack{})
        }
    }
}

trait ConstantDuckTraits {
    fn quack(&self);
    fn set_quack_behaviour(&mut self, sound_behaviour: Box<dyn SoundBehaviour>);
}

impl ConstantDuckTraits for Duck {
    fn quack(&self) {
        self.sound_behaviour.make_sound(self);
    }
    fn set_quack_behaviour(&mut self, sound_behaviour: Box<dyn SoundBehaviour>) {
        self.sound_behaviour = sound_behaviour;
    }
}



/**
 * Notes:
 *  - there is no such thing as class extend in Rust, I initially wanted something like struct RubberDuck extends Duck
 *  - mastering Ownership concept is important
 */
pub fn run() {
    // let "mut" is important because we are mutating it in fn set_quack_behaviour
    // in a way its an additional cogload to users(developers) but intellisense catches it easily
    // this would help -> https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
    // Ownership is a unique concept in Rust would help a lot if you understand it
    let mut default_duck = Duck {
        name: "donald".to_string(),
        ..Default::default()
    };
    default_duck.quack();

    let rubber_duck: Duck = Duck {
        name: "Rubber Duck".to_string(),
        sound_behaviour: Box::new(SqueakQuack{})
    };

    rubber_duck.quack();

    default_duck.set_quack_behaviour(Box::new(SqueakQuack{}));
    default_duck.quack();

    // Not going to work unless let "mut" rubber_duck = 
    // rubber_duck.set_quack_behaviour(Box::new(DefaultQuack{}));
    // rubber_duck.quack();


}