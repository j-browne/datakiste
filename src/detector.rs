//TODO: Display for detid, daqid, etc
use crate::DaqId;
use std::fmt::Debug;

pub trait Detector: Debug {
    fn name(&self) -> String;
    fn num_chans(&self) -> u16;
    #[allow(unused_variables)]
    fn val_corr(&self, detch: u16, val: u16) -> u16 {
        val
    }
    fn contains_daq(&self, id: DaqId) -> bool;
    fn daq_to_det(&self, id: DaqId) -> Option<u16>;
    fn det_to_daq(&self, detch: u16) -> Option<DaqId>;
}

#[derive(Debug)]
pub struct BB10F {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl BB10F {
    pub fn new(id: DaqId, n: String) -> BB10F {
        BB10F {
            start: id,
            name: n,
            num_chans: 8,
        }
    }
}

impl Detector for BB10F {
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

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0)
            && (id.1 == self.start.1)
            && (id.2 == self.start.2)
            && ((id.3 >= self.start.3) && (id.3 < self.start.3 + self.num_chans))
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BB15B {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl BB15B {
    pub fn new(id: DaqId, n: String) -> BB15B {
        BB15B {
            start: id,
            name: n,
            num_chans: 4,
        }
    }
}

impl Detector for BB15B {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0)
            && (id.1 == self.start.1)
            && (id.2 == self.start.2)
            && ((id.3 >= self.start.3) && (id.3 < self.start.3 + self.num_chans))
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BB15F {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl BB15F {
    pub fn new(id: DaqId, n: String) -> BB15F {
        BB15F {
            start: id,
            name: n,
            num_chans: 64,
        }
    }
}

impl Detector for BB15F {
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

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0)
            && (id.1 == self.start.1)
            && ((id.2 >= self.start.2) && (id.2 < self.start.2 + 4))
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else if (id.2 - self.start.2) % 2 == 0 {
            Some(16 * (id.2 - self.start.2) + id.3)
        } else {
            Some(16 * (id.2 - self.start.2) + (15 - id.3))
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
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

#[derive(Debug)]
pub struct HABANERO {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl HABANERO {
    pub fn new(id: DaqId, n: String) -> HABANERO {
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

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0)
            && (id.1 == self.start.1)
            && ((id.2 >= self.start.2) && (id.2 < self.start.2 + 5))
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(16 * (id.2 - self.start.2) + id.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
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

#[derive(Debug)]
pub struct HAGRID {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl HAGRID {
    pub fn new(id: DaqId, n: String) -> HAGRID {
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

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0)
            && (id.1 == self.start.1)
            && (id.2 == self.start.2)
            && ((id.3 >= self.start.3) && (id.3 < self.start.3 + self.num_chans))
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct PSICE {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl PSICE {
    pub fn new(id: DaqId, n: String) -> PSICE {
        PSICE {
            start: id,
            name: n,
            num_chans: 1,
        }
    }
}

impl Detector for PSICE {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: DaqId) -> bool {
        id == self.start
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(16 * (id.2 - self.start.2) + id.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
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

#[derive(Debug)]
pub struct PSICXY {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl PSICXY {
    pub fn new(id: DaqId, n: String) -> PSICXY {
        PSICXY {
            start: id,
            name: n,
            num_chans: 32,
        }
    }
}

impl Detector for PSICXY {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0)
            && (id.1 == self.start.1)
            && ((id.2 >= self.start.2) && (id.2 < self.start.2 + 2))
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(16 * (id.2 - self.start.2) + id.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
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

#[derive(Debug)]
pub struct QQQ3B {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl QQQ3B {
    pub fn new(id: DaqId, n: String) -> QQQ3B {
        QQQ3B {
            start: id,
            name: n,
            num_chans: 16,
        }
    }
}

impl Detector for QQQ3B {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2)
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct QQQ3F {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl QQQ3F {
    pub fn new(id: DaqId, n: String) -> QQQ3F {
        QQQ3F {
            start: id,
            name: n,
            num_chans: 16,
        }
    }
}

impl Detector for QQQ3F {
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

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2)
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct QQQ5B {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl QQQ5B {
    pub fn new(id: DaqId, n: String) -> QQQ5B {
        QQQ5B {
            start: id,
            name: n,
            num_chans: 4,
        }
    }
}

impl Detector for QQQ5B {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn num_chans(&self) -> u16 {
        self.num_chans
    }

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0)
            && (id.1 == self.start.1)
            && (id.2 == self.start.2)
            && ((id.3 >= self.start.3) && (id.3 < self.start.3 + self.num_chans))
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct QQQ5F {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl QQQ5F {
    pub fn new(id: DaqId, n: String) -> QQQ5F {
        QQQ5F {
            start: id,
            name: n,
            num_chans: 32,
        }
    }
}

impl Detector for QQQ5F {
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

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0)
            && (id.1 == self.start.1)
            && ((id.2 >= self.start.2) && (id.2 < self.start.2 + 2))
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(16 * (id.2 - self.start.2) + id.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
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

#[derive(Debug)]
pub struct YY1F {
    name: String,
    start: DaqId,
    num_chans: u16,
}

impl YY1F {
    pub fn new(id: DaqId, n: String) -> YY1F {
        YY1F {
            start: id,
            name: n,
            num_chans: 16,
        }
    }
}

impl Detector for YY1F {
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

    fn contains_daq(&self, id: DaqId) -> bool {
        (id.0 == self.start.0) && (id.1 == self.start.1) && (id.2 == self.start.2)
    }

    fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        if !self.contains_daq(id) {
            None
        } else {
            Some(id.3 - self.start.3)
        }
    }

    fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
        if detch < self.num_chans {
            let mut id = self.start;
            id.3 += detch;
            Some(id)
        } else {
            None
        }
    }
}
