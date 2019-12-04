use crate::crate01::module::Module;

pub struct Spacecraft {
    modules: Vec<Module>,
}

impl Spacecraft {
    pub fn new(modules: Vec<Module>) -> Spacecraft {
        Spacecraft{modules}
    }
    pub fn get_modules(&self) -> &Vec<Module> {
        &(self.modules)
    }
}