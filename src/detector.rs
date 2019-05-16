use crate::DaqId;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Detector {
    BB10F(u16, u16, u16, u16),
    BB15B(u16, u16, u16, u16),
    BB15F(u16, u16, u16),
    QQQ3B(u16, u16, u16),
    QQQ3F(u16, u16, u16),
    QQQ5B(u16, u16, u16, u16),
    QQQ5F(u16, u16, u16),
    YY1F(u16, u16, u16),
    Hagrid(u16, u16, u16, u16),
    Habanero(u16, u16, u16),
    PSIC_E(u16, u16, u16, u16),
    PSIC_XY(u16, u16, u16),
}

impl Detector {
    pub fn num_chans(&self) -> u16 {
        use Detector::*;
        match self {
            BB10F(..) => 8,
            BB15B(..) => 4,
            BB15F(..) => 64,
            QQQ3B(..) => 16,
            QQQ3F(..) => 16,
            QQQ5B(..) => 4,
            QQQ5F(..) => 32,
            YY1F(..) => 16,
            Hagrid(..) => 9,
            Habanero(..) => 60,
            PSIC_E(..) => 1,
            PSIC_XY(..) => 32,
        }
    }

    pub fn val_corr(&self, _detch: u16, val: u16) -> u16 {
        use Detector::*;
        match self {
            BB10F(..) | BB15F(..) | QQQ3F(..) | QQQ5F(..) | YY1F(..) => 16383 - val,
            BB15B(..) | QQQ3B(..) | QQQ5B(..) | Hagrid(..) | Habanero(..) | PSIC_E(..)
            | PSIC_XY(..) => val,
        }
    }

    pub fn contains_daq(&self, id: DaqId) -> bool {
        use Detector::*;
        match *self {
            BB10F(so, cr, sl, ch)
            | BB15B(so, cr, sl, ch)
            | QQQ5B(so, cr, sl, ch)
            | Hagrid(so, cr, sl, ch) => {
                (id.0 == so)
                    && (id.1 == cr)
                    && (id.2 == sl)
                    && ((id.3 >= ch) && (id.3 < ch + self.num_chans()))
            }
            BB15F(so, cr, sl) => (id.0 == so) && (id.1 == cr) && ((id.2 >= sl) && (id.2 < sl + 4)),
            QQQ3B(so, cr, sl) | QQQ3F(so, cr, sl) | YY1F(so, cr, sl) => {
                (id.0 == so) && (id.1 == cr) && (id.2 == sl)
            }
            QQQ5F(so, cr, sl) => (id.0 == so) && (id.1 == cr) && ((id.2 >= sl) && (id.2 < sl + 2)),
            Habanero(so, cr, sl) => {
                (id.0 == so) && (id.1 == cr) && ((id.2 >= sl) && (id.2 < sl + 5))
            }
            PSIC_E(so, cr, sl, ch) => (id.0 == so) && (id.1 == cr) && (id.2 == sl) && (id.3 == ch),
            PSIC_XY(so, cr, sl) => {
                (id.0 == so) && (id.1 == cr) && ((id.2 >= sl) && (id.2 < sl + 2))
            }
        }
    }

    pub fn daq_to_det(&self, id: DaqId) -> Option<u16> {
        use Detector::*;
        match *self {
            BB10F(_so, _cr, _sl, ch)
            | BB15B(_so, _cr, _sl, ch)
            | QQQ5B(_so, _cr, _sl, ch)
            | Hagrid(_so, _cr, _sl, ch) => {
                if !self.contains_daq(id) {
                    None
                } else {
                    Some(id.3 - ch)
                }
            }
            BB15F(_so, _cr, sl) => {
                if !self.contains_daq(id) {
                    None
                } else if (id.2 - sl) % 2 == 0 {
                    Some(16 * (id.2 - sl) + id.3)
                } else {
                    Some(16 * (id.2 - sl) + (15 - id.3))
                }
            }
            QQQ3B(_so, _cr, _sl) | QQQ3F(_so, _cr, _sl) | YY1F(_so, _cr, _sl) => {
                if !self.contains_daq(id) {
                    None
                } else {
                    Some(id.3)
                }
            }
            QQQ5F(_so, _cr, sl) => {
                if !self.contains_daq(id) {
                    None
                } else {
                    Some(16 * (id.2 - sl) + id.3)
                }
            }
            Habanero(_so, _cr, sl) | PSIC_XY(_so, _cr, sl) => {
                if !self.contains_daq(id) {
                    None
                } else {
                    Some(16 * (id.2 - sl) + id.3)
                }
            }
            PSIC_E(_so, _cr, _sl, _ch) => {
                if !self.contains_daq(id) {
                    None
                } else {
                    Some(0)
                }
            }
        }
    }

    pub fn det_to_daq(&self, detch: u16) -> Option<DaqId> {
        use Detector::*;
        match *self {
            BB10F(so, cr, sl, ch)
            | BB15B(so, cr, sl, ch)
            | QQQ5B(so, cr, sl, ch)
            | Hagrid(so, cr, sl, ch) => {
                if detch < self.num_chans() {
                    Some(DaqId(so, cr, sl, ch + detch))
                } else {
                    None
                }
            }
            BB15F(so, cr, sl) => {
                if detch < self.num_chans() {
                    let mut ch = detch % 16;
                    if (detch / 16) % 2 == 1 {
                        ch = 15 - ch;
                    }
                    Some(DaqId(so, cr, sl + detch / 16, ch))
                } else {
                    None
                }
            }
            QQQ3B(so, cr, sl) | QQQ3F(so, cr, sl) | YY1F(so, cr, sl) => {
                if detch < self.num_chans() {
                    Some(DaqId(so, cr, sl, detch))
                } else {
                    None
                }
            }
            QQQ5F(so, cr, sl) | Habanero(so, cr, sl) | PSIC_XY(so, cr, sl) => {
                if detch < self.num_chans() {
                    Some(DaqId(so, cr, sl + detch / 16, detch % 16))
                } else {
                    None
                }
            }
            PSIC_E(so, cr, sl, ch) => {
                if detch == 0 {
                    Some(DaqId(so, cr, sl, ch))
                } else {
                    None
                }
            }
        }
    }
}
