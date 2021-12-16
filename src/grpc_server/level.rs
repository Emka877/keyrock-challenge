
pub struct Level {
    exchange: String,
    price: f32,
    amount: f32,
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
