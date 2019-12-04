#[derive(Clone, Copy)]
pub struct Module {
    mass: i32,
}

impl Module{
    pub fn new(mass: i32) -> Module {
        Module{mass}
    }

    pub fn get_mass(&self) -> i32 {
        self.mass
    }
}