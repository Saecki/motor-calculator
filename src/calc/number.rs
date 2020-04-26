use std::ops::{Add, Div, Mul, Sub};

const METRIC_PREFIXES: [(&str, f64); 6] = [
    ("n", 0.000_000_001),
    ("µu", 0.000_001),
    ("m", 0.001),
    ("k", 1_000.0),
    ("M", 1_000_000.0),
    ("G", 1_000_000_000.0),
];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Num {
    In(f64),
    Out(f64),
    None,
}

impl Num {
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

    pub fn is_num(&self) -> bool {
        match self {
            Num::In(_) => true,
            Num::Out(_) => true,
            Num::None => false,
        }
    }

    pub fn as_option(&self) -> Option<f64> {
        match self {
            Num::In(v) => Some(*v),
            Num::Out(v) => Some(*v),
            Num::None => None,
        }
    }

    /// Returns the numbers value if there is one panics otherwise.
    pub fn num(&self) -> f64 {
        match self {
            Num::In(v) => *v,
            Num::Out(v) => *v,
            Num::None => panic!("Error unwrapping a Number::None"),
        }
    }

    pub fn display(&self) -> String {
        if self.is_num() {
            format!("{}", self.num())
        } else {
            String::new()
        }
    }

    pub fn parse(str: impl Into<String>) -> Num {
        let mut s = str.into().replace(",", ".");
        let mut factor = 1.0;

        'outer: for ms in &METRIC_PREFIXES {
            for c in ms.0.chars() {
                if s.ends_with(c) {
                    s.pop();
                    factor = ms.1;
                    break 'outer;
                }
            }
        }

        if let Ok(v) = s.parse::<f64>() {
            Num::In(v * factor)
        } else {
            Num::None
        }
    }
}

impl Add<f64> for Num {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        match self {
            Num::In(v) => Num::In(v + rhs),
            Num::Out(v) => Num::Out(v + rhs),
            Num::None => Num::None,
        }
    }
}

impl Add<Num> for Num {
    type Output = Self;

    fn add(self, rhs: Num) -> Self::Output {
        if rhs.is_none() { return Num::None; }

        match self {
            Num::In(v) => Num::In(v + rhs.num()),
            Num::Out(v) => Num::Out(v + rhs.num()),
            Num::None => Num::None,
        }
    }
}

impl Sub<f64> for Num {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        match self {
            Num::In(v) => Num::In(v - rhs),
            Num::Out(v) => Num::Out(v - rhs),
            Num::None => Num::None,
        }
    }
}

impl Sub<Num> for Num {
    type Output = Self;

    fn sub(self, rhs: Num) -> Self::Output {
        if rhs.is_none() { return Num::None; }

        match self {
            Num::In(v) => Num::In(v - rhs.num()),
            Num::Out(v) => Num::Out(v - rhs.num()),
            Num::None => Num::None,
        }
    }
}

impl Mul<f64> for Num {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Num::In(v) => Num::In(v * rhs),
            Num::Out(v) => Num::Out(v * rhs),
            Num::None => Num::None,
        }
    }
}

impl Mul<Num> for Num {
    type Output = Self;

    fn mul(self, rhs: Num) -> Self::Output {
        if rhs.is_none() { return Num::None; }

        match self {
            Num::In(v) => Num::In(v * rhs.num()),
            Num::Out(v) => Num::Out(v * rhs.num()),
            Num::None => Num::None,
        }
    }
}

impl Div<f64> for Num {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        match self {
            Num::In(v) => Num::In(v / rhs),
            Num::Out(v) => Num::Out(v / rhs),
            Num::None => Num::None,
        }
    }
}

impl Div<Num> for Num {
    type Output = Self;

    fn div(self, rhs: Num) -> Self::Output {
        if rhs.is_none() { return Num::None; }

        match self {
            Num::In(v) => Num::In(v / rhs.num()),
            Num::Out(v) => Num::Out(v / rhs.num()),
            Num::None => Num::None,
        }
    }
}
