enum PizzaType {
    Hawaiian,
    Pepperoni,
    Greek
}

trait Pizza {
    fn get_name(&self) -> String;
    fn bake(&self);
}

trait Store {
    fn create_pizza(&self, pt: &PizzaType) -> Box<dyn Pizza>;
    fn order_pizza(&self, pt: &PizzaType) -> Box<dyn Pizza>;
}

struct HawaiianPizza {
    name: String,
}
impl Pizza for HawaiianPizza {
    fn bake(&self) {
        println!("baking {}", &self.name);
    }
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

struct PepperoniPizza {
    name: String,
}
impl Pizza for PepperoniPizza {
    fn bake(&self) {
        println!("baking {}", &self.name);
    }
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

struct GreekPizza {
    name: String,
}
impl Pizza for GreekPizza {
    fn bake(&self) {
        println!("baking {}", &self.name);
    }
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

struct NyStore;
impl Store for NyStore {
    fn create_pizza(&self, pt: &PizzaType) -> Box<dyn Pizza> {
        match pt {
            PizzaType::Greek => Box::new(GreekPizza {
                name: "NY Style: Greek Pizza".to_string(),
            }),
            PizzaType::Hawaiian => Box::new(HawaiianPizza {
                name: "Ny Style: Hawaiian Pizza".to_string(),
            }),
            PizzaType::Pepperoni => Box::new(PepperoniPizza {
                name: "NY Style: Pepperoni Pizza".to_string(),
            }),
        }
    }

    fn order_pizza(&self, pt: &PizzaType) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(pt);
        pizza.bake();
        pizza
    }
}


struct ChicagoStore;
impl Store for ChicagoStore {
    fn create_pizza(&self, pt: &PizzaType) -> Box<dyn Pizza> {
        match pt {
            PizzaType::Greek => Box::new(GreekPizza {
                name: "Deep Dish Style: Greek Pizza".to_string(),
            }),
            PizzaType::Hawaiian => Box::new(HawaiianPizza {
                name: "Deep Dish Style: Hawaiian Pizza".to_string(),
            }),
            PizzaType::Pepperoni => Box::new(PepperoniPizza {
                name: "Deep Dish Style: Pepperoni Pizza".to_string(),
            }),
        }
    }

    fn order_pizza(&self, pt: &PizzaType) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(pt);
        pizza.bake();
        pizza
    }
}

pub fn run() {
    let ny_store = NyStore {};

    let greek_ny_pizza = ny_store.order_pizza(&PizzaType::Greek);
    println!("ny: {}", greek_ny_pizza.get_name());

    let chicago_store = ChicagoStore {};
    let pepperoni_chicago_pizza = chicago_store.order_pizza(&PizzaType::Pepperoni);
    println!("chicago: {}", pepperoni_chicago_pizza.get_name());

    let ny_hawaiian_pizza = ny_store.create_pizza(&PizzaType::Hawaiian);
    println!("make pizza dont bake {}", ny_hawaiian_pizza.get_name());
}
