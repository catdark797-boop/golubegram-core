pub mod proof_of_work;

use std::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;

/// Whitespace Billing Gateway
/// Decentralized abstract point system for Quality of Service (QoS) tiering.
pub struct WhitespaceBilling {
    credits: AtomicU64,
}

impl WhitespaceBilling {
    pub fn new() -> Self {
        Self {
            credits: AtomicU64::new(0),
        }
    }

    pub fn top_up(&self, amount: u64) {
        self.credits.fetch_add(amount, Ordering::SeqCst);
    }

    pub fn get_balance(&self) -> u64 {
        self.credits.load(Ordering::SeqCst)
    }

    pub fn spend(&self, amount: u64) -> bool {
        let current = self.get_balance();
        if current >= amount {
            self.credits.store(current - amount, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_BILLING: WhitespaceBilling = WhitespaceBilling::new();
}
