// Copyright (c) 2024 INVAP, open@invap.com.ar
// SPDX-License-Identifier: AGPL-3.0-or-later OR Fundacion-Sadosky-Commercial

use rand::Rng;

pub struct ADC {
    // For when adc data is generated
    previous_sample: u16,
    // Random Number Generator
    rng: rand::rngs::ThreadRng
}

impl ADC {
    // Initialization of the ADC
    pub fn new () -> Self {
        let mut rng = rand::rng();
        let previous_sample = rng.random_range(0..4096);
        Self {
            rng,
            previous_sample,
        }
    }

    // Sampling the ADC
    pub fn sample (&mut self) -> u16 {
        // The generation policy is that only 12-bit data is generated (0 <= sample < 4096)
        // The new sample is only -16 < diff_sample < 16 (4-bit difference magnitude)
        let diff_sample: u16 = self.rng.random_range(0..16);
        let signo: i16 = if self.rng.random_range(0..2) == 0 { 1 } else { -1 };
        let datum: i16 = self.previous_sample as i16 + (signo * diff_sample as i16);

        self.previous_sample = if datum >= 0 && datum <= 4095 {
            datum as u16
        } else if datum > 4095 {
            4095
        } else {
            0
        };

        self.previous_sample
    }
}
