/// A enum representing a elementary arithmetic operation.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    /// Returns true if the operation is addition false otherwise.
    pub fn is_add(&self) -> bool {
        if let Op::Add = self { true } else { false }
    }

    /// Returns true if the operation is subtraction false otherwise.
    pub fn is_sub(&self) -> bool {
        if let Op::Sub = self { true } else { false }
    }

    /// Returns true if the operation is multiplication false otherwise.
    pub fn is_mul(&self) -> bool {
        if let Op::Mul = self { true } else { false }
    }

    /// Returns true if the operation is division false otherwise.
    pub fn is_div(&self) -> bool {
        if let Op::Div = self { true } else { false }
    }

    /// Returns true if the operation is commutative false otherwise.
    pub fn is_commutative(&self) -> bool {
        match self {
            Op::Add => true,
            Op::Mul => true,
            _ => false,
        }
    }

    /// Returns the inverse operation.
    pub fn inv(&self) -> Op {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }

    /// Calculates the result of the two operands.
    pub fn calc(&self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}
