// Copyright (c) 2024 INVAP, open@invap.com.ar
// SPDX-License-Identifier: AGPL-3.0-or-later OR Fundacion-Sadosky-Commercial

mod data_source {
    pub mod ex_adc;
}
mod data_display {
    pub mod ex_display;
}
mod functions;

use crate::data_source::ex_adc::ADC;
use crate::functions::{background, bar, measurement};

fn main() {
    let mut realvalue_old: u16;
    let mut value: u16;
    let mut addition: u16;
    let mut realvalue: u16;
    // INSTRUMENTACION:
    // Process event: task_started,init
    // Timed event: clock_start,init_clk
    let mut adc = ADC::new();
    // INSTRUMENTACION: Component event: adc,adc_init
    background();
    // INSTRUMENTACION: Component event: display,background
    realvalue_old = 0;
    // INSTRUMENTACION:
    // Variable assigned: variable_value_assigned,main_realvalue_old,{ realvalue_old }
    // TimedEvent: clock_pause,init_clk
    // ProcessEvent: task_finished,init
    // TimedEvent: clock_start,filtering_clk
    // TimedEvent: clock_pause,filtering_clk
    loop {
        addition = 0;
        // INSTRUMENTACION:
        // StateEvent: variable_value_assigned,main_addition,{ addition }
        // ProcessEvent: task_started,filtering
        // TimedEvent: clock_reset,filtering_clk
        for _i in 0..16{
            value = adc.sample();
            // INSTRUMENTACION:
            // ComponentEvent: adc,sample,{ value }
            // StateEvent: variable_value_assigned,main_value_{ i },{ value }
            // StateEvent: variable_value_assigned,main_value,{ value }
            // ProcessEvent: checkpoint_reached,filtering_chk
            addition = addition + value;
            // INSTRUMENTACION:
            // StateEvent, variable_value_assigned,main_addition,{ addition }
        }
        realvalue = addition / 16;
        // INSTRUMENTACION:
        // StateEvent: variable_value_assigned,main_realvalue,{ realvalue }
        // TimedEvent: clock_pause,filtering_clk
        // ProcessEvent: task_finished,filtering
        // ProcessEvent: task_started,conversion
        measurement(realvalue);
        bar(realvalue, realvalue_old);
        // INSTRUMENTACION:
        // ProcessEvent: task_finished,conversion
        realvalue_old = realvalue;
        // INSTRUMENTACION:
        // StateEvent: variable_value_assigned,main_realvalue_old,{ realvalue_old }
        // ProcessEvent: checkpoint_reached,display_chk
    } // WHILE
} // MAIN
