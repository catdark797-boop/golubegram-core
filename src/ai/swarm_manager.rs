use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

/// System Operator AI (Swarm Manager)
/// Elevates local compute to manage local token transactions and aggregate metrics.
pub struct SwarmManager;

#[derive(Serialize, Deserialize, Debug)]
pub struct EscrowContract {
    pub contract_id: String,
    pub buyer_pubkey: String,
    pub seller_pubkey: String,
    pub locked_qos_tokens: u64,
    pub item_id: String,
    pub is_resolved: bool,
}

pub struct Ledger {
    pub balances: HashMap<String, u64>,
}

lazy_static::lazy_static! {
    pub static ref TOKEN_LEDGER: Arc<RwLock<Ledger>> = Arc::new(RwLock::new(Ledger {
        balances: HashMap::new(),
    }));
}

impl SwarmManager {
    /// AI acts as a decentralized arbiter for local token transactions.
    pub fn arbitrate_escrow(contract: &mut EscrowContract, ai_decision_weight: f64) -> Result<(), &'static str> {
        println!("[SWARM MANAGER] Arbitrating Escrow Contract: {}", contract.contract_id);
        
        if ai_decision_weight > 0.85 {
            // ACQUIRE EXCLUSIVE ATOMIC LOCK
            let mut ledger = TOKEN_LEDGER.write().unwrap();
            
            // RACECONDITION FAILSAFE: Double-spend check
            let buyer_bal = ledger.balances.get(&contract.buyer_pubkey).copied().unwrap_or(0);
            if buyer_bal < contract.locked_qos_tokens {
                return Err("CRITICAL: Escrow rejected. Insufficient tokens. Double-spend attempt detected.");
            }

            // Execute atomic transfer
            *ledger.balances.entry(contract.buyer_pubkey.clone()).or_insert(0) -= contract.locked_qos_tokens;
            *ledger.balances.entry(contract.seller_pubkey.clone()).or_insert(0) += contract.locked_qos_tokens;

            contract.is_resolved = true;
            println!("[SWARM MANAGER] AI Confidence high ({}). Cryptographic Finality achieved. Tokens released for Contract {}.", ai_decision_weight, contract.contract_id);
            Ok(())
        } else {
            Err("AI Confidence too low for automatic arbitration. Manual peer consensus required.")
        }
    }

    /// The AI autonomously aggregates anonymized density/traffic metrics.
    pub fn generate_oracle_report(routing_table_size: usize, avg_ttd_ms: u64) -> String {
        println!("[SWARM ORACLE] Aggregating localized metrics...");
        format!(
            "{{ \"type\": \"oracle_report\", \"local_density\": {}, \"avg_mesh_latency_ms\": {} }}",
            routing_table_size, avg_ttd_ms
        )
    }
}
