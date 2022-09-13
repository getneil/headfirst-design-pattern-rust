use std::fmt::{self, Debug, Display};

#[derive(Clone, Copy)]
enum BeverageSize {
    Tall,
    Grande,
    Venti,
}
impl fmt::Display for BeverageSize {
    // adds to_string() functionality to enum as used in `self.size.to_string()` below
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BeverageSize::Tall => write!(f, "Tall"),
            BeverageSize::Grande => write!(f, "Grande"),
            BeverageSize::Venti => write!(f, "Venti"),
        }
    }
}

struct BeverageBase {
	name: String,
    cost: i32,
    size: BeverageSize
}

trait Compute {
	fn get_name(&self) -> String;
    fn get_cost(&self) -> i32;
    fn get_size(&self) -> BeverageSize;
    // fn change_size(&mut self);
}
impl Compute for BeverageBase {
	fn get_name(&self) -> String {
		format!("{} {}", self.size.to_string(), self.name.clone())
	}
    fn get_cost(&self) -> i32 {
        self.cost
    }
    fn get_size(&self) -> BeverageSize {
        self.size
    }
}

struct Milk<T> {
	base: T,
}

impl<T:Compute> Compute for Milk<T> {
	fn get_name(&self) -> String {
        format!("{} milk", self.base.get_name())
    }
    fn get_cost(&self) -> i32 {
        let mut cost = 30;
        let size = self.base.get_size();
        match size {
            BeverageSize::Tall => cost = 30,
            BeverageSize::Grande => cost = 35,
            BeverageSize::Venti => cost = 40,
        }
        cost + self.base.get_cost()
    }
    fn get_size(&self) -> BeverageSize {
        self.base.get_size()
    }
}

struct WhipCream<T> {
    base: T,
}

impl<T:Compute> Compute for WhipCream<T> {
	fn get_name(&self) -> String {
        format!("{} whipcream", self.base.get_name())
    }
    fn get_cost(&self) -> i32 {
        let mut cost = 30;
        let size = self.base.get_size();
        match size {
            BeverageSize::Tall => cost = 40,
            BeverageSize::Grande => cost = 45,
            BeverageSize::Venti => cost = 50,
        }
        cost + self.base.get_cost()
    }
    fn get_size(&self) -> BeverageSize {
        self.base.get_size()
    }
}

/**
 * REad more: 
 * - http://doc.rust-lang.org/tutorial.html#declaring-and-implementing-traits
 */
pub fn run() {
	let coffee = BeverageBase {
        name: "Coffee".to_string(),
        cost: 35,
        size: BeverageSize::Tall
    };
	let coffee_with_milk = Milk{base: coffee};
	println!("{} cost:{}", coffee_with_milk.get_name(), coffee_with_milk.get_cost());

    let with_cream = WhipCream{base: coffee_with_milk};
	println!("{} cost:{}", with_cream.get_name(), with_cream.get_cost());

    let dark_roast = BeverageBase {
        name: "Dark Roast".to_string(),
        cost: 55,
        size: BeverageSize::Venti
    };
	println!("{} cost:{}", dark_roast.get_name(), dark_roast.get_cost());
	let dr_with_milk = Milk{base: dark_roast};
	println!("{} cost:{}", dr_with_milk.get_name(), dr_with_milk.get_cost());

}