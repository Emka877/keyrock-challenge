use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Level {
    pub exchange: String,
    pub price: f32,
    pub amount: f32,
}

impl Level {
    pub fn new(exchange: String, price: f32, amount: f32) -> Self {
        Self {
            exchange,
            price,
            amount,
        }
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
        if self.price > other.price {
            return Some(Ordering::Greater);
        } else if self.price == other.price && self.amount > other.amount {
            return Some(Ordering::Greater);
        } else if self.price == other.price && self.amount == other.amount {
            return Some(Ordering::Equal);
        }
        Some(Ordering::Less)
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
        if self.price > other.price {
            return Ordering::Greater;
        } else if self.price == other.price && self.amount > other.amount {
            return Ordering::Greater;
        } else if self.price == other.price && self.amount == other.amount {
            return Ordering::Equal;
        }
        Ordering::Less
    }
}