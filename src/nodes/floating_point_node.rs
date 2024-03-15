use std::any::Any;
use std::fmt::{Display, Formatter};

use crate::nodes::{Node, NodeToAny, NodeType};
use crate::position::Position;
use crate::token::Token;
use crate::values::value_type::ValueType;

#[derive(Clone)]
pub struct FloatingPointNode {
    token: Token,
    size: Box<dyn ValueType>,
}

impl FloatingPointNode {
    pub fn new(token: Token, size: Box<dyn ValueType>) -> Self {
        Self { token, size }
    }

    pub fn get_float(&self) -> f64 {
        self.token
            .token_value()
            .as_ref()
            .unwrap()
            .parse::<f64>()
            .unwrap()
    }

    // get the double ieee 754 representation of the float
    pub fn ieee_754(&self) -> u64 {
        let mut ieee_754: u64 = 0;
        let mut exponent: u64 = 0;
        let mut mantissa: u64 = 0;

        let float = self.get_float();

        if float == 0.0 {
            return ieee_754;
        }

        // get the sign bit
        if float < 0.0 {
            ieee_754 |= 1 << 63;
        }

        // get the exponent
        let mut float = float.abs();
        while float >= 2.0 {
            float /= 2.0;
            exponent += 1;
        }
        while float < 1.0 {
            float *= 2.0;
            exponent -= 1;
        }
        exponent += 1023;
        ieee_754 |= exponent << 52;

        // get the mantissa
        float -= 1.0;
        for i in 0..52 {
            float *= 2.0;
            if float >= 1.0 {
                mantissa |= 1 << (51 - i);
                float -= 1.0;
            }
        }
        ieee_754 |= mantissa;

        ieee_754
    }

    pub fn size(&self) -> &Box<dyn ValueType> {
        &self.size
    }
}

impl Display for FloatingPointNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<FloatingPointNode>[Token: {}, Size: {}]",
            self.token, self.size
        )
    }
}

impl NodeToAny for FloatingPointNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for FloatingPointNode {
    fn pos_start(&self) -> &Position {
        &self.token.pos_start()
    }

    fn pos_end(&self) -> &Position {
        &self.token.pos_end()
    }

    fn node_type(&self) -> NodeType {
        NodeType::FloatingPoint
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
