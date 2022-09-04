mod utils;

use std::collections::VecDeque;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// kg/m^3
const DENSITY_AIR: f32 = 1.225;
// J per kg per °K
// Assume constant volume of fridge
const C_V_AIR: f32 = 700.0;

#[wasm_bindgen]
pub struct Simulation {
    // Cubic meters 
    capacity: f32,
    history: Vec<f32>,
    history_queue: VecDeque<f32>,
    set_point: f32,
    refrigeration_on: bool,
    // W
    rate_of_heat_gain: f32,
    // W
    rate_of_cooling: f32,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(capacity: f32, initial_temperature: f32) -> Simulation {
        let vol_ratio = capacity/0.220;
        let length_ratio = vol_ratio.powf(1.0/3.0);
        // Estimated using a value of 47.00W for a 220L fridge
        // Assuming the surface area can be estimated using the cube root of the volume ratio
        let rate_of_heat_gain = 47.00*length_ratio;

        // Pulled this value of of nowhere
        let rate_of_cooling = 40.0;

        Simulation{
            capacity,
            history: vec![initial_temperature],
            history_queue: VecDeque::from([initial_temperature]),
            set_point: 2.0,
            refrigeration_on: false,
            rate_of_heat_gain,
            rate_of_cooling: rate_of_cooling,
        }
    }

    pub fn tick(&mut self) {
        let current_temp = *self.history_queue.back().unwrap();

        let mut dE_dot = self.rate_of_heat_gain;

        if self.refrigeration_on {
            if current_temp < (self.set_point - 2.0) {
                self.refrigeration_on = false;
            }
        } else {
            if current_temp > (self.set_point + 2.0) {
                self.refrigeration_on = true;
            }
        }

        if self.refrigeration_on {
            dE_dot -= self.rate_of_cooling;
        }

        let mass_air = self.capacity * DENSITY_AIR;
        let new_temp = current_temp + dE_dot*1.0/(mass_air*C_V_AIR);

        self.history_queue.push_back(new_temp);
        if self.history_queue.len() > 1440 {
            self.history_queue.pop_front();
        }

        self.history_queue.make_contiguous();
        self.history = self.history_queue.as_slices().0.to_vec();
    }

    pub fn rate_of_heat_gain(&self) -> f32 {
        self.rate_of_heat_gain
    }

    pub fn refrigeration_on(&self) -> bool {    
        self.refrigeration_on
    }

    pub fn history(&self) -> *const f32 {
        self.history.as_ptr()
    }

    pub fn len_history(&self) -> i32 {
        self.history.len() as i32
    }

    pub fn current_temp(&self) -> f32 {
        *self.history_queue.back().unwrap()
    }
}