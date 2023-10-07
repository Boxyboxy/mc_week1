
#[derive(PartialEq, Debug)]
enum PaymentType {
    DigitalToken,
    Cash
}

struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32
}

#[derive(PartialEq, Debug)]
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32
}

struct  BuyerGroup {
    members: Vec<Buyer>
}

impl BuyerGroup {
    fn add_member(&mut self, new_buyer: Buyer) {
        self.members.push(new_buyer);
    }

    fn find_buyer(&self, payment_type: PaymentType) -> usize {
        for i in 0..self.members.len() {
            if self.members[i].payment_type == payment_type {
                return i;
            } 
        }
        return usize::MAX;
    }

    fn buy(&mut self, buyer_index: usize, seller: &mut Seller){    
        while self.members[buyer_index].balance >= seller.price {
            self.members[buyer_index].balance -= seller.price;
            seller.balance += seller.price;
        }
    }

}




fn main() {
    let  john = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.0
    };

    let  sally = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.0
    };    

    let mut buyer_group = BuyerGroup {
        members: Vec::new()
    };

    buyer_group.add_member(john);
    buyer_group.add_member(sally);

    let buyer_index = buyer_group.find_buyer(PaymentType::Cash);

    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0
    };

    buyer_group.buy(buyer_index, &mut seller);
}

// TODO: extra write test cases
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_add_member() {
        let john = Buyer {
            name: "John".to_owned(),
            payment_type: PaymentType::DigitalToken,
            balance: 100.0
        };

        let sally = Buyer {
            name: "Sally".to_owned(),
            payment_type: PaymentType::Cash,
            balance: 55.5
        };

        let mut buyer_group = BuyerGroup {
            members: Vec::new()
        };

        buyer_group.add_member(john);
        buyer_group.add_member(sally);

        let  john1 = Buyer {
            name: "John".to_owned(),
            payment_type: PaymentType::DigitalToken,
            balance: 100.0
        };

        let sally1 = Buyer {
            name: "Sally".to_owned(),
            payment_type: PaymentType::Cash,
            balance: 55.5
        };
        assert_eq!(buyer_group.members[0], john1);
        assert_eq!(buyer_group.members[1], sally1);
    }

    #[test]
    fn check_find_member() {
        let john = Buyer {
            name: "John".to_owned(),
            payment_type: PaymentType::DigitalToken,
            balance: 100.0
        };

        let sally = Buyer {
            name: "Sally".to_owned(),
            payment_type: PaymentType::Cash,
            balance: 55.5
        };

        let mut buyer_group = BuyerGroup {
            members: Vec::new()

        };
        buyer_group.add_member(john);
        buyer_group.add_member(sally);

        let buyer_index = buyer_group.find_buyer(PaymentType::Cash);
        assert_eq!(buyer_index , 1);

        let mut seller = Seller {
            payment_type: PaymentType::Cash,
            price: 5.5,
            balance: 0.0
        };

        buyer_group.buy(buyer_index, &mut seller);
        assert_eq!(buyer_group.members[buyer_index].balance, 0.5);
        assert_eq!( seller.balance, 55.0);
    }

    #[test]
    fn check_buy() {
        let john = Buyer {
            name: "John".to_owned(),
            payment_type: PaymentType::DigitalToken,
            balance: 100.0
        };

        let sally = Buyer {
            name: "Sally".to_owned(),
            payment_type: PaymentType::Cash,
            balance: 55.5
        };

        let mut buyer_group = BuyerGroup {
            members: Vec::new()

        };
        buyer_group.add_member(john);
        buyer_group.add_member(sally);

        let buyer_index = buyer_group.find_buyer(PaymentType::Cash);
        assert_eq!(buyer_index , 1);

        let mut seller = Seller {
            payment_type: PaymentType::Cash,
            price: 5.5,
            balance: 0.0
        };

        buyer_group.buy(buyer_index, &mut seller);
        assert_eq!(buyer_group.members[buyer_index].balance, 0.5);
        assert_eq!( seller.balance, 55.0);
    }

    
}