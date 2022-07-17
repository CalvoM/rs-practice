pub enum EatingType {
    PLANTS,
    MEAT,
}

pub trait Animal {
    fn eat(&self);
}

pub struct Herbivore{
    pub name: String
}
pub struct Carnivore{
    pub name: String
}

impl Animal for Herbivore {
    fn eat(&self) {
        println!("{} eats plants!", self.name);
    }
}

impl Animal for Carnivore {
    fn eat(&self) {
        println!("{} eats flesh!", self.name);
    }
}

pub fn eating(eating: EatingType) -> Box<dyn Animal> {
    match eating{
        EatingType::PLANTS => Box::new(Herbivore{name:String::from("herbivore")}),
        EatingType::MEAT => Box::new(Carnivore{name:String::from("carnivore")})
    }
}
