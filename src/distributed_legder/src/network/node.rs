use serde::{Deserialize, Serialize};

use crate::constants::fixed_sizes::{WEIGHT_REPUTATION, WEIGHT_RISK};
use crate::network::key::Key;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Node {
    pub ip: String,
    pub port: u16,
    pub id: Key,
}

impl Node {
    pub fn new(ip: String, port: u16) -> Self {
        let node_id = format!("{}:{}", ip, port);
        let id = Key::new(node_id);

        Node { ip, port, id}
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ThrustAndReputation {
    pub total_interaction:   u32,
    pub successfully_interaction: u32,
}

impl ThrustAndReputation {
    pub fn new() -> Self {

        ThrustAndReputation{
            total_interaction: 0,
            successfully_interaction: 0
        }
    }
    pub fn update_interaction(mut self, is_successful: bool) -> ThrustAndReputation {
        self.total_interaction += 1;
        if is_successful { self.successfully_interaction += 1; }

        self
    }

    pub fn calculate_thrust_factor(self) -> f64 {
        WEIGHT_REPUTATION * self.clone().calculate_risk_integration() as f64
        + WEIGHT_RISK*self.calculate_risk_environment()
    }

    /**
     * Calculate the risk of the interaction with the given node
     *
     * The risk is given by: + 1/ Np for every correct interaction, - 2/ Np for every wrong interaction
     */
    fn calculate_risk_integration(self) -> u32 {
        let total_interaction = self.total_interaction;
        let successfully_interaction = self.successfully_interaction;

        if total_interaction == 0 { return 1; }

        (successfully_interaction - (total_interaction - successfully_interaction))
            / total_interaction
    }

    /**
    * Calculate the risk of the network
    *
    * This risk takes into account the number of low grade interactions expected by the network
    * And the total number of requests made by the peers in the network
    *
    * Since we don't have this information, we just use a relatively high risk factor of 0.3 (1 in every 3 transactions
    * Is a low grade interaction).
    */
    fn calculate_risk_environment(self) -> f64 {
        0.3
    }
}