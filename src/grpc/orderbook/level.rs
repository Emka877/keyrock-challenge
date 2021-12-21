use std::cmp::Ordering;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Level {
    pub exchange: String,
    pub price: f32,
    pub amount: f32,
}

impl Level {
    pub fn new(exchange: &str, price: f32, amount: f32) -> Self {
        Self {
            exchange: exchange.to_owned(),
            price,
            amount,
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(from {}) pr: {}, amt: {}", self.exchange.to_uppercase(), self.price, self.amount)
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price && self.amount == other.amount
    }
}

impl Eq for Level {}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.price.partial_cmp(&other.price).unwrap() {
            Ordering::Less => {
                Some(Ordering::Less)
            }
            Ordering::Equal => {
                if self.amount > other.amount {
                    return Some(Ordering::Greater);
                } else if self.amount == other.amount {
                    return Some(Ordering::Equal);
                }
                Some(Ordering::Less)
            }
            Ordering::Greater => {
                Some(Ordering::Greater)
            }
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.price < other.price || (self.price == other.price && self.amount < other.amount)
    }

    fn le(&self, other: &Self) -> bool {
        self.price < other.price
            || (self.price == other.price && self.amount < other.amount)
            || (self.price == other.price && self.amount == other.amount)
    }

    fn gt(&self, other: &Self) -> bool {
        self.price > other.price
            || (self.price == other.price && self.amount > other.amount)
    }

    fn ge(&self, other: &Self) -> bool {
        self.price > other.price
            || (self.price == other.price && self.amount > other.amount)
            || (self.price == other.price && self.amount == other.amount)
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.price.partial_cmp(&other.price).unwrap() {
            Ordering::Less => {
                Ordering::Less
            }
            Ordering::Equal => {
                if self.amount > other.amount {
                    return Ordering::Greater;
                } else if self.amount == other.amount {
                    return Ordering::Equal;
                }
                Ordering::Less
            }
            Ordering::Greater => {
                Ordering::Greater
            }
        }
    }
}
