// impl_enums.rs

enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}

// Payments Handler Methods -->
fn pay_by_credit(amt: u64) {
    println!("Processing credit payment of: {}", amt);
}

fn pay_by_debit(amt: u64) {
    println!("Processing debit payment of: {}", amt);
}

fn paypal_redirect(amt: u64) {
    println!("Redirecting to paypal for amount: {}", amt);
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => paypal_redirect(amount),
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Credit
}

pub fn run() {
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(4500);
}
