/*
Single Layer Perceptron Test
Smart Contract
*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, require, env};
use rand::Rng;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};
    use std::convert::TryInto;
    use rand::Rng;
    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("vidur.testnet".to_string().try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn my_test() {
    let context = get_context(false);
        testing_env!(context);
        let mut rng = rand::thread_rng();
        let mut contract = InputMatrixWeight {
            height_weight: rng.gen_range(-10i32..11i32),
            weight_weight: rng.gen_range(-10i32..11i32),
            legs_weight: rng.gen_range(-10i32..11i32),
            is_alive_weight: rng.gen_range(-10i32..11i32),
            bias: 0
        };
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Default)]
pub struct InputMatrixWeight {
    height_weight: i32,
    weight_weight: i32,
    legs_weight: i32,
    is_alive_weight: i32,
    bias: i32,
}
#[near_bindgen]
impl InputMatrixWeight {
    #[init]
    pub fn new() -> Self{
        let mut rng = rand::thread_rng();
        require!(!env::state_exists(), "Already initialized");
        Self {
            height_weight: rng.gen_range(-10i32..11i32),
            weight_weight: rng.gen_range(-10i32..11i32),
            legs_weight: rng.gen_range(-10i32..11i32),
            is_alive_weight: rng.gen_range(-10i32..11i32),
            bias: 0
        }
    }
    #[private]
    pub fn predict(&self, height: &u32, weight: &u32, legs: &u32, is_alive: &u8) -> u8{
        let casted_height = *height as i32;
        let casted_weight = *weight as i32;
        let casted_legs = *legs as i32;
        let casted_is_alive =  *is_alive as i32;
        let weighted_sum = &self.bias + casted_height * &self.height_weight + casted_weight * &self.weight_weight + casted_legs * &self.legs_weight + casted_is_alive * &self.is_alive_weight;
        self.step_function(weighted_sum)
    }
    pub fn train(&mut self, inputs: [(u32, u32, u32, u8); 10], outputs: [u8; 10]){
        for i in 0..9{
            let prediction = self.predict(&inputs[i].0, &inputs[i].1, &inputs[i].2, &inputs[i].3);
            let offset: i8 = outputs[i] as i8 - prediction as i8;
            self.adjust(offset);
        }
    }
    pub fn step_function(&self, sum: i32) -> u8{
        let mut return_value: u8 = 0;
        if sum >= 0{return_value = 1}
        else if sum < 0{return_value = 0};
        return_value
    }
    pub fn adjust(&mut self, offset2: i8){
        let offset = offset2 as i32;
        self.height_weight = &self.height_weight + &offset;
        self.weight_weight = &self.weight_weight + &offset;
        self.legs_weight = &self.legs_weight + &offset;
        self.is_alive_weight = &self.is_alive_weight + &offset;
        self.bias = &self.bias + &offset;
    }
}