use std::fmt::Debug;

pub trait Detector: Debug {
    fn name(&self) -> String;
    fn num_chans(&self) -> u16;
    #[allow(unused_variables)]
    fn val_corr(&self, detch: u16, val: u16) -> u16 {
        val
    }
    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool;
    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16>;
    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)>;
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct BB10_F {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl BB10_F {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> BB10_F {
        BB10_F {
            start: id,
            name: n,
            num_chans: 8,
        }
    }
}

impl Detector for BB10_F {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    #[allow(unused_variables)]
    fn val_corr(&self, detch: u16, val: u16) -> u16 {
        16383 - val
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2) &&
           ((id.3 >= self.start.3) && (id.3 < self.start.3 + self.num_chans)) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct BB15_B {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl BB15_B {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> BB15_B {
        BB15_B {
            start: id,
            name: n,
            num_chans: 4,
        }
    }
}

impl Detector for BB15_B {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2) &&
           ((id.3 >= self.start.3) && (id.3 < self.start.3 + self.num_chans)) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct BB15_F {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl BB15_F {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> BB15_F {
        BB15_F {
            start: id,
            name: n,
            num_chans: 64,
        }
    }
}

impl Detector for BB15_F {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    #[allow(unused_variables)]
    fn val_corr(&self, detch: u16, val: u16) -> u16 {
        16383 - val
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) &&
           ((id.2 >= self.start.2) && (id.2 < self.start.2 + 4)) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else if (id.2 - self.start.2) % 2 == 0 {
            Some(16 * (id.2 - self.start.2) + id.3)
        } else {
            Some(16 * (id.2 - self.start.2) + (15 - id.3))
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.2 += detch / 16;
            id.3 = detch % 16;
            if (detch / 16) % 2 == 1 {
                id.3 = 15 - id.3;
            }
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct HABANERO {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl HABANERO {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> HABANERO {
        HABANERO {
            start: id,
            name: n,
            num_chans: 60,
        }
    }
}

impl Detector for HABANERO {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) &&
           ((id.2 >= self.start.2) && (id.2 < self.start.2 + 5)) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(16 * (id.2 - self.start.2) + id.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.2 += detch / 16;
            id.3 = detch % 16;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct HAGRID {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl HAGRID {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> HAGRID {
        HAGRID {
            start: id,
            name: n,
            num_chans: 9,
        }
    }
}

impl Detector for HAGRID {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2) &&
           ((id.3 >= self.start.3) && (id.3 < self.start.3 + self.num_chans)) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct PSIC_E {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl PSIC_E {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> PSIC_E {
        PSIC_E {
            start: id,
            name: n,
            num_chans: 1,
        }
    }
}

impl Detector for PSIC_E {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if id == self.start {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(16 * (id.2 - self.start.2) + id.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.2 += detch / 16;
            id.3 += detch % 16;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct PSIC_XY {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl PSIC_XY {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> PSIC_XY {
        PSIC_XY {
            start: id,
            name: n,
            num_chans: 32,
        }
    }
}

impl Detector for PSIC_XY {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) &&
           ((id.2 >= self.start.2) && (id.2 < self.start.2 + 2)) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(16 * (id.2 - self.start.2) + id.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.2 += detch / 16;
            id.3 = detch % 16;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct QQQ3_B {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl QQQ3_B {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> QQQ3_B {
        QQQ3_B {
            start: id,
            name: n,
            num_chans: 16,
        }
    }
}

impl Detector for QQQ3_B {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct QQQ3_F {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl QQQ3_F {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> QQQ3_F {
        QQQ3_F {
            start: id,
            name: n,
            num_chans: 16,
        }
    }
}

impl Detector for QQQ3_F {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    #[allow(unused_variables)]
    fn val_corr(&self, detch: u16, val: u16) -> u16 {
        16383 - val
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct QQQ5_B {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl QQQ5_B {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> QQQ5_B {
        QQQ5_B {
            start: id,
            name: n,
            num_chans: 4,
        }
    }
}

impl Detector for QQQ5_B {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2) &&
           ((id.3 >= self.start.3) && (id.3 < self.start.3 + self.num_chans)) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct QQQ5_F {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl QQQ5_F {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> QQQ5_F {
        QQQ5_F {
            start: id,
            name: n,
            num_chans: 32,
        }
    }
}

impl Detector for QQQ5_F {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    #[allow(unused_variables)]
    fn val_corr(&self, detch: u16, val: u16) -> u16 {
        16383 - val
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) &&
           ((id.2 >= self.start.2) && (id.2 < self.start.2 + 2)) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(16 * (id.2 - self.start.2) + id.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.2 += detch / 16;
            id.3 = detch % 16;
            Some(id)
        } else {
            None
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct YY1_F {
    name: String,
    start: (u16, u16, u16, u16),
    num_chans: u16,
}

impl YY1_F {
    pub fn new(id: (u16, u16, u16, u16), n: String) -> YY1_F {
        YY1_F {
            start: id,
            name: n,
            num_chans: 16,
        }
    }
}

impl Detector for YY1_F {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    #[allow(unused_variables)]
    fn val_corr(&self, detch: u16, val: u16) -> u16 {
        16383 - val
    }

    fn contains_daq(&self, id: (u16, u16, u16, u16)) -> bool {
        if (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2) {
            true
        } else {
            false
        }
    }

    fn daq_to_det(&self, id: (u16, u16, u16, u16)) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<(u16, u16, u16, u16)> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}
