/*
* File: silent_lpf.rs
* Project: src
* Created Date: 19/09/2021
* Author: Shun Suzuki
* -----
* Last Modified: 19/09/2021
* Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
* -----
* Copyright (c) 2021 Hapis Lab. All rights reserved.
*
*/

use std::collections::VecDeque;

#[derive(Clone)]
pub struct LPF {
    coef: Vec<f32>,
    duty_buf: VecDeque<f32>,
    phase_buf: VecDeque<f32>,
}

impl LPF {
    pub fn new() -> Self {
        let coef = vec![
            -0.0000000, 0.0000011, 0.0000044, 0.0000098, 0.0000175, 0.0000275, 0.0000397,
            0.0000543, 0.0000714, 0.0000908, 0.0001129, 0.0001375, 0.0001649, 0.0001951, 0.0002282,
            0.0002644, 0.0003037, 0.0003462, 0.0003921, 0.0004415, 0.0004945, 0.0005512, 0.0006117,
            0.0006762, 0.0007448, 0.0008176, 0.0008947, 0.0009762, 0.0010623, 0.0011529, 0.0012483,
            0.0013484, 0.0014534, 0.0015633, 0.0016781, 0.0017980, 0.0019228, 0.0020528, 0.0021878,
            0.0023278, 0.0024729, 0.0026229, 0.0027779, 0.0029378, 0.0031025, 0.0032718, 0.0034457,
            0.0036241, 0.0038068, 0.0039936, 0.0041844, 0.0043789, 0.0045771, 0.0047786, 0.0049831,
            0.0051906, 0.0054006, 0.0056129, 0.0058273, 0.0060434, 0.0062608, 0.0064793, 0.0066986,
            0.0069182, 0.0071379, 0.0073572, 0.0075758, 0.0077933, 0.0080093, 0.0082235, 0.0084354,
            0.0086447, 0.0088510, 0.0090538, 0.0092529, 0.0094477, 0.0096380, 0.0098234, 0.0100035,
            0.0101778, 0.0103462, 0.0105082, 0.0106635, 0.0108117, 0.0109526, 0.0110859, 0.0112113,
            0.0113286, 0.0114373, 0.0115375, 0.0116287, 0.0117109, 0.0117839, 0.0118474, 0.0119015,
            0.0119458, 0.0119804, 0.0120052, 0.0120200, 0.0120250, 0.0120200, 0.0120052, 0.0119804,
            0.0119458, 0.0119015, 0.0118474, 0.0117839, 0.0117109, 0.0116287, 0.0115375, 0.0114373,
            0.0113286, 0.0112113, 0.0110859, 0.0109526, 0.0108117, 0.0106635, 0.0105082, 0.0103462,
            0.0101778, 0.0100035, 0.0098234, 0.0096380, 0.0094477, 0.0092529, 0.0090538, 0.0088510,
            0.0086447, 0.0084354, 0.0082235, 0.0080093, 0.0077933, 0.0075758, 0.0073572, 0.0071379,
            0.0069182, 0.0066986, 0.0064793, 0.0062608, 0.0060434, 0.0058273, 0.0056129, 0.0054006,
            0.0051906, 0.0049831, 0.0047786, 0.0045771, 0.0043789, 0.0041844, 0.0039936, 0.0038068,
            0.0036241, 0.0034457, 0.0032718, 0.0031025, 0.0029378, 0.0027779, 0.0026229, 0.0024729,
            0.0023278, 0.0021878, 0.0020528, 0.0019228, 0.0017980, 0.0016781, 0.0015633, 0.0014534,
            0.0013484, 0.0012483, 0.0011529, 0.0010623, 0.0009762, 0.0008947, 0.0008176, 0.0007448,
            0.0006762, 0.0006117, 0.0005512, 0.0004945, 0.0004415, 0.0003921, 0.0003462, 0.0003037,
            0.0002644, 0.0002282, 0.0001951, 0.0001649, 0.0001375, 0.0001129, 0.0000908, 0.0000714,
            0.0000543, 0.0000397, 0.0000275, 0.0000175, 0.0000098, 0.0000044, 0.0000011,
            -0.0000000,
        ];

        let mut buf = VecDeque::new();
        for _ in 0..coef.len() {
            buf.push_back(0.);
        }
        Self {
            coef,
            duty_buf: buf.clone(),
            phase_buf: buf,
        }
    }

    pub fn update(&mut self, duty: u8, phase: u8) -> (u8, u8) {
        self.duty_buf.push_back(duty as f32);
        self.phase_buf.push_back(phase as f32);
        self.duty_buf.pop_front();
        self.phase_buf.pop_front();

        let mut duty = 0.0;
        let mut phase = 0.0;
        for i in 0..self.coef.len() {
            let c = self.coef[i];
            duty += self.duty_buf[i] * c;
            phase += self.phase_buf[i] * c;
        }

        let duty = duty.round() as u16;
        let phase = phase.round() as u16;
        (duty.clamp(0, 0xFF) as _, (phase & 0xFF) as _)
    }
}
