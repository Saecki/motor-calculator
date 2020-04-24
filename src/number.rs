const METRIC_PREFIXES: [(&str, f64); 6] = [
    ("n", 0.000_000_001),
    ("µu", 0.000_001),
    ("m", 0.001),
    ("k", 1_000.0),
    ("M", 1_000_000.0),
    ("G", 1_000_000_000.0),
];

#[derive(Copy, Clone, Debug)]
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

    pub fn add(&self, other: f64) -> Num {
        match self {
            Num::In(v) => Num::In(v + other),
            Num::Out(v) => Num::Out(v + other),
            Num::None => Num::None,
        }
    }

    pub fn sub(&self, other: f64) -> Num {
        match self {
            Num::In(v) => Num::In(v - other),
            Num::Out(v) => Num::Out(v - other),
            Num::None => Num::None,
        }
    }

    pub fn mul(&self, other: f64) -> Num {
        match self {
            Num::In(v) => Num::In(v * other),
            Num::Out(v) => Num::Out(v * other),
            Num::None => Num::None,
        }
    }

    pub fn div(&self, other: f64) -> Num {
        match self {
            Num::In(v) => Num::In(v / other),
            Num::Out(v) => Num::Out(v / other),
            Num::None => Num::None,
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
