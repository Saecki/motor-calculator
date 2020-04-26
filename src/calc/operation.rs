#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn is_add(&self) -> bool {
        if let Op::Add = self { true } else { false }
    }

    pub fn is_sub(&self) -> bool {
        if let Op::Sub = self { true } else { false }
    }

    pub fn is_mul(&self) -> bool {
        if let Op::Mul = self { true } else { false }
    }

    pub fn is_div(&self) -> bool {
        if let Op::Div = self { true } else { false }
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            Op::Add => true,
            Op::Mul => true,
            _ => false,
        }
    }

    pub fn inv(&self) -> Op {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }

    pub fn calc(&self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}
