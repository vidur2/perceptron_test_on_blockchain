/*
Single Layer Perceptron Test
Smart Contract
*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, require, env};

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};
    use std::convert::TryInto;
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
        let mut contract = InputMatrixWeight {
            height_weight: -10i32,
            weight_weight: -1i32,
            legs_weight: 4i32 * 100,
            is_alive_weight: 1i32 * 1000,
            bias: 0
        };
        let outputs = [1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        let inputs = [(46u32, 150u32, 4u32, 1u8), (50u32, 180u32, 4u32, 1u8), (52u32, 200u32, 4u32, 1u8), (55u32, 250u32, 4u32, 1u8), (46u32, 150u32, 4u32, 1u8), (1000u32, 3500u32, 0u32, 0u8), (400u32, 4000u32, 0u32, 0u8), (500u32, 8000u32, 0u32, 0u8), (300u32, 5000u32, 0u32, 0u8), (600u32, 6000u32, 0u32, 0u8)];
        let mut counter = 0u64;
        loop{
            contract.train(inputs, outputs);
            counter = counter + 1;
            if counter == 1000000u64{break};
        };
        //let prediction = contract.predict(&46u32, &150u32, &4u32, &1u8);
        let prediction2 = contract.predict(&46u32, &150u32, &4u32, &1u8);
        //assert_eq!(1, prediction, "predicted value of {}", prediction);
        assert_eq!(1, prediction2, "predicted2 value of {}", prediction2);
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
    training_datapoints: u32,
}
#[near_bindgen]
impl InputMatrixWeight {
    #[init]
    pub fn new() -> Self{
        require!(!env::state_exists(), "Already initialized");
        Self {
            height_weight: -10i32,
            weight_weight: -1i32,
            legs_weight: 4i32,
            is_alive_weight: 1i32,
            bias: 0
        }
    }
    pub fn predict(&self, height: &u32, weight: &u32, legs: &u32, is_alive: &u8) -> u8{
        let casted_height = *height as i32;
        let casted_weight = *weight as i32;
        let casted_legs = *legs as i32;
        let casted_is_alive =  *is_alive as i32;
        let weighted_sum = &self.bias + casted_height * &self.height_weight + casted_weight * &self.weight_weight + casted_legs * &self.legs_weight + casted_is_alive * &self.is_alive_weight;
        self.step_function(weighted_sum)
    }
    pub fn set_training_length(&mut self, training_datapoints_amt: u32){
        self.training_datapoints = training_datapoints_amt
    }
    pub fn train(&mut self, inputs: [(u32, u32, u32, u8); 10], outputs: [u8; 10]){
        for i in 0..9{
            let prediction = self.predict(&inputs[i].0, &inputs[i].1, &inputs[i].2, &inputs[i].3);
            let offset: i8 = outputs[i] as i8 - prediction as i8;
            self.adjust(offset, inputs[i].0 as i32, inputs[i].1 as i32, inputs[i].2 as i32, inputs[i].3 as i32);
        }
    }
    fn step_function(&self, sum: i32) -> u8{
        let mut return_value: u8 = 0;
        if sum >= 0{return_value = 1}
        else if sum < 0{return_value = 0};
        return_value
    }
    fn adjust(&mut self, offset2: i8, input1: i32, input2: i32, input3: i32, input4: i32){
        let offset = offset2 as i32;
        let mut counter = 0;
        loop {
            self.height_weight = &self.height_weight + &offset * input1;
            self.weight_weight = &self.weight_weight + &offset * input2;
            self.legs_weight = &self.legs_weight + &offset * input3 * 1000000;
            self.is_alive_weight = &self.is_alive_weight + &offset * input4 * 1000000;
            counter = counter + 1;
            if counter == 10 {break};
        }
        self.bias = &self.bias + &offset;
    }
}