use crate::billing::GLOBAL_BILLING;

/// Proof-of-Compute Tokenomics Engine
pub struct ProofOfWorkLedger;

impl ProofOfWorkLedger {
    /// Grants QoS Tokens (Credits) directly to the user's local billing system 
    /// after successfully completing an AI micro-batch operation.
    pub fn reward_computation(batch_size: u64, precision: u8) {
        // Simple tokenomics formula: base reward scaled by batch size and FP/INT precision
        let base_reward: u64 = 5;
        let precision_multiplier = match precision {
            32 => 2, // FP32 (Full precision, higher reward)
            16 => 1, // FP16
            8  => 1, // INT8
            _  => 1, // Default fallback
        };
        
        // Final reward
        let total_reward = base_reward * batch_size * precision_multiplier;
        
        GLOBAL_BILLING.top_up(total_reward);
        println!("[TOKENOMICS] Federated Epoch Completed. Rewarded {} QoS Credits. Total Balance: {}", 
                 total_reward, GLOBAL_BILLING.get_balance());
    }
}
