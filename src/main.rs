mod crate01 {
    pub mod main01;
    mod module;
    mod spacecraft;
    mod fuel_calculator;
}
mod crate02 {
    pub mod main02;
}
mod crate03 {
    pub mod main03;
    mod point;
    mod wire;
}/*

mod crate04 {
    pub mod main04;
    }

mod crate05 {
    pub mod main05;
}

mod crate06 {
    pub mod main06;
    }

mod crate07 {
    pub mod main07;
}*/

use crate::crate01::main01;
use crate::crate02::main02;
use crate::crate03::main03;/*
use crate::crate04::main04;
use crate::crate05::main05;
use crate::crate06::main06;
use crate::crate07::main07;*/
fn main() {
    if false {
        main01::main01();
        main02::main02();
        main03::main03();
    }
}