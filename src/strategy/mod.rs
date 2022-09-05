/**
0 - determine which things stay constant and those that changes, then group them
1 - program to an interface over implementation, Run Time vs Compile Time, Program to Supertype
*/

struct Duck {
    name: String,
    description: String // display
    // soundBehaviour: SoundBehaviour - enum
}

// enum SoundBehaviour {

// }

impl Default for Duck {
    fn default() -> Self {
        Duck {
            name: "unknown".to_string(),
            description: "a generic duck".to_string()
        }
    }
}

trait ConstantDuckTraits {
    fn trySound(&self);
    fn tryFly(&self);
    fn swim(&self);
}

impl ConstantDuckTraits for Duck {
    fn trySound(&self) {
        println!("try make sound! {}", self.name);
    }
    fn swim(&self) {
        println!("swimming! {}", self.description);
    }
    fn tryFly(&self) {
        println!("try flying! {}", self.name)
    }
}



pub fn run() {
    let duck1 = Duck {
        name: "donald".to_string(),
        ..Default::default()
    };
    duck1.swim();
    duck1.trySound();
    duck1.tryFly();
}