use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddValidatorInput {
    // SBP-M1 review: remove seed
    pub seed: Seed,
    pub validator_id: ValidatorId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddValidatorOutput {
    pub validator_id: ValidatorId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveValidatorInput {
    // SBP-M1 review: remove seed
    pub seed: Seed,
    pub validator_id: ValidatorId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveValidatorOutput {
    pub validator_id: ValidatorId,
}
