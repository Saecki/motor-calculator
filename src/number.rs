#[derive(Copy, Clone, Debug)]
pub enum Num {
    In(f64),
    Out(f64),
    None,
}

impl Num {
    pub fn is_num(&self) -> bool {
        match self {
            Num::In(_) => true,
            Num::Out(_) => true,
            Num::None => false,
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            Num::In(_) => true,
            _ => false,
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            Num::Out(_) => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Num::None => true,
            _ => false,
        }
    }

    pub fn as_option(&self) -> Option<f64> {
        match self {
            Num::In(v) => Some(*v),
            Num::Out(v) => Some(*v),
            Num::None => None,
        }
    }

    pub fn num(&self) -> f64 {
        match self {
            Num::In(v) => *v,
            Num::Out(v) => *v,
            Num::None => panic!("Error unwrapping a Number::None"),
        }
    }
}
