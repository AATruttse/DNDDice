use crate::dices::IntValue;
use crate::errors::DiceError;

/// Type for generation methods
pub type GenMethod = fn (&mut Vec<IntValue>) -> Result<(), DiceError>;

/// struct for generation method
pub struct Method {
    statlist:  &'static str,
    is_ordered: bool,

    desc:      &'static str,
    desc_long: &'static str,

    method:    GenMethod
}

impl Method {
    pub fn get_statlist(&self) -> &str {
        self.statlist
    }

    pub fn is_ordered(&self) -> bool {
        self.is_ordered
    }

    pub fn get_desc(&self) -> &str {
        self.desc
    }

    pub fn get_desc_long(&self) -> &str {
        self.desc_long
    }

    pub fn get_method(&self) -> GenMethod {
        self.method
    }

    pub fn new(
        statlist:   &'static str,
        is_ordered: bool,
        desc:       &'static str,
        desc_long:  &'static str,
        method:     GenMethod
    ) -> Self {
        let new_method = Method {
            statlist: statlist,
            is_ordered: is_ordered,
            desc: desc,
            desc_long: desc_long,
            method: method
        };

        new_method
    }
}
