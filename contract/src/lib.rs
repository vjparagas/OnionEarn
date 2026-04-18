use std::collections::HashMap;

#[derive(Clone)]
pub struct Escrow {
    pub buyer: String,
    pub farmer: String,
    pub amount: i128,
    pub delivered: bool,
}

pub struct EscrowStore {
    escrows: HashMap<String, Escrow>,
}

impl EscrowStore {
    pub fn new() -> Self {
        Self { escrows: HashMap::new() }
    }

    pub fn create_escrow(&mut self, id: &str, buyer: &str, farmer: &str, amount: i128) {
        let escrow = Escrow {
            buyer: buyer.to_string(),
            farmer: farmer.to_string(),
            amount,
            delivered: false,
        };
        self.escrows.insert(id.to_string(), escrow);
    }

    pub fn confirm_delivery(&mut self, id: &str, buyer: &str) {
        let escrow = self.escrows.get_mut(id).expect("Escrow not found");
        if escrow.buyer != buyer {
            panic!("Not authorized");
        }
        escrow.delivered = true;
    }

    pub fn release_funds(&mut self, id: &str) -> i128 {
        let escrow = self.escrows.get(id).expect("Escrow not found");
        if !escrow.delivered {
            panic!("Delivery not confirmed");
        }
        let amount = escrow.amount;
        self.escrows.remove(id);
        amount
    }

    pub fn get_escrow(&self, id: &str) -> &Escrow {
        self.escrows.get(id).expect("Escrow not found")
    }
}

#[cfg(test)]
mod test;
