use std::collections::HashMap;

/// How a parameter value should be displayed
#[derive(Debug, Clone)]
pub enum ParamDisp {
    /// No special display (just the raw number)
    None,
    /// Simple offset (e.g. disp: -10 → value-10)
    Offset(i16),
    /// List of labels (e.g. ["Slow", "Fast"])
    Labels(Vec<String>),
    /// Time-like display with a min/max and a list of textual choices
    Time {
        min: i32,
        max: i32,
        list: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct ParamDef {
    pub name: String,
    pub def: u16,        // default value
    pub max: u16,        // maximum value
    pub disp: ParamDisp, // how to show it
}

impl ParamDef {
    pub fn format_param(&self, value: u16) -> String {
        match &self.disp {
            ParamDisp::None => value.to_string(),
            ParamDisp::Offset(offset) => (value as i16 + offset).to_string(),
            ParamDisp::Labels(labels) => {
                let idx = value as usize;
                if idx < labels.len() {
                    labels[idx].clone()
                } else {
                    value.to_string()
                }
            }
            ParamDisp::Time { min, max, list } => {
                if value >= *min as u16 && value as usize <= *max as usize {
                    let idx = (value - *min as u16) as usize;
                    if idx < list.len() {
                        return list[idx].clone();
                    }
                }
                format!("{}ms", value)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EffectDef {
    pub id: u32,
    pub name: String,
    pub group: String,
    pub title: String, // long description
    pub order: u16,
    pub install: u8,
    pub ver: u16,
    pub dsp: f32,     // base DSP cost
    pub dsp_max: f32, // maximum DSP fraction
    pub dsp_min: f32, // minimum DSP fraction
    pub params: Vec<ParamDef>,
}

pub type EffectDb = HashMap<u32, EffectDef>;

// Helper functions for creating ParamDisp
pub fn labels(list: &[&str]) -> ParamDisp {
    ParamDisp::Labels(list.iter().map(|s| s.to_string()).collect())
}

pub fn offset(v: i16) -> ParamDisp {
    ParamDisp::Offset(v)
}

pub fn time_display(min: i32, max: i32, list: &[&str]) -> ParamDisp {
    ParamDisp::Time {
        min,
        max,
        list: list.iter().map(|s| s.to_string()).collect(),
    }
}

// CAB display lists - these are referenced by many effects
pub const BAMPCAB_DISP: &[&str] = &[
    // This would be the full list from the JS file
    // For now, using placeholder as the actual list isn't provided
];
pub const GAMPCAB_DISP: &[&str] = &[
    // This would be the full list from the JS file
    // For now, using placeholder as the actual list isn't provided
];

pub fn build_effect_db() -> EffectDb {
    let mut db = HashMap::new();

    // THRU
    db.insert(
        0x00000000,
        EffectDef {
            id: 0x00000000,
            name: "THRU".to_string(),
            group: "THRU".to_string(),
            title: "".to_string(),
            order: 1,
            install: 1,
            ver: 0x0101,
            dsp: 10000.0,
            dsp_max: 0.0,
            dsp_min: 0.0,
            params: vec![],
        },
    );

    // D Comp
    db.insert(
        0x00300002,
        EffectDef {
            id: 0x00300002,
            name: "D Comp".to_string(),
            group: "COMP".to_string(),
            title: "MXR Dyna Comp style comp".to_string(),
            order: 2000,
            install: 0,
            ver: 0x0010,
            dsp: 9.7325,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 40.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 3,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 125,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ATTCK".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["Slow", "Fast"]),
                },
            ],
        },
    );

    // Ba Boost
    db.insert(
        0x200018,
        EffectDef {
            id: 0x200018,
            name: "Ba Boost".to_string(),
            group: "DRIVE".to_string(),
            title: "Xotic EP Booster simulation".to_string(),
            order: 2001,
            install: 0,
            ver: 0x0010,
            dsp: 5.8859,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 35,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 10,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 8,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 78,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Bass OD
    db.insert(
        0x400018,
        EffectDef {
            id: 0x400018,
            name: "Bass OD".to_string(),
            group: "DRIVE".to_string(),
            title: "BOSS ODB-3 simulation".to_string(),
            order: 2002,
            install: 0,
            ver: 0x0010,
            dsp: 6.0283,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 0,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 20,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 120,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Bass Muff
    db.insert(
        0x600018,
        EffectDef {
            id: 0x600018,
            name: "Bass Muff".to_string(),
            group: "DRIVE".to_string(),
            title: "Electro-Harmonix Bass Big Muff simulation".to_string(),
            order: 2003,
            install: 0,
            ver: 0x0010,
            dsp: 6.0283,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 88,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 95,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["NORM", "BsBST"]),
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ba Dist 1
    db.insert(
        0x40200018,
        EffectDef {
            id: 0x40200018,
            name: "Ba Dist 1".to_string(),
            group: "DRIVE".to_string(),
            title: "BOSS DS-1 emulation with added paremeter".to_string(),
            order: 2004,
            install: 0,
            ver: 0x0010,
            dsp: 6.4430,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 42,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ba Metal
    db.insert(
        0x00200118,
        EffectDef {
            id: 0x00200118,
            name: "Ba Metal".to_string(),
            group: "DRIVE".to_string(),
            title: "BOSS Metal Zone emulation with added parameter".to_string(),
            order: 2005,
            install: 0,
            ver: 0x0010,
            dsp: 6.3343,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 67,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 85,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 60,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // TS+DRY
    db.insert(
        0x00400118,
        EffectDef {
            id: 0x00400118,
            name: "TS+DRY".to_string(),
            group: "DRIVE".to_string(),
            title: "Ibanez TS808 emulation with added parameter".to_string(),
            order: 2006,
            install: 0,
            ver: 0x0020,
            dsp: 6.0283,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 35,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 74,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 110,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ba Squeak
    db.insert(
        0x00600118,
        EffectDef {
            id: 0x00600118,
            name: "Ba Squeak".to_string(),
            group: "DRIVE".to_string(),
            title: "ProCo RAT emulation with added parameter".to_string(),
            order: 2007,
            install: 0,
            ver: 0x0020,
            dsp: 5.9782,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // BaFzSmile
    db.insert(
        0x40000118,
        EffectDef {
            id: 0x40000118,
            name: "BaFzSmile".to_string(),
            group: "DRIVE".to_string(),
            title: "FUZZ FACE emulation with added parameter".to_string(),
            order: 2008,
            install: 0,
            ver: 0x0020,
            dsp: 6.0283,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 43,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // BassDrive
    db.insert(
        0x0020001a,
        EffectDef {
            id: 0x0020001a,
            name: "BassDrive".to_string(),
            group: "DRIVE".to_string(),
            title: "SansAmp BASS DRIVER DI simulation".to_string(),
            order: 2009,
            install: 0,
            ver: 0x0010,
            dsp: 4.8000,
            dsp_max: 68.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 11,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 11,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 93,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Blend".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 40,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 17,
                    max: 20,
                    disp: offset(-10),
                },
            ],
        },
    );

    // D.I Plus
    db.insert(
        0x0040001a,
        EffectDef {
            id: 0x0040001a,
            name: "D.I Plus".to_string(),
            group: "DRIVE".to_string(),
            title: "MXR Bass D.I.+ simulation".to_string(),
            order: 2010,
            install: 0,
            ver: 0x0010,
            dsp: 5.1429,
            dsp_max: 68.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 12,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 12,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Blend".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 120,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Color".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "CHAN".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["CLN", "DIST"]),
                },
            ],
        },
    );

    // Bass BB
    db.insert(
        0x0060001a,
        EffectDef {
            id: 0x0060001a,
            name: "Bass BB".to_string(),
            group: "DRIVE".to_string(),
            title: "Xotic Bass BB Preamp simulation".to_string(),
            order: 2011,
            install: 0,
            ver: 0x0010,
            dsp: 6.3099,
            dsp_max: 59.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 84,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 16,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Blend".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 70,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // DI5
    db.insert(
        0x4000001a,
        EffectDef {
            id: 0x4000001a,
            name: "DI5".to_string(),
            group: "DRIVE".to_string(),
            title: "AVALON DESIGN U5 preamp simulation".to_string(),
            order: 2012,
            install: 0,
            ver: 0x0010,
            dsp: 4.9091,
            dsp_max: 68.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 0,
                    max: 6,
                    disp: labels(&["OFF", "1", "2", "3", "4", "5", "6"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiCut".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Bass Pre
    db.insert(
        0x4020001a,
        EffectDef {
            id: 0x4020001a,
            name: "Bass Pre".to_string(),
            group: "DRIVE".to_string(),
            title: "Preamp with semi-parametric EQ".to_string(),
            order: 2013,
            install: 0,
            ver: 0x0010,
            dsp: 4.9091,
            dsp_max: 68.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 3,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Treble".to_string(),
                    def: 3,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 80,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 14,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Freq".to_string(),
                    def: 7,
                    max: 23,
                    disp: labels(&[
                        "100Hz", "120Hz", "140Hz", "150Hz", "160Hz", "180Hz", "200Hz", "250Hz",
                        "300Hz", "350Hz", "400Hz", "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz",
                        "1.2kHz", "1.6kHz", "2.0kHz", "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz",
                        "4.5kHz",
                    ]),
                },
            ],
        },
    );

    // AC Bs Pre
    db.insert(
        0x4040001a,
        EffectDef {
            id: 0x4040001a,
            name: "AC Bs Pre".to_string(),
            group: "DRIVE".to_string(),
            title: "Preamp with graphic EQ".to_string(),
            order: 2014,
            install: 0,
            ver: 0x0010,
            dsp: 4.9091,
            dsp_max: 68.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 10,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "L-Mid".to_string(),
                    def: 8,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "LM_F".to_string(),
                    def: 20,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 7,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "H-Mid".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 12,
                    max: 20,
                    disp: offset(-10),
                },
            ],
        },
    );

    // SVT
    db.insert(
        0x0020000a,
        EffectDef {
            id: 0x0020000a,
            name: "SVT".to_string(),
            group: "AMP".to_string(),
            title: "Ampeg SVT simulation".to_string(),
            order: 2100,
            install: 0,
            ver: 0x0010,
            dsp: 3.7895,
            dsp_max: 5.0 / 18.0,
            dsp_min: 201.0 / 750.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 12,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 14,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 12,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 15,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 38,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Ultra".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&["OFF", "Low", "Hi", "Both", "Cut"]),
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 16,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // B-Man
    db.insert(
        0x0040000a,
        EffectDef {
            id: 0x0040000a,
            name: "B-Man".to_string(),
            group: "AMP".to_string(),
            title: "Fender BASSMAN 100 simulation".to_string(),
            order: 2101,
            install: 0,
            ver: 0x0010,
            dsp: 3.7895,
            dsp_max: 5.0 / 18.0,
            dsp_min: 201.0 / 750.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 15,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 16,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 10,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 12,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 60,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Deep".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 32,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // HRT3500
    db.insert(
        0x0060000a,
        EffectDef {
            id: 0x0060000a,
            name: "HRT3500".to_string(),
            group: "AMP".to_string(),
            title: "Hartke HA3500 simulation".to_string(),
            order: 2102,
            install: 0,
            ver: 0x0010,
            dsp: 3.7895,
            dsp_max: 5.0 / 18.0,
            dsp_min: 201.0 / 750.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 12,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 14,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 11,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 12,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 50,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Comp".to_string(),
                    def: 0,
                    max: 10,
                    disp: labels(&["OFF", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]),
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 48,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // acoustic
    db.insert(
        0x4040000a,
        EffectDef {
            id: 0x4040000a,
            name: "acoustic".to_string(),
            group: "AMP".to_string(),
            title: "acoustic 360 simulation".to_string(),
            order: 2103,
            install: 0,
            ver: 0x0010,
            dsp: 3.7895,
            dsp_max: 5.0 / 18.0,
            dsp_min: 201.0 / 750.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 15,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 15,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 17,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 19,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 85,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bright".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 96,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ag Amp
    db.insert(
        0x4060000a,
        EffectDef {
            id: 0x4060000a,
            name: "Ag Amp".to_string(),
            group: "AMP".to_string(),
            title: "Aguilar DB750 simulation".to_string(),
            order: 2104,
            install: 0,
            ver: 0x0010,
            dsp: 3.7895,
            dsp_max: 5.0 / 18.0,
            dsp_min: 201.0 / 750.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 14,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 17,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 13,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 75,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Char".to_string(),
                    def: 0,
                    max: 3,
                    disp: labels(&["OFF", "Deep", "Brght", "Both"]),
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 112,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Mark B
    db.insert(
        0x4000010a,
        EffectDef {
            id: 0x4000010a,
            name: "Mark B".to_string(),
            group: "AMP".to_string(),
            title: "Markbass Little Mark III simulation".to_string(),
            order: 2105,
            install: 0,
            ver: 0x0010,
            dsp: 3.7895,
            dsp_max: 5.0 / 18.0,
            dsp_min: 201.0 / 750.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 16,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 12,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 80,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Color".to_string(),
                    def: 0,
                    max: 6,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 192,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SMR
    db.insert(
        0x4000000a,
        EffectDef {
            id: 0x4000000a,
            name: "SMR".to_string(),
            group: "AMP".to_string(),
            title: "SWR SM-900 simulation".to_string(),
            order: 2106,
            install: 0,
            ver: 0x0020,
            dsp: 3.4923,
            dsp_max: 5.0 / 18.0,
            dsp_min: 746.0 / 2700.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 17,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 9,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 13,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 60,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ENHNC".to_string(),
                    def: 1,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 64,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Flip Top
    db.insert(
        0x4020000a,
        EffectDef {
            id: 0x4020000a,
            name: "Flip Top".to_string(),
            group: "AMP".to_string(),
            title: "Ampeg B-15 simulation".to_string(),
            order: 2107,
            install: 0,
            ver: 0x0020,
            dsp: 3.4923,
            dsp_max: 84.0 / 300.0,
            dsp_min: 746.0 / 2700.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 12,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 18,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 5,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 12,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 90,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Ultra".to_string(),
                    def: 0,
                    max: 3,
                    disp: labels(&["Off", "Low", "Hi", "Both"]),
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 80,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Monotone
    db.insert(
        0x0000010a,
        EffectDef {
            id: 0x0000010a,
            name: "Monotone".to_string(),
            group: "AMP".to_string(),
            title: "POLYTONE MINI-BRUTE III simulation".to_string(),
            order: 2108,
            install: 0,
            ver: 0x0020,
            dsp: 3.4923,
            dsp_max: 84.0 / 300.0,
            dsp_min: 746.0 / 2700.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 18,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 13,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 70,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Char".to_string(),
                    def: 2,
                    max: 2,
                    disp: labels(&["Dark", "Brght", "Flat"]),
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 128,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SuperB
    db.insert(
        0x0020010a,
        EffectDef {
            id: 0x0020010a,
            name: "SuperB".to_string(),
            group: "AMP".to_string(),
            title: "Marshall Super Bass I simulation".to_string(),
            order: 2109,
            install: 0,
            ver: 0x0020,
            dsp: 3.3479,
            dsp_max: 1.0 / 3.0,
            dsp_min: 82.0 / 300.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 15,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 17,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 17,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 13,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 55,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 40,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 144,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // G-Krueger
    db.insert(
        0x0040010a,
        EffectDef {
            id: 0x0040010a,
            name: "G-Krueger".to_string(),
            group: "AMP".to_string(),
            title: "Gallien-Krueger 800RB simulation".to_string(),
            order: 2110,
            install: 0,
            ver: 0x0020,
            dsp: 3.4923,
            dsp_max: 84.0 / 300.0,
            dsp_min: 746.0 / 2700.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 18,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 14,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 12,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 12,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 80,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Color".to_string(),
                    def: 0,
                    max: 3,
                    disp: labels(&["Off", "Low", "Mid", "Hi"]),
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 160,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Heaven
    db.insert(
        0x0060010a,
        EffectDef {
            id: 0x0060010a,
            name: "Heaven".to_string(),
            group: "AMP".to_string(),
            title: "Eden WT-800 simulation".to_string(),
            order: 2111,
            install: 0,
            ver: 0x0020,
            dsp: 3.4923,
            dsp_max: 84.0 / 300.0,
            dsp_min: 746.0 / 2700.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 15,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 13,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 14,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mid_F".to_string(),
                    def: 12,
                    max: 30,
                    disp: labels(&[
                        "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
                        "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz",
                        "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz", "6.3kHz",
                    ]),
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 80,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ENHNC".to_string(),
                    def: 1,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 176,
                    max: 255,
                    disp: labels(BAMPCAB_DISP),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ba Synth
    db.insert(
        0x0070000e,
        EffectDef {
            id: 0x0070000e,
            name: "Ba Synth".to_string(),
            group: "SFX".to_string(),
            title: "Monophonic bass synth sound".to_string(),
            order: 2200,
            install: 0,
            ver: 0x0010,
            dsp: 4.8000,
            dsp_max: 1.0 / 4.0,
            dsp_min: 22.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 36,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Wave".to_string(),
                    def: 2,
                    max: 2,
                    disp: labels(&["Saw", "Pulse", "PWM"]),
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 9,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Synth".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 135,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // StdSyn
    db.insert(
        0x4060000e,
        EffectDef {
            id: 0x4060000e,
            name: "StdSyn".to_string(),
            group: "SFX".to_string(),
            title: "ZOOM original bass synth sound".to_string(),
            order: 2201,
            install: 0,
            ver: 0x0010,
            dsp: 4.8000,
            dsp_max: 1.0 / 4.0,
            dsp_min: 22.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 10,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Sound".to_string(),
                    def: 0,
                    max: 3,
                    disp: ParamDisp::None, // JS had disp:1 which seems like a placeholder
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Synth".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SynTlk
    db.insert(
        0x0000010e,
        EffectDef {
            id: 0x0000010e,
            name: "SynTlk".to_string(),
            group: "SFX".to_string(),
            title: "Talking modulator like sound".to_string(),
            order: 2202,
            install: 0,
            ver: 0x0010,
            dsp: 4.8000,
            dsp_max: 1.0 / 4.0,
            dsp_min: 22.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Type".to_string(),
                    def: 1,
                    max: 3,
                    disp: labels(&["iA", "UE", "UA", "oA"]),
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Synth".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z-Syn
    db.insert(
        0x0020010e,
        EffectDef {
            id: 0x0020010e,
            name: "Z-Syn".to_string(),
            group: "SFX".to_string(),
            title: "analog bass synth sound".to_string(),
            order: 2203,
            install: 0,
            ver: 0x0010,
            dsp: 4.8000,
            dsp_max: 68.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Wave".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Saw", "Sqr"]),
                },
                ParamDef {
                    name: "Decay".to_string(),
                    def: 72,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Freq".to_string(),
                    def: 2,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Range".to_string(),
                    def: 8,
                    max: 20,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 18,
                    max: 20,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Synth".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 0,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Defret
    db.insert(
        0x0040010e,
        EffectDef {
            id: 0x0040010e,
            name: "Defret".to_string(),
            group: "SFX".to_string(),
            title: "Fretless bass sound".to_string(),
            order: 2204,
            install: 0,
            ver: 0x0010,
            dsp: 10.8126,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 40.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 11,
                    max: 30,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Color".to_string(),
                    def: 7,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 140,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 36,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
            ],
        },
    );

    // V-Syn
    db.insert(
        0x4000010e,
        EffectDef {
            id: 0x4000010e,
            name: "V-Syn".to_string(),
            group: "SFX".to_string(),
            title: "Vintage bass synth sound".to_string(),
            order: 2205,
            install: 0,
            ver: 0x0020,
            dsp: 4.8000,
            dsp_max: 1.0 / 4.0,
            dsp_min: 22.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 24,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Sense".to_string(),
                    def: 11,
                    max: 30,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Range".to_string(),
                    def: 17,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Synth".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // 4VoiceSyn
    db.insert(
        0x4020010e,
        EffectDef {
            id: 0x4020010e,
            name: "4VoiceSyn".to_string(),
            group: "SFX".to_string(),
            title: "Add synth harmony effect".to_string(),
            order: 2205,
            install: 0,
            ver: 0x0020,
            dsp: 6.6977,
            dsp_max: 68.0 / 300.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "ATTCK".to_string(),
                    def: 0,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 3,
                    max: 8,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Scale".to_string(),
                    def: 0,
                    max: 1,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Synth".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // MS-70CDR Section - Ba Chorus
    db.insert(
        0x4070000c,
        EffectDef {
            id: 0x4070000c,
            name: "Ba Chorus".to_string(),
            group: "MOD".to_string(),
            title: "Chorus effect for bass".to_string(),
            order: 1000,
            install: 0,
            ver: 0x0110,
            dsp: 10.5744,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 42,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 22,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 57,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LoCut".to_string(),
                    def: 2,
                    max: 10,
                    disp: labels(&[
                        "OFF", "60Hz", "120Hz", "180Hz", "200Hz", "280Hz", "340Hz", "400Hz",
                        "500Hz", "630Hz", "800Hz",
                    ]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Ba Detune
    db.insert(
        0x0010010c,
        EffectDef {
            id: 0x0010010c,
            name: "Ba Detune".to_string(),
            group: "MOD".to_string(),
            title: "Mix a small amount of the pitch-shift".to_string(),
            order: 1001,
            install: 0,
            ver: 0x0110,
            dsp: 6.9314,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Cent".to_string(),
                    def: 35,
                    max: 50,
                    disp: offset(-50),
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 0,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LoCut".to_string(),
                    def: 1,
                    max: 10,
                    disp: labels(&[
                        "OFF", "60Hz", "120Hz", "180Hz", "200Hz", "280Hz", "340Hz", "400Hz",
                        "500Hz", "630Hz", "800Hz",
                    ]),
                },
            ],
        },
    );

    // Ba Ensmbl
    db.insert(
        0x0070010c,
        EffectDef {
            id: 0x0070010c,
            name: "Ba Ensmbl".to_string(),
            group: "MOD".to_string(),
            title: "Bass chorus with 3D movement".to_string(),
            order: 1002,
            install: 0,
            ver: 0x0110,
            dsp: 7.5711,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 22,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 5,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // BaFlanger
    db.insert(
        0x4050010c,
        EffectDef {
            id: 0x4050010c,
            name: "BaFlanger".to_string(),
            group: "MOD".to_string(),
            title: "ADA Flanger modeling".to_string(),
            order: 1003,
            install: 0,
            ver: 0x0110,
            dsp: 7.1489,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 76,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 40,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 15,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 2,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 95,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LoCut".to_string(),
                    def: 1,
                    max: 10,
                    disp: labels(&[
                        "OFF", "60Hz", "120Hz", "180Hz", "200Hz", "280Hz", "340Hz", "400Hz",
                        "500Hz", "630Hz", "800Hz",
                    ]),
                },
            ],
        },
    );

    // Ba Octave
    db.insert(
        0x0030020c,
        EffectDef {
            id: 0x0030020c,
            name: "Ba Octave".to_string(),
            group: "MOD".to_string(),
            title: "Adds sound one octave below".to_string(),
            order: 1004,
            install: 0,
            ver: 0x0210,
            dsp: 11.4286,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Oct".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Low".to_string(),
                    def: 3,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mid".to_string(),
                    def: 4,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ba Pitch
    db.insert(
        0x0070020c,
        EffectDef {
            id: 0x0070020c,
            name: "Ba Pitch".to_string(),
            group: "MOD".to_string(),
            title: "Pitch shifter for bass".to_string(),
            order: 1004,
            install: 0,
            ver: 0x0110,
            dsp: 5.4545,
            dsp_max: 1.0 / 5.0,
            dsp_min: 1.0 / 6.0,
            params: vec![
                ParamDef {
                    name: "Shift".to_string(),
                    def: 0,
                    max: 25,
                    disp: labels(&[
                        "-12", "-11", "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1",
                        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "24",
                    ]),
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Fine".to_string(),
                    def: 25,
                    max: 50,
                    disp: offset(-25),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 140,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // ModDelay2
    db.insert(
        0x00500010,
        EffectDef {
            id: 0x00500010,
            name: "ModDelay2".to_string(),
            group: "DELAY".to_string(),
            title: "Modulation delay with depth adjust".to_string(),
            order: 1005,
            install: 0,
            ver: 0x0110,
            dsp: 7.5506,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 529,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 16,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // St Bs GEQ
    db.insert(
        0x00000204,
        EffectDef {
            id: 0x00000204,
            name: "St Bs GEQ".to_string(),
            group: "FILTER".to_string(),
            title: "7 bands stereo GEQ for bass".to_string(),
            order: 1006,
            install: 0,
            ver: 0x0120,
            dsp: 5.4545,
            dsp_max: 23.0 / 125.0,
            dsp_min: 18.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "50Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "120Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "400Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "500Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "800Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "4.5kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "10kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // 160 Comp
    db.insert(
        0x006a0002,
        EffectDef {
            id: 0x006a0002,
            name: "160 Comp".to_string(),
            group: "COMP".to_string(),
            title: "dbx 160A style comp".to_string(),
            order: 1008,
            install: 0,
            ver: 0x0210,
            dsp: 7.2000,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 38,
                    max: 60,
                    disp: offset(-60),
                },
                ParamDef {
                    name: "Ratio".to_string(),
                    def: 30,
                    max: 90,
                    disp: ParamDisp::None, // JS had disp:1, dispr:0.1
                },
                ParamDef {
                    name: "Gain".to_string(),
                    def: 6,
                    max: 20,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Knee".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Hard", "Soft"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Limiter
    db.insert(
        0x00740002,
        EffectDef {
            id: 0x00740002,
            name: "Limiter".to_string(),
            group: "COMP".to_string(),
            title: "Limiter that suppresses signal peaks".to_string(),
            order: 1010,
            install: 0,
            ver: 0x0210,
            dsp: 9.7509,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 20,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Ratio".to_string(),
                    def: 3,
                    max: 9,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 90,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "REL".to_string(),
                    def: 2,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
            ],
        },
    );

    // DualComp
    db.insert(
        0x00400102,
        EffectDef {
            id: 0x00400102,
            name: "DualComp".to_string(),
            group: "COMP".to_string(),
            title: "Compressor with low/high separate frequency".to_string(),
            order: 1012,
            install: 0,
            ver: 0x0220,
            dsp: 7.5358,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Hi".to_string(),
                    def: 24,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Lo".to_string(),
                    def: 15,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Freq".to_string(),
                    def: 9,
                    max: 9,
                    disp: labels(&[
                        "300Hz", "400Hz", "500Hz", "600Hz", "700Hz", "800Hz", "900Hz", "1.0kHz",
                        "1.2kHz", "1.5kHz",
                    ]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 2,
                    max: 10,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ba GEQ
    db.insert(
        0x00300004,
        EffectDef {
            id: 0x00300004,
            name: "Ba GEQ".to_string(),
            group: "FILTER".to_string(),
            title: "7 bands GEQ for bass".to_string(),
            order: 1020,
            install: 0,
            ver: 0x0210,
            dsp: 10.4591,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "50Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "120Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "400Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "500Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "800Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "4.5kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "10kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ba PEQ
    db.insert(
        0x00480004,
        EffectDef {
            id: 0x00480004,
            name: "Ba PEQ".to_string(),
            group: "FILTER".to_string(),
            title: "2-band parametric equalizer for bass".to_string(),
            order: 1100,
            install: 0,
            ver: 0x0210,
            dsp: 11.2783,
            dsp_max: 1.0 / 9.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Freq1".to_string(),
                    def: 8,
                    max: 37,
                    disp: labels(&[
                        "20Hz", "25Hz", "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz",
                        "120Hz", "140Hz", "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz",
                        "350Hz", "400Hz", "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz",
                        "1.6kHz", "2.0kHz", "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz",
                        "6.3kHz", "8.0kHz", "10kHz", "12kHz", "16kHz", "20kHz",
                    ]),
                },
                ParamDef {
                    name: "Q1".to_string(),
                    def: 1,
                    max: 5,
                    disp: labels(&["0.5", "1", "2", "4", "8", "16"]),
                },
                ParamDef {
                    name: "Gain1".to_string(),
                    def: 20,
                    max: 40,
                    disp: offset(-20),
                },
                ParamDef {
                    name: "Freq2".to_string(),
                    def: 15,
                    max: 37,
                    disp: labels(&[
                        "20Hz", "25Hz", "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz",
                        "120Hz", "140Hz", "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz",
                        "350Hz", "400Hz", "450Hz", "500Hz", "630Hz", "800Hz", "1.0kHz", "1.2kHz",
                        "1.6kHz", "2.0kHz", "2.5kHz", "3.0kHz", "3.6kHz", "4.0kHz", "4.5kHz",
                        "6.3kHz", "8.0kHz", "10kHz", "12kHz", "16kHz", "20kHz",
                    ]),
                },
                ParamDef {
                    name: "Q2".to_string(),
                    def: 1,
                    max: 5,
                    disp: labels(&["0.5", "1", "2", "4", "8", "16"]),
                },
                ParamDef {
                    name: "Gain2".to_string(),
                    def: 20,
                    max: 40,
                    disp: offset(-20),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Splitter
    db.insert(
        0x00500004,
        EffectDef {
            id: 0x00500004,
            name: "Splitter".to_string(),
            group: "FILTER".to_string(),
            title: "Divide into 2bands and mix with ratio".to_string(),
            order: 1101,
            install: 0,
            ver: 0x0210,
            dsp: 15.7377,
            dsp_max: 1.0 / 9.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Hi".to_string(),
                    def: 19,
                    max: 50,
                    disp: ParamDisp::None, // JS had disp:0, dispr:2
                },
                ParamDef {
                    name: "Lo".to_string(),
                    def: 16,
                    max: 50,
                    disp: ParamDisp::None, // JS had disp:0, dispr:2
                },
                ParamDef {
                    name: "Freq".to_string(),
                    def: 2,
                    max: 15,
                    disp: labels(&[
                        "80Hz", "100Hz", "125Hz", "160Hz", "200hz", "250Hz", "315Hz", "400Hz",
                        "500Hz", "630Hz", "800Hz", "1.0kHz", "1.3kHz", "1.6kHz", "2.0kHz",
                        "2.5kHz",
                    ]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 95,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Bottom B
    db.insert(
        0x00580004,
        EffectDef {
            id: 0x00580004,
            name: "Bottom B".to_string(),
            group: "FILTER".to_string(),
            title: "Emphasizes low/high frequencies".to_string(),
            order: 1102,
            install: 0,
            ver: 0x0210,
            dsp: 10.7592,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 60,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // BaAutoWah
    db.insert(
        0x40300004,
        EffectDef {
            id: 0x40300004,
            name: "BaAutoWah".to_string(),
            group: "FILTER".to_string(),
            title: "Auto wah for bass".to_string(),
            order: 1103,
            install: 0,
            ver: 0x0210,
            dsp: 11.4685,
            dsp_max: 1.0 / 9.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 11,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 0,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z Tron
    db.insert(
        0x00100104,
        EffectDef {
            id: 0x00100104,
            name: "Z Tron".to_string(),
            group: "FILTER".to_string(),
            title: "Envelope Filter like Q-Tron in LP mode".to_string(),
            order: 1104,
            install: 0,
            ver: 0x0210,
            dsp: 14.7735,
            dsp_max: 1.0 / 9.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 12,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 25,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 108,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // A-Filter
    db.insert(
        0x002a0104,
        EffectDef {
            id: 0x002a0104,
            name: "A-Filter".to_string(),
            group: "FILTER".to_string(),
            title: "Resonance filter with a sharp envelope".to_string(),
            order: 1105,
            install: 0,
            ver: 0x0210,
            dsp: 14.4594,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 7,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Peak".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Up", "Down"]),
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 10,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ba Cry
    db.insert(
        0x00340104,
        EffectDef {
            id: 0x00340104,
            name: "Ba Cry".to_string(),
            group: "FILTER".to_string(),
            title: "Bass frequency talking modulator".to_string(),
            order: 1106,
            install: 0,
            ver: 0x0210,
            dsp: 9.0000,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Range".to_string(),
                    def: 4,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Sense".to_string(),
                    def: 16,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // MS-50G Section - Comp
    db.insert(
        0x00100002,
        EffectDef {
            id: 0x00100002,
            name: "Comp".to_string(),
            group: "COMP".to_string(),
            title: "MXR DynaComp style comp".to_string(),
            order: 100,
            install: 0,
            ver: 0x0201,
            dsp: 14.2979,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ATTCK".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Slow", "Fast"]),
                },
            ],
        },
    );

    // RackComp
    db.insert(
        0x00200002,
        EffectDef {
            id: 0x00200002,
            name: "RackComp".to_string(),
            group: "COMP".to_string(),
            title: "Comp with more detailed parameter".to_string(),
            order: 101,
            install: 0,
            ver: 0x0221,
            dsp: 11.9657,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 40,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Ratio".to_string(),
                    def: 5,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ATTCK".to_string(),
                    def: 6,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
            ],
        },
    );

    // M Comp
    db.insert(
        0x00400002,
        EffectDef {
            id: 0x00400002,
            name: "M Comp".to_string(),
            group: "COMP".to_string(),
            title: "More natural sound comp".to_string(),
            order: 102,
            install: 0,
            ver: 0x0212,
            dsp: 10.0229,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 40,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Ratio".to_string(),
                    def: 3,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ATTCK".to_string(),
                    def: 0,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
            ],
        },
    );

    // OptComp
    db.insert(
        0x00600002,
        EffectDef {
            id: 0x00600002,
            name: "OptComp".to_string(),
            group: "COMP".to_string(),
            title: "APHex Punch FACTORY style comp".to_string(),
            order: 103,
            install: 0,
            ver: 0x0212,
            dsp: 7.5045,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Drive".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 54,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 50,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SlowATTCK
    db.insert(
        0x40000002,
        EffectDef {
            id: 0x40000002,
            name: "SlowATTCK".to_string(),
            group: "COMP".to_string(),
            title: "Violin like slow attack sounds".to_string(),
            order: 104,
            install: 0,
            ver: 0x0211,
            dsp: 12.0646,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 20,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Curve".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // ZNR
    db.insert(
        0x40200002,
        EffectDef {
            id: 0x40200002,
            name: "ZNR".to_string(),
            group: "COMP".to_string(),
            title: "ZOOM's unique noise reduction".to_string(),
            order: 105,
            install: 0,
            ver: 0x0111,
            dsp: 17.4545,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 30.0,
            params: vec![
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 9,
                    max: 24,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "DETCT".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["GtrIn", "EfxIn"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // NoiseGate
    db.insert(
        0x40400002,
        EffectDef {
            id: 0x40400002,
            name: "NoiseGate".to_string(),
            group: "COMP".to_string(),
            title: "Cuts the sound during playing pauses".to_string(),
            order: 106,
            install: 0,
            ver: 0x0222,
            dsp: 14.1548,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 30.0,
            params: vec![
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 9,
                    max: 24,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // DirtyGate
    db.insert(
        0x40600002,
        EffectDef {
            id: 0x40600002,
            name: "DirtyGate".to_string(),
            group: "COMP".to_string(),
            title: "Gate with vintage style way of closing".to_string(),
            order: 107,
            install: 0,
            ver: 0x0223,
            dsp: 16.0624,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 9,
                    max: 24,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // OrangeLim
    db.insert(
        0x00000102,
        EffectDef {
            id: 0x00000102,
            name: "OrangeLim".to_string(),
            group: "COMP".to_string(),
            title: "ORANGE SQUEEZER modeling".to_string(),
            order: 108,
            install: 0,
            ver: 0x0223,
            dsp: 5.4545,
            dsp_max: 23.0 / 125.0,
            dsp_min: 823.0 / 4500.0,
            params: vec![],
        },
    );

    // GrayComp
    db.insert(
        0x00200102,
        EffectDef {
            id: 0x00200102,
            name: "GrayComp".to_string(),
            group: "COMP".to_string(),
            title: "ROSS Compressor modiling".to_string(),
            order: 109,
            install: 0,
            ver: 0x0223,
            dsp: 4.8276,
            dsp_max: 68.0 / 300.0,
            dsp_min: 1.0 / 5.0,
            params: vec![
                ParamDef {
                    name: "SUSTN".to_string(),
                    def: 63,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 88,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // LineSel
    db.insert(
        0x00100004,
        EffectDef {
            id: 0x00100004,
            name: "LineSel".to_string(),
            group: "FILTER".to_string(),
            title: "Send directly to OUTPUT when OFF".to_string(),
            order: 200,
            install: 0,
            ver: 0x0111,
            dsp: 17.6121,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 30.0,
            params: vec![
                ParamDef {
                    name: "EFX_L".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "OUT_L".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // GraphicEQ
    db.insert(
        0x00200004,
        EffectDef {
            id: 0x00200004,
            name: "GraphicEQ".to_string(),
            group: "FILTER".to_string(),
            title: "6-band quealizer".to_string(),
            order: 201,
            install: 0,
            ver: 0x0201,
            dsp: 11.6667,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "160Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "400Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "800Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "3.2kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "6.4kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "12kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // ParaEQ
    db.insert(
        0x00400004,
        EffectDef {
            id: 0x00400004,
            name: "ParaEQ".to_string(),
            group: "FILTER".to_string(),
            title: "2-band parametric equalizer".to_string(),
            order: 202,
            install: 0,
            ver: 0x0201,
            dsp: 16.0000,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 30.0,
            params: vec![
                ParamDef {
                    name: "Freq1".to_string(),
                    def: 8,
                    max: 30,
                    disp: labels(&[
                        "20Hz", "25Hz", "32Hz", "40Hz", "50Hz", "63Hz", "80Hz", "100Hz", "125Hz",
                        "160Hz", "200Hz", "250Hz", "320Hz", "400Hz", "500Hz", "630Hz", "800Hz",
                        "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz", "2.5kHz", "3.2kHz", "4.0kHz",
                        "5.0kHz", "6.3kHz", "8.0kHz", "10kHz", "12kHz", "16kHz", "20kHz",
                    ]),
                },
                ParamDef {
                    name: "Q1".to_string(),
                    def: 1,
                    max: 5,
                    disp: labels(&["0.5", "1", "2", "4", "8", "16"]),
                },
                ParamDef {
                    name: "Gain1".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "Freq2".to_string(),
                    def: 15,
                    max: 30,
                    disp: labels(&[
                        "20Hz", "25Hz", "32Hz", "40Hz", "50Hz", "63Hz", "80Hz", "100Hz", "125Hz",
                        "160Hz", "200Hz", "250Hz", "320Hz", "400Hz", "500Hz", "630Hz", "800Hz",
                        "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz", "2.5kHz", "3.2kHz", "4.0kHz",
                        "5.0kHz", "6.3kHz", "8.0kHz", "10kHz", "12kHz", "16kHz", "20kHz",
                    ]),
                },
                ParamDef {
                    name: "Q2".to_string(),
                    def: 1,
                    max: 5,
                    disp: labels(&["0.5", "1", "2", "4", "8", "16"]),
                },
                ParamDef {
                    name: "Gain2".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Exciter
    db.insert(
        0x00600004,
        EffectDef {
            id: 0x00600004,
            name: "Exciter".to_string(),
            group: "FILTER".to_string(),
            title: "2-band phase exciter".to_string(),
            order: 203,
            install: 0,
            ver: 0x0212,
            dsp: 11.8597,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Bass".to_string(),
                    def: 0,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 0,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // CombFLTR
    db.insert(
        0x40000004,
        EffectDef {
            id: 0x40000004,
            name: "CombFLTR".to_string(),
            group: "FILTER".to_string(),
            title: "Comb filter, that like fix modulated flanger".to_string(),
            order: 204,
            install: 0,
            ver: 0x0202,
            dsp: 12.6000,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Freq".to_string(),
                    def: 24,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 15,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // AutoWah
    db.insert(
        0x40200004,
        EffectDef {
            id: 0x40200004,
            name: "AutoWah".to_string(),
            group: "FILTER".to_string(),
            title: "Wah accordance with picking intensity".to_string(),
            order: 205,
            install: 0,
            ver: 0x0201,
            dsp: 10.1045,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 17,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Resonance
    db.insert(
        0x40400004,
        EffectDef {
            id: 0x40400004,
            name: "Resonance".to_string(),
            group: "FILTER".to_string(),
            title: "Resonance filter according to picking intensisty".to_string(),
            order: 206,
            install: 0,
            ver: 0x0202,
            dsp: 9.9802,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Sense".to_string(),
                    def: 14,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Cry
    db.insert(
        0x40600004,
        EffectDef {
            id: 0x40600004,
            name: "Cry".to_string(),
            group: "FILTER".to_string(),
            title: "Like the talking modulator".to_string(),
            order: 207,
            install: 0,
            ver: 0x0201,
            dsp: 11.9290,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Range".to_string(),
                    def: 6,
                    max: 9,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Sense".to_string(),
                    def: 16,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SlowFLTR
    db.insert(
        0x00000104,
        EffectDef {
            id: 0x00000104,
            name: "SlowFLTR".to_string(),
            group: "FILTER".to_string(),
            title: "Filter changing by picking trigger".to_string(),
            order: 208,
            install: 0,
            ver: 0x0203,
            dsp: 7.2000,
            dsp_max: 1.0 / 6.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 20,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Curve".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Chara".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["2Pole", "4Pole"]),
                },
                ParamDef {
                    name: "DRCTN".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Open", "Close"]),
                },
            ],
        },
    );

    // M-Filter
    db.insert(
        0x00200104,
        EffectDef {
            id: 0x00200104,
            name: "M-Filter".to_string(),
            group: "FILTER".to_string(),
            title: "Evelope filter with Moog MF-101 like LPF".to_string(),
            order: 209,
            install: 0,
            ver: 0x0211,
            dsp: 8.0000,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Freq".to_string(),
                    def: 56,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Sense".to_string(),
                    def: 5,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Type".to_string(),
                    def: 2,
                    max: 2,
                    disp: labels(&["HPF", "BPF", "LPF"]),
                },
                ParamDef {
                    name: "Chara".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["2Pole", "4Pole"]),
                },
                ParamDef {
                    name: "VLCTY".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Fast", "Slow"]),
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Step
    db.insert(
        0x00400104,
        EffectDef {
            id: 0x00400104,
            name: "Step".to_string(),
            group: "FILTER".to_string(),
            title: "Special effect for sound stepping".to_string(),
            order: 210,
            install: 0,
            ver: 0x0201,
            dsp: 11.2783,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 25,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Shape".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SeqFLTR
    db.insert(
        0x00600104,
        EffectDef {
            id: 0x00600104,
            name: "SeqFLTR".to_string(),
            group: "FILTER".to_string(),
            title: "Z.Vex Seek-Wah like sequence filter".to_string(),
            order: 211,
            install: 0,
            ver: 0x0211,
            dsp: 9.6000,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Step".to_string(),
                    def: 6,
                    max: 6,
                    disp: ParamDisp::None, // JS had disp:2
                },
                ParamDef {
                    name: "PTTRN".to_string(),
                    def: 6,
                    max: 7,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Speed".to_string(),
                    def: 25,
                    max: 77,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Shape".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // RndmFLTR
    db.insert(
        0x40000104,
        EffectDef {
            id: 0x40000104,
            name: "RndmFLTR".to_string(),
            group: "FILTER".to_string(),
            title: "Randomly changing filter".to_string(),
            order: 212,
            install: 0,
            ver: 0x0222,
            dsp: 9.6000,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Speed".to_string(),
                    def: 34,
                    max: 77,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Range".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Type".to_string(),
                    def: 2,
                    max: 2,
                    disp: labels(&["HPF", "BPF", "LPF"]),
                },
                ParamDef {
                    name: "Chara".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["2Pole", "4Pole"]),
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 90,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // fCycle
    db.insert(
        0x40200104,
        EffectDef {
            id: 0x40200104,
            name: "fCycle".to_string(),
            group: "FILTER".to_string(),
            title: "Cyclic changing filter".to_string(),
            order: 213,
            install: 0,
            ver: 0x0222,
            dsp: 10.1045,
            dsp_max: 1.0 / 10.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Rate".to_string(),
                    def: 5,
                    max: 77,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Wave".to_string(),
                    def: 3,
                    max: 3,
                    disp: labels(&["Sine", "Tri", "SawUp", "SawDn"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // St Gt GEQ
    db.insert(
        0x40400104,
        EffectDef {
            id: 0x40400104,
            name: "St Gt GEQ".to_string(),
            group: "FILTER".to_string(),
            title: "6-bands Stereo Graphic equalizer for guitar".to_string(),
            order: 214,
            install: 0,
            ver: 0x0103,
            dsp: 7.2000,
            dsp_max: 1.0 / 6.0,
            dsp_min: 33.0 / 250.0,
            params: vec![
                ParamDef {
                    name: "160Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "400Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "800Hz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "3.2kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "6.4kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "12kHz".to_string(),
                    def: 12,
                    max: 24,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // DRIVE Section - Booster
    db.insert(
        0x00100006,
        EffectDef {
            id: 0x00100006,
            name: "Booster".to_string(),
            group: "DRIVE".to_string(),
            title: "Boost signal gain for more power".to_string(),
            order: 300,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // OverDrive
    db.insert(
        0x00200006,
        EffectDef {
            id: 0x00200006,
            name: "OverDrive".to_string(),
            group: "DRIVE".to_string(),
            title: "BOSS OD-1 Overdrive modiling".to_string(),
            order: 301,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // T Scream
    db.insert(
        0x00400006,
        EffectDef {
            id: 0x00400006,
            name: "T Scream".to_string(),
            group: "DRIVE".to_string(),
            title: "Ibanez TS808 modeling".to_string(),
            order: 302,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Governor
    db.insert(
        0x00600006,
        EffectDef {
            id: 0x00600006,
            name: "Governor".to_string(),
            group: "DRIVE".to_string(),
            title: "Marshall Guv'nor distortion modeling".to_string(),
            order: 303,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Dist+
    db.insert(
        0x40000006,
        EffectDef {
            id: 0x40000006,
            name: "Dist+".to_string(),
            group: "DRIVE".to_string(),
            title: "MXR distortion+ modeling".to_string(),
            order: 304,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Dist 1
    db.insert(
        0x40200006,
        EffectDef {
            id: 0x40200006,
            name: "Dist 1".to_string(),
            group: "DRIVE".to_string(),
            title: "BOSS DS-1 distortion modeling".to_string(),
            order: 305,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Squeak
    db.insert(
        0x40400006,
        EffectDef {
            id: 0x40400006,
            name: "Squeak".to_string(),
            group: "DRIVE".to_string(),
            title: "Pro Co Rat distortion modeling".to_string(),
            order: 306,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // FuzzSmile
    db.insert(
        0x40600006,
        EffectDef {
            id: 0x40600006,
            name: "FuzzSmile".to_string(),
            group: "DRIVE".to_string(),
            title: "Fuzz Face modeling".to_string(),
            order: 307,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // GreatMuff
    db.insert(
        0x00000106,
        EffectDef {
            id: 0x00000106,
            name: "GreatMuff".to_string(),
            group: "DRIVE".to_string(),
            title: "Electro-Harmonix Big Muff modeling".to_string(),
            order: 308,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // MetalWRLD
    db.insert(
        0x00200106,
        EffectDef {
            id: 0x00200106,
            name: "MetalWRLD".to_string(),
            group: "DRIVE".to_string(),
            title: "BOSS Meta Zone modeling".to_string(),
            order: 309,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // HotBox
    db.insert(
        0x00400106,
        EffectDef {
            id: 0x00400106,
            name: "HotBox".to_string(),
            group: "DRIVE".to_string(),
            title: "Matchless Hotbox pre-amp modeling".to_string(),
            order: 310,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z Clean
    db.insert(
        0x00600106,
        EffectDef {
            id: 0x00600106,
            name: "Z Clean".to_string(),
            group: "DRIVE".to_string(),
            title: "ZOOM original clean sound".to_string(),
            order: 311,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z MP 1
    db.insert(
        0x40000106,
        EffectDef {
            id: 0x40000106,
            name: "Z MP 1".to_string(),
            group: "DRIVE".to_string(),
            title: "Original sounds with ADA MP1 + Marshall JCM800".to_string(),
            order: 312,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z Bottom
    db.insert(
        0x40200106,
        EffectDef {
            id: 0x40200106,
            name: "Z Bottom".to_string(),
            group: "DRIVE".to_string(),
            title: "High gain sound with low-mid emphasis".to_string(),
            order: 313,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z Dream
    db.insert(
        0x40400106,
        EffectDef {
            id: 0x40400106,
            name: "Z Dream".to_string(),
            group: "DRIVE".to_string(),
            title: "High gain sounds based on Mesa Boogie Road King Series II Lead".to_string(),
            order: 314,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z Scream
    db.insert(
        0x40600106,
        EffectDef {
            id: 0x40600106,
            name: "Z Scream".to_string(),
            group: "DRIVE".to_string(),
            title: "Original balanced high-gain sounds".to_string(),
            order: 315,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z Neos
    db.insert(
        0x00000206,
        EffectDef {
            id: 0x00000206,
            name: "Z Neos".to_string(),
            group: "DRIVE".to_string(),
            title: "Crunch soudns of British class A combo amp".to_string(),
            order: 316,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z Wild
    db.insert(
        0x00200206,
        EffectDef {
            id: 0x00200206,
            name: "Z Wild".to_string(),
            group: "DRIVE".to_string(),
            title: "High-gain sound even more overdrive".to_string(),
            order: 317,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Lead
    db.insert(
        0x00400206,
        EffectDef {
            id: 0x00400206,
            name: "Lead".to_string(),
            group: "DRIVE".to_string(),
            title: "Bright and smooth distortion".to_string(),
            order: 318,
            install: 0,
            ver: 0x0002,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // ExtremeDS
    db.insert(
        0x00600206,
        EffectDef {
            id: 0x00600206,
            name: "ExtremeDS".to_string(),
            group: "DRIVE".to_string(),
            title: "Highest gain distortion".to_string(),
            order: 319,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Aco.Sim
    db.insert(
        0x40000206,
        EffectDef {
            id: 0x40000206,
            name: "Aco.Sim".to_string(),
            group: "DRIVE".to_string(),
            title: "Acoustic guitar simulator".to_string(),
            order: 320,
            install: 0,
            ver: 0x0001,
            dsp: 7.9892,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 10.0,
            params: vec![
                ParamDef {
                    name: "Top".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Body".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // CentaGold
    db.insert(
        0x40200206,
        EffectDef {
            id: 0x40200206,
            name: "CentaGold".to_string(),
            group: "DRIVE".to_string(),
            title: "Klon Centaur Gold overdrive modeling".to_string(),
            order: 321,
            install: 0,
            ver: 0x0003,
            dsp: 4.4444,
            dsp_max: 1.0 / 4.0,
            dsp_min: 1.0 / 5.0,
            params: vec![
                ParamDef {
                    name: "GAIN".to_string(),
                    def: 69,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TRBL".to_string(),
                    def: 56,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 43,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // NYC Muff
    db.insert(
        0x40400206,
        EffectDef {
            id: 0x40400206,
            name: "NYC Muff".to_string(),
            group: "DRIVE".to_string(),
            title: "Electro-Harmonix Big Muff Pi modeling".to_string(),
            order: 322,
            install: 0,
            ver: 0x0003,
            dsp: 6.9604,
            dsp_max: 1.0 / 6.0,
            dsp_min: 33.0 / 250.0,
            params: vec![
                ParamDef {
                    name: "VOL".to_string(),
                    def: 58,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 55,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "SUSTN".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // TS Drive
    db.insert(
        0x40600206,
        EffectDef {
            id: 0x40600206,
            name: "TS Drive".to_string(),
            group: "DRIVE".to_string(),
            title: "Ibanez TS808 modeling".to_string(),
            order: 323,
            install: 0,
            ver: 0x0003,
            dsp: 5.5385,
            dsp_max: 1.0 / 5.0,
            dsp_min: 1.0 / 6.0,
            params: vec![
                ParamDef {
                    name: "O.DRV".to_string(),
                    def: 74,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 57,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LEVEL".to_string(),
                    def: 82,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // BG_THRTTL
    db.insert(
        0x00000306,
        EffectDef {
            id: 0x00000306,
            name: "BG_THRTTL".to_string(),
            group: "DRIVE".to_string(),
            title: "Mesa Boogie THROTTLE BOX modeling".to_string(),
            order: 324,
            install: 0,
            ver: 0x0003,
            dsp: 2.8366,
            dsp_max: 13.0 / 36.0,
            dsp_min: 1.0 / 3.0,
            params: vec![
                ParamDef {
                    name: "LEVEL".to_string(),
                    def: 54,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LO/HI".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["LO", "HI"]),
                },
                ParamDef {
                    name: "GAIN".to_string(),
                    def: 78,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MdCut".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 56,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "BOOST".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // OctFuzz
    db.insert(
        0x00200306,
        EffectDef {
            id: 0x00200306,
            name: "OctFuzz".to_string(),
            group: "DRIVE".to_string(),
            title: "Fuzz adding an octave above".to_string(),
            order: 325,
            install: 0,
            ver: 0x0003,
            dsp: 4.4444,
            dsp_max: 1.0 / 4.0,
            dsp_min: 1.0 / 5.0,
            params: vec![
                ParamDef {
                    name: "VOL".to_string(),
                    def: 68,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "COLOR".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["1", "2"]),
                },
                ParamDef {
                    name: "BOOST".to_string(),
                    def: 65,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // BG GRID
    db.insert(
        0x00400306,
        EffectDef {
            id: 0x00400306,
            name: "BG GRID".to_string(),
            group: "DRIVE".to_string(),
            title: "Mesa Boogie GRID SLAMMER modeling".to_string(),
            order: 326,
            install: 0,
            ver: 0x0003,
            dsp: 3.4286,
            dsp_max: 37.0 / 144.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "LEVEL".to_string(),
                    def: 74,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "GAIN".to_string(),
                    def: 68,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // RedCrunch
    db.insert(
        0x00600306,
        EffectDef {
            id: 0x00600306,
            name: "RedCrunch".to_string(),
            group: "DRIVE".to_string(),
            title: "Effect for EVH 'Brown Sound'".to_string(),
            order: 327,
            install: 0,
            ver: 0x0003,
            dsp: 4.4444,
            dsp_max: 1.0 / 4.0,
            dsp_min: 28.0 / 125.0,
            params: vec![
                ParamDef {
                    name: "VOL".to_string(),
                    def: 61,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LO/HI".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["LO", "HI"]),
                },
                ParamDef {
                    name: "GAIN".to_string(),
                    def: 68,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PRES".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "COMP".to_string(),
                    def: 0,
                    max: 2,
                    disp: labels(&["1", "0", "2"]),
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 47,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // TB MK1.5
    db.insert(
        0x40000306,
        EffectDef {
            id: 0x40000306,
            name: "TB MK1.5".to_string(),
            group: "DRIVE".to_string(),
            title: "Classic fuzz".to_string(),
            order: 328,
            install: 0,
            ver: 0x0003,
            dsp: 3.4286,
            dsp_max: 28.0 / 100.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "LEVEL".to_string(),
                    def: 92,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "COLOR".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["1", "2"]),
                },
                ParamDef {
                    name: "ATTCK".to_string(),
                    def: 90,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SweetDrv
    db.insert(
        0x40200306,
        EffectDef {
            id: 0x40200306,
            name: "SweetDrv".to_string(),
            group: "DRIVE".to_string(),
            title: "Modeling of a sweet sounding overdrive".to_string(),
            order: 329,
            install: 0,
            ver: 0x0003,
            dsp: 2.6334,
            dsp_max: 38.0 / 100.0,
            dsp_min: 1.0 / 3.0,
            params: vec![
                ParamDef {
                    name: "VOL".to_string(),
                    def: 62,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FOCUS".to_string(),
                    def: 67,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DRIVE".to_string(),
                    def: 78,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // RC Boost
    db.insert(
        0x40600306,
        EffectDef {
            id: 0x40600306,
            name: "RC Boost".to_string(),
            group: "DRIVE".to_string(),
            title: "Booster for from clean to light drives".to_string(),
            order: 330,
            install: 0,
            ver: 0x0003,
            dsp: 4.4444,
            dsp_max: 1.0 / 4.0,
            dsp_min: 2.0 / 9.0,
            params: vec![
                ParamDef {
                    name: "GAIN".to_string(),
                    def: 58,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TRBL".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "BASS".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "VOL".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // DynmcDrv
    db.insert(
        0x00200406,
        EffectDef {
            id: 0x00200406,
            name: "DynmcDrv".to_string(),
            group: "DRIVE".to_string(),
            title: "Warm drive tone of a tube amp".to_string(),
            order: 331,
            install: 0,
            ver: 0x0003,
            dsp: 3.4286,
            dsp_max: 28.0 / 100.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "LEVEL".to_string(),
                    def: 62,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 67,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "GAIN".to_string(),
                    def: 78,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MODE".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["COMBO", "STACK"]),
                },
            ],
        },
    );

    // AMP Section - FD COMBO
    db.insert(
        0x00100008,
        EffectDef {
            id: 0x00100008,
            name: "FD COMBO".to_string(),
            group: "AMP".to_string(),
            title: "Fender Twin Reverb ('65) modeling".to_string(),
            order: 400,
            install: 0,
            ver: 0x0001,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 24,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 86,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 44,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 8,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // DELUXE-R
    db.insert(
        0x00200008,
        EffectDef {
            id: 0x00200008,
            name: "DELUXE-R".to_string(),
            group: "AMP".to_string(),
            title: "Fender Deluxe Reverb ('65) modeling".to_string(),
            order: 401,
            install: 0,
            ver: 0x0001,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 16,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // FD VIBRO
    db.insert(
        0x00400008,
        EffectDef {
            id: 0x00400008,
            name: "FD VIBRO".to_string(),
            group: "AMP".to_string(),
            title: "Fender Vibroverb ('63) modeling".to_string(),
            order: 402,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 56,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 98,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 54,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 47,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 32,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // US BLUES
    db.insert(
        0x00600008,
        EffectDef {
            id: 0x00600008,
            name: "US BLUES".to_string(),
            group: "AMP".to_string(),
            title: "Fender Tweed Bassman modeling".to_string(),
            order: 403,
            install: 0,
            ver: 0x0001,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 59,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 107,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 58,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 48,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // VX COMBO
    db.insert(
        0x40000008,
        EffectDef {
            id: 0x40000008,
            name: "VX COMBO".to_string(),
            group: "AMP".to_string(),
            title: "British combo amp modeling".to_string(),
            order: 404,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 31,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 44,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 49,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 53,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 64,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // VX JMI
    db.insert(
        0x40200008,
        EffectDef {
            id: 0x40200008,
            name: "VX JMI".to_string(),
            group: "AMP".to_string(),
            title: "Class-A British combo amp modeling".to_string(),
            order: 405,
            install: 0,
            ver: 0x0001,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 80,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // BG CRUNCH
    db.insert(
        0x40400008,
        EffectDef {
            id: 0x40400008,
            name: "BG CRUNCH".to_string(),
            group: "AMP".to_string(),
            title: "Mesa Boogie MkIII modeling".to_string(),
            order: 406,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 57,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 99,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 96,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // MATCH 30
    db.insert(
        0x40600008,
        EffectDef {
            id: 0x40600008,
            name: "MATCH 30".to_string(),
            group: "AMP".to_string(),
            title: "Matchless DC-30(channel-1) modeling".to_string(),
            order: 408,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 28,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 53,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 112,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // CAR DRIVE
    db.insert(
        0x00000108,
        EffectDef {
            id: 0x00000108,
            name: "CAR DRIVE".to_string(),
            group: "AMP".to_string(),
            title: "Carr Mercury combo amp modeling".to_string(),
            order: 409,
            install: 0,
            ver: 0x0003,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 51,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 74,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 128,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // TW ROCK
    db.insert(
        0x00200108,
        EffectDef {
            id: 0x00200108,
            name: "TW ROCK".to_string(),
            group: "AMP".to_string(),
            title: "Two Rock Emerald 50 drive channel".to_string(),
            order: 410,
            install: 0,
            ver: 0x0001,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 53,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 95,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 51,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 144,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // TONE CITY
    db.insert(
        0x00400108,
        EffectDef {
            id: 0x00400108,
            name: "TONE CITY".to_string(),
            group: "AMP".to_string(),
            title: "Sound City 50 Plus Mark 2 modeling".to_string(),
            order: 411,
            install: 0,
            ver: 0x0003,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 78,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 89,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 54,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 160,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // HW STACK
    db.insert(
        0x00600108,
        EffectDef {
            id: 0x00600108,
            name: "HW STACK".to_string(),
            group: "AMP".to_string(),
            title: "Hiwatt Custom 100 tube amp modeling".to_string(),
            order: 412,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 54,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 106,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 56,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 176,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // TANGERINE
    db.insert(
        0x40000108,
        EffectDef {
            id: 0x40000108,
            name: "TANGERINE".to_string(),
            group: "AMP".to_string(),
            title: "Orange Graphic 120 modeling".to_string(),
            order: 413,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 99,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 192,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // B-BREAKER
    db.insert(
        0x40200108,
        EffectDef {
            id: 0x40200108,
            name: "B-BREAKER".to_string(),
            group: "AMP".to_string(),
            title: "Marshall 1962 Bluesbreaker modeling".to_string(),
            order: 414,
            install: 0,
            ver: 0x0003,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 61,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 31,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 51,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 208,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // MS CRUNCH
    db.insert(
        0x40400108,
        EffectDef {
            id: 0x40400108,
            name: "MS CRUNCH".to_string(),
            group: "AMP".to_string(),
            title: "Marshall 1959 crunch sound modeling".to_string(),
            order: 415,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 72,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 98,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 53,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 54,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 224,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // MS 1959
    db.insert(
        0x40600108,
        EffectDef {
            id: 0x40600108,
            name: "MS 1959".to_string(),
            group: "AMP".to_string(),
            title: "Marshall 1959 Plexi ('69)".to_string(),
            order: 416,
            install: 0,
            ver: 0x0001,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 58,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 240,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // MS DRIVE
    db.insert(
        0x00000208,
        EffectDef {
            id: 0x00000208,
            name: "MS DRIVE".to_string(),
            group: "AMP".to_string(),
            title: "Marshall JCM2000 high gain sound modeling".to_string(),
            order: 417,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 82,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 103,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 56,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 53,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 256,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // BGN DRIVE
    db.insert(
        0x00200208,
        EffectDef {
            id: 0x00200208,
            name: "BGN DRIVE".to_string(),
            group: "AMP".to_string(),
            title: "Bogner Ecstasy lead sound modeling".to_string(),
            order: 418,
            install: 0,
            ver: 0x0003,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 84,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 91,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 49,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 272,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // BG DRIVE
    db.insert(
        0x00400208,
        EffectDef {
            id: 0x00400208,
            name: "BG DRIVE".to_string(),
            group: "AMP".to_string(),
            title: "Mesa Boogie Dual Rectifier red channel modeling".to_string(),
            order: 419,
            install: 0,
            ver: 0x0003,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 47,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 97,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 288,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // DZ DRIVE
    db.insert(
        0x00600208,
        EffectDef {
            id: 0x00600208,
            name: "DZ DRIVE".to_string(),
            group: "AMP".to_string(),
            title: "High gain sound of Diezel Herbert".to_string(),
            order: 420,
            install: 0,
            ver: 0x0001,
            dsp: 3.1102,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 93,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 53,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 47,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 51,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 55,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 304,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // ALIEN
    db.insert(
        0x40000208,
        EffectDef {
            id: 0x40000208,
            name: "ALIEN".to_string(),
            group: "AMP".to_string(),
            title: "Engl Invader modeling".to_string(),
            order: 421,
            install: 0,
            ver: 0x0001,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 62,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 87,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 44,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 54,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 320,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // REVO-1
    db.insert(
        0x40200208,
        EffectDef {
            id: 0x40200208,
            name: "REVO-1".to_string(),
            group: "AMP".to_string(),
            title: "Krank Revolution 1 Plus modeling".to_string(),
            order: 422,
            install: 0,
            ver: 0x0003,
            dsp: 2.3404,
            dsp_max: 41.0 / 100.0,
            dsp_min: 547.0 / 1500.0,
            params: vec![
                ParamDef {
                    name: "Gain".to_string(),
                    def: 64,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tube".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 89,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Trebl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Middl".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bass".to_string(),
                    def: 51,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Prese".to_string(),
                    def: 51,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "CAB".to_string(),
                    def: 336,
                    max: 255,
                    disp: labels(GAMPCAB_DISP),
                },
                ParamDef {
                    name: "OUT".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&[
                        "LINE",
                        "COMBO FRONT",
                        "STACK FRONT",
                        "COMBO POWER AMP",
                        "STACK POWER AMP",
                    ]),
                },
            ],
        },
    );

    // MOD Section - Tremolo
    db.insert(
        0x0010000c,
        EffectDef {
            id: 0x0010000c,
            name: "Tremolo".to_string(),
            group: "MOD".to_string(),
            title: "Volume varieing effect".to_string(),
            order: 500,
            install: 0,
            ver: 0x0211,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 33,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 130,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Wave".to_string(),
                    def: 21,
                    max: 29,
                    disp: labels(&[
                        "UP 0", "UP 1", "UP 2", "UP 3", "UP 4", "UP 5", "UP 6", "UP 7", "UP 8",
                        "UP 9", "DWN 0", "DWN 1", "DWN 2", "DWN 3", "DWN 4", "DWN 5", "DWN 6",
                        "DWN 7", "DWN 8", "DWN 9", "TRI 0", "TRI 1", "TRI 2", "TRI 3", "TRI 4",
                        "TRI 5", "TRI 6", "TRI 7", "TRI 8", "TRI 9",
                    ]),
                },
            ],
        },
    );

    // DuoTrem
    db.insert(
        0x0020000c,
        EffectDef {
            id: 0x0020000c,
            name: "DuoTrem".to_string(),
            group: "MOD".to_string(),
            title: "Combines two tremolos".to_string(),
            order: 501,
            install: 0,
            ver: 0x0123,
            dsp: 12.7757,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "RateA".to_string(),
                    def: 46,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "RateB".to_string(),
                    def: 5,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DPT_A".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DPT_B".to_string(),
                    def: 90,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Link".to_string(),
                    def: 0,
                    max: 2,
                    disp: labels(&["Seri", "Para", "STR"]),
                },
                ParamDef {
                    name: "WaveA".to_string(),
                    def: 19,
                    max: 29,
                    disp: labels(&[
                        "UP 0", "UP 1", "UP 2", "UP 3", "UP 4", "UP 5", "UP 6", "UP 7", "UP 8",
                        "UP 9", "DWN 0", "DWN 1", "DWN 2", "DWN 3", "DWN 4", "DWN 5", "DWN 6",
                        "DWN 7", "DWN 8", "DWN 9", "TRI 0", "TRI 1", "TRI 2", "TRI 3", "TRI 4",
                        "TRI 5", "TRI 6", "TRI 7", "TRI 8", "TRI 9",
                    ]),
                },
                ParamDef {
                    name: "WaveB".to_string(),
                    def: 21,
                    max: 29,
                    disp: labels(&[
                        "UP 0", "UP 1", "UP 2", "UP 3", "UP 4", "UP 5", "UP 6", "UP 7", "UP 8",
                        "UP 9", "DWN 0", "DWN 1", "DWN 2", "DWN 3", "DWN 4", "DWN 5", "DWN 6",
                        "DWN 7", "DWN 8", "DWN 9", "TRI 0", "TRI 1", "TRI 2", "TRI 3", "TRI 4",
                        "TRI 5", "TRI 6", "TRI 7", "TRI 8", "TRI 9",
                    ]),
                },
            ],
        },
    );

    // Slicer
    db.insert(
        0x0040000c,
        EffectDef {
            id: 0x0040000c,
            name: "Slicer".to_string(),
            group: "MOD".to_string(),
            title: "Rhythmical sounds by slicing".to_string(),
            order: 502,
            install: 0,
            ver: 0x0202,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "PTTRN".to_string(),
                    def: 0,
                    max: 19,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Speed".to_string(),
                    def: 24,
                    max: 77,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 20,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 130,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Phaser
    db.insert(
        0x0060000c,
        EffectDef {
            id: 0x0060000c,
            name: "Phaser".to_string(),
            group: "MOD".to_string(),
            title: "Phase varieing effect".to_string(),
            order: 503,
            install: 0,
            ver: 0x0111,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Rate".to_string(),
                    def: 11,
                    max: 77,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Color".to_string(),
                    def: 3,
                    max: 3,
                    disp: labels(&["4 STG", "8 STG", "inv 4", "inv 8"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // DuoPhase
    db.insert(
        0x006a000c,
        EffectDef {
            id: 0x006a000c,
            name: "DuoPhase".to_string(),
            group: "MOD".to_string(),
            title: "Combines 2 phasers".to_string(),
            order: 504,
            install: 0,
            ver: 0x0221,
            dsp: 9.4118,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "RateA".to_string(),
                    def: 46,
                    max: 77,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "RateB".to_string(),
                    def: 5,
                    max: 51,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ResoA".to_string(),
                    def: 0,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ResoB".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Link".to_string(),
                    def: 0,
                    max: 2,
                    disp: labels(&["Seri", "Para", "STR"]),
                },
                ParamDef {
                    name: "DPT_A".to_string(),
                    def: 36,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "DPT_B".to_string(),
                    def: 62,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
            ],
        },
    );

    // WarpPhase
    db.insert(
        0x0074000c,
        EffectDef {
            id: 0x0074000c,
            name: "WarpPhase".to_string(),
            group: "MOD".to_string(),
            title: "Phaser with one way effect".to_string(),
            order: 505,
            install: 0,
            ver: 0x0222,
            dsp: 9.4118,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Speed".to_string(),
                    def: 24,
                    max: 77,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DRCTN".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Go", "Back"]),
                },
            ],
        },
    );

    // TheVibe
    db.insert(
        0x4000000c,
        EffectDef {
            id: 0x4000000c,
            name: "TheVibe".to_string(),
            group: "MOD".to_string(),
            title: "Unique undulations vibe".to_string(),
            order: 506,
            install: 0,
            ver: 0x0121,
            dsp: 9.4118,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Speed".to_string(),
                    def: 25,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bias".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Wave".to_string(),
                    def: 24,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["VIBRT", "CHORS"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Chorus
    db.insert(
        0x4060000c,
        EffectDef {
            id: 0x4060000c,
            name: "Chorus".to_string(),
            group: "MOD".to_string(),
            title: "Mixing shifted pitch effect".to_string(),
            order: 507,
            install: 0,
            ver: 0x0101,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 12.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 24,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Detune
    db.insert(
        0x0000010c,
        EffectDef {
            id: 0x0000010c,
            name: "Detune".to_string(),
            group: "MOD".to_string(),
            title: "Chorus without modulation by slightly pitch-shifted mix".to_string(),
            order: 508,
            install: 0,
            ver: 0x0101,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Cent".to_string(),
                    def: 35,
                    max: 50,
                    disp: offset(-25),
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 0,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // VintageCE
    db.insert(
        0x0020010c,
        EffectDef {
            id: 0x0020010c,
            name: "VintageCE".to_string(),
            group: "MOD".to_string(),
            title: "BOSS CE-1 modeling".to_string(),
            order: 509,
            install: 0,
            ver: 0x0122,
            dsp: 10.9091,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Comp".to_string(),
                    def: 2,
                    max: 9,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 24,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // StereoCho
    db.insert(
        0x0040010c,
        EffectDef {
            id: 0x0040010c,
            name: "StereoCho".to_string(),
            group: "MOD".to_string(),
            title: "Stereo chorus".to_string(),
            order: 510,
            install: 0,
            ver: 0x0121,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 29,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Ensemble
    db.insert(
        0x0060010c,
        EffectDef {
            id: 0x0060010c,
            name: "Ensemble".to_string(),
            group: "MOD".to_string(),
            title: "Chorus with 3D movement".to_string(),
            order: 511,
            install: 0,
            ver: 0x0102,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 29,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SuperCho
    db.insert(
        0x4020010c,
        EffectDef {
            id: 0x4020010c,
            name: "SuperCho".to_string(),
            group: "MOD".to_string(),
            title: "BOSS CH-1 SUPER CHORUS modeling".to_string(),
            order: 512,
            install: 0,
            ver: 0x0121,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "E.LVL".to_string(),
                    def: 50,
                    max: 120,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "EQ".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["MONO", "STR"]),
                },
            ],
        },
    );

    // VinFLNGR
    db.insert(
        0x4030010c,
        EffectDef {
            id: 0x4030010c,
            name: "VinFLNGR".to_string(),
            group: "MOD".to_string(),
            title: "MXR M-117R like analog flanger".to_string(),
            order: 513,
            install: 0,
            ver: 0x0222,
            dsp: 12.4737,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 47,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 7,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 18,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 4,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 65,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Flanger
    db.insert(
        0x4040010c,
        EffectDef {
            id: 0x4040010c,
            name: "Flanger".to_string(),
            group: "MOD".to_string(),
            title: "ADA flanger like jet sound".to_string(),
            order: 514,
            install: 0,
            ver: 0x0101,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 47,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 7,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 18,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 4,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 65,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // DynaFLNGR
    db.insert(
        0x4060010c,
        EffectDef {
            id: 0x4060010c,
            name: "DynaFLNGR".to_string(),
            group: "MOD".to_string(),
            title: "Flanger with effect changes according to input level".to_string(),
            order: 515,
            install: 0,
            ver: 0x0222,
            dsp: 9.4118,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 38,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Sense".to_string(),
                    def: 0,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 15,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Vibrato
    db.insert(
        0x0000020c,
        EffectDef {
            id: 0x0000020c,
            name: "Vibrato".to_string(),
            group: "MOD".to_string(),
            title: "Automatic vibrato".to_string(),
            order: 516,
            install: 0,
            ver: 0x0121,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 30,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 72,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 120,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Octave
    db.insert(
        0x0020020c,
        EffectDef {
            id: 0x0020020c,
            name: "Octave".to_string(),
            group: "MOD".to_string(),
            title: "Adding one/two octave below sound".to_string(),
            order: 517,
            install: 0,
            ver: 0x0201,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Oct1".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Oct2".to_string(),
                    def: 15,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Chara".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // PitchSHFT
    db.insert(
        0x0040020c,
        EffectDef {
            id: 0x0040020c,
            name: "PitchSHFT".to_string(),
            group: "MOD".to_string(),
            title: "Pitch shift up or down".to_string(),
            order: 518,
            install: 0,
            ver: 0x0111,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Shift".to_string(),
                    def: 19,
                    max: 25,
                    disp: labels(&[
                        "-12", "-11", "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1",
                        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "24",
                    ]),
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Fine".to_string(),
                    def: 25,
                    max: 50,
                    disp: offset(-25),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // MonoPitch
    db.insert(
        0x0060020c,
        EffectDef {
            id: 0x0060020c,
            name: "MonoPitch".to_string(),
            group: "MOD".to_string(),
            title: "Sound variance pitch shifter for monophonic".to_string(),
            order: 519,
            install: 0,
            ver: 0x0201,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Shift".to_string(),
                    def: 0,
                    max: 25,
                    disp: offset(-12),
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Fine".to_string(),
                    def: 25,
                    max: 50,
                    disp: offset(-25),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // HPS
    db.insert(
        0x4000020c,
        EffectDef {
            id: 0x4000020c,
            name: "HPS".to_string(),
            group: "MOD".to_string(),
            title: "Intelligent pitch shifter according to scale/key".to_string(),
            order: 520,
            install: 0,
            ver: 0x0101,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Scale".to_string(),
                    def: 6,
                    max: 9,
                    disp: labels(&["-6", "-5", "-4", "-3", "-m", "m", "3", "4", "5", "6"]),
                },
                ParamDef {
                    name: "Key".to_string(),
                    def: 0,
                    max: 11,
                    disp: labels(&[
                        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
                    ]),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // BendCho
    db.insert(
        0x4020020c,
        EffectDef {
            id: 0x4020020c,
            name: "BendCho".to_string(),
            group: "MOD".to_string(),
            title: "Pitch bending each input note".to_string(),
            order: 521,
            install: 0,
            ver: 0x0202,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Time".to_string(),
                    def: 50,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Up", "Down"]),
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // MojoRolle
    db.insert(
        0x4040020c,
        EffectDef {
            id: 0x4040020c,
            name: "MojoRolle".to_string(),
            group: "MOD".to_string(),
            title: "Pitch modulation after picking".to_string(),
            order: 522,
            install: 0,
            ver: 0x0202,
            dsp: 9.4118,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 37,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Speed".to_string(),
                    def: 57,
                    max: 128,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rise".to_string(),
                    def: 0,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 2,
                    disp: labels(&["Up-Dn", "Up", "Down"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // RingMod
    db.insert(
        0x4060020c,
        EffectDef {
            id: 0x4060020c,
            name: "RingMod".to_string(),
            group: "MOD".to_string(),
            title: "Metallic ringing sound".to_string(),
            order: 523,
            install: 0,
            ver: 0x0222,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Freq".to_string(),
                    def: 27,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 120,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // CE-Cho5
    db.insert(
        0x0000030c,
        EffectDef {
            id: 0x0000030c,
            name: "CE-Cho5".to_string(),
            group: "MOD".to_string(),
            title: "BOSS CE-5 chorus modeling".to_string(),
            order: 524,
            install: 0,
            ver: 0x0123,
            dsp: 10.1887,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "E.LVL".to_string(),
                    def: 100,
                    max: 120,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "RATE".to_string(),
                    def: 31,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DEPTH".to_string(),
                    def: 67,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LOW".to_string(),
                    def: 43,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HIGH".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MODE".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["MONO", "STR"]),
                },
            ],
        },
    );

    // CloneCho
    db.insert(
        0x0020030c,
        EffectDef {
            id: 0x0020030c,
            name: "CloneCho".to_string(),
            group: "MOD".to_string(),
            title: "Electro-Harmonix SmallClone chorus modeling".to_string(),
            order: 525,
            install: 0,
            ver: 0x0123,
            dsp: 9.4118,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "DEPTH".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["1", "2"]),
                },
                ParamDef {
                    name: "RATE".to_string(),
                    def: 23,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // StonePha
    db.insert(
        0x0040030c,
        EffectDef {
            id: 0x0040030c,
            name: "StonePha".to_string(),
            group: "MOD".to_string(),
            title: "Electro-Harmonix SmallStone phaser modeling".to_string(),
            order: 526,
            install: 0,
            ver: 0x0223,
            dsp: 10.3226,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "COLOR".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["1", "2"]),
                },
                ParamDef {
                    name: "RATE".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // BF FLG 2
    db.insert(
        0x0060030c,
        EffectDef {
            id: 0x0060030c,
            name: "BF FLG 2".to_string(),
            group: "MOD".to_string(),
            title: "BOSS BF-2 Flanger modeling".to_string(),
            order: 527,
            install: 0,
            ver: 0x0223,
            dsp: 9.4118,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "MNL".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DEPTH".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "RATE".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "RES".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // SilkyCho
    db.insert(
        0x4000030c,
        EffectDef {
            id: 0x4000030c,
            name: "SilkyCho".to_string(),
            group: "MOD".to_string(),
            title: "2 bands detune and chorus".to_string(),
            order: 528,
            install: 0,
            ver: 0x0103,
            dsp: 5.4545,
            dsp_max: 1.0 / 5.0,
            dsp_min: 1.0 / 6.0,
            params: vec![
                ParamDef {
                    name: "LoMix".to_string(),
                    def: 38,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiMix".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ChMix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LoPit".to_string(),
                    def: 31,
                    max: 50,
                    disp: offset(-25),
                },
                ParamDef {
                    name: "HiPit".to_string(),
                    def: 33,
                    max: 50,
                    disp: offset(-25),
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 27,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // MirageCho
    db.insert(
        0x4020030c,
        EffectDef {
            id: 0x4020030c,
            name: "MirageCho".to_string(),
            group: "MOD".to_string(),
            title: "Chorus like mirage".to_string(),
            order: 529,
            install: 0,
            ver: 0x0103,
            dsp: 9.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 38,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 55,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 10,
                    max: 19,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // CoronaCho
    db.insert(
        0x4040030c,
        EffectDef {
            id: 0x4040030c,
            name: "CoronaCho".to_string(),
            group: "MOD".to_string(),
            title: "tc electronic CORONA CHORUS modeling".to_string(),
            order: 530,
            install: 0,
            ver: 0x0123,
            dsp: 5.4545,
            dsp_max: 23.0 / 125.0,
            dsp_min: 1094.0 / 6000.0,
            params: vec![
                ParamDef {
                    name: "SPEED".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DEPTH".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FxLVL".to_string(),
                    def: 65,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 75,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DRY".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // ANA234Cho
    db.insert(
        0x4060030c,
        EffectDef {
            id: 0x4060030c,
            name: "ANA234Cho".to_string(),
            group: "MOD".to_string(),
            title: "MXR M234 analog chorus modeling".to_string(),
            order: 531,
            install: 0,
            ver: 0x0123,
            dsp: 7.2000,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "LEVEL".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "RATE".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DEPTH".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LOW".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HIGH".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["MONO", "STR"]),
                },
            ],
        },
    );

    // CoronaTri
    db.insert(
        0x0000040c,
        EffectDef {
            id: 0x0000040c,
            name: "CoronaTri".to_string(),
            group: "MOD".to_string(),
            title: "tc electonic CORONA Tri-Chorus modeling".to_string(),
            order: 532,
            install: 0,
            ver: 0x0123,
            dsp: 3.6923,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "SPEED".to_string(),
                    def: 25,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DEPTH".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FxLVL".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DRY".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // SFX Section - BitCrush
    db.insert(
        0x0020000e,
        EffectDef {
            id: 0x0020000e,
            name: "BitCrush".to_string(),
            group: "SFX".to_string(),
            title: "Lo-Fi effect".to_string(),
            order: 600,
            install: 0,
            ver: 0x0222,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Bit".to_string(),
                    def: 5,
                    max: 12,
                    disp: ParamDisp::None, // JS had disp:4
                },
                ParamDef {
                    name: "SMPL".to_string(),
                    def: 2,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 90,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Bomber
    db.insert(
        0x0040000e,
        EffectDef {
            id: 0x0040000e,
            name: "Bomber".to_string(),
            group: "SFX".to_string(),
            title: "Explosive sound effect".to_string(),
            order: 601,
            install: 0,
            ver: 0x0222,
            dsp: 10.7911,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "PTTRN".to_string(),
                    def: 3,
                    max: 3,
                    disp: labels(&["HndGn", "Arm", "Bomb", "Thndr"]),
                },
                ParamDef {
                    name: "Deay".to_string(),
                    def: 49,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 15,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 40,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Power".to_string(),
                    def: 30,
                    max: 30,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 4,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // MonoSynth
    db.insert(
        0x0060000e,
        EffectDef {
            id: 0x0060000e,
            name: "MonoSynth".to_string(),
            group: "SFX".to_string(),
            title: "Monophonic guitar synth effect".to_string(),
            order: 602,
            install: 0,
            ver: 0x0202,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Synth".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Wave".to_string(),
                    def: 2,
                    max: 3,
                    disp: labels(&["Sine", "Tri", "SawUp", "SawDn"]),
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Speed".to_string(),
                    def: 0,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Z-Organ
    db.insert(
        0x4000000e,
        EffectDef {
            id: 0x4000000e,
            name: "Z-Organ".to_string(),
            group: "SFX".to_string(),
            title: "Organ sound effect".to_string(),
            order: 603,
            install: 0,
            ver: 0x0222,
            dsp: 7.2000,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Upper".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Lower".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HPF".to_string(),
                    def: 3,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LPF".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // AutoPan
    db.insert(
        0x4020000e,
        EffectDef {
            id: 0x4020000e,
            name: "AutoPan".to_string(),
            group: "SFX".to_string(),
            title: "Cyclic panning position movement".to_string(),
            order: 604,
            install: 0,
            ver: 0x0122,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Rate".to_string(),
                    def: 5,
                    max: 78,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Width".to_string(),
                    def: 100,
                    max: 100,
                    disp: offset(-50),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Clip".to_string(),
                    def: 0,
                    max: 10,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Rt Closet
    db.insert(
        0x4040000e,
        EffectDef {
            id: 0x4040000e,
            name: "Rt Closet".to_string(),
            group: "SFX".to_string(),
            title: "Rotary speaker simulation".to_string(),
            order: 605,
            install: 0,
            ver: 0x0122,
            dsp: 4.4444,
            dsp_max: 1.0 / 4.0,
            dsp_min: 1.0 / 5.0,
            params: vec![
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["Slow", "Fast"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Drive".to_string(),
                    def: 20,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // DELAY Section - Delay
    db.insert(
        0x00100010,
        EffectDef {
            id: 0x00100010,
            name: "Delay".to_string(),
            group: "DELAY".to_string(),
            title: "Long delay upto 4000ms".to_string(),
            order: 700,
            install: 0,
            ver: 0x0111,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 559,
                    max: 4022,
                    disp: time_display(
                        1,
                        4001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                            "&#x1D15F; x9",
                            "&#x1D15F; x10",
                            "&#x1D15F; x11",
                            "&#x1D15F; x12",
                            "&#x1D15F; x13",
                            "&#x1D15F; x14",
                            "&#x1D15F; x15",
                            "&#x1D15F; x16",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "P-P".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["MONO", "P-P"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // TapeEcho
    db.insert(
        0x00200010,
        EffectDef {
            id: 0x00200010,
            name: "TapeEcho".to_string(),
            group: "DELAY".to_string(),
            title: "Tape echo simulation".to_string(),
            order: 701,
            install: 0,
            ver: 0x0121,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 559,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 64,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 56,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 5,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // ModDelay
    db.insert(
        0x00400010,
        EffectDef {
            id: 0x00400010,
            name: "ModDelay".to_string(),
            group: "DELAY".to_string(),
            title: "Delay effect with modulation".to_string(),
            order: 702,
            install: 0,
            ver: 0x0121,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 499,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 62,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 20,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // AnalogDly
    db.insert(
        0x00600010,
        EffectDef {
            id: 0x00600010,
            name: "AnalogDly".to_string(),
            group: "DELAY".to_string(),
            title: "Analog delay simulation".to_string(),
            order: 703,
            install: 0,
            ver: 0x0121,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 359,
                    max: 4022,
                    disp: time_display(
                        1,
                        4001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                            "&#x1D15F; x9",
                            "&#x1D15F; x10",
                            "&#x1D15F; x11",
                            "&#x1D15F; x12",
                            "&#x1D15F; x13",
                            "&#x1D15F; x14",
                            "&#x1D15F; x15",
                            "&#x1D15F; x16",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 28,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "P-P".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["MONO", "P-P"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // ReverseDL
    db.insert(
        0x40000010,
        EffectDef {
            id: 0x40000010,
            name: "ReverseDL".to_string(),
            group: "DELAY".to_string(),
            title: "Reverse delay upto 2000ms".to_string(),
            order: 704,
            install: 0,
            ver: 0x0121,
            dsp: 14.2628,
            dsp_max: 1.0 / 12.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 990,
                    max: 2005,
                    disp: time_display(
                        10,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 20,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Bal".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // MultiTapD
    db.insert(
        0x40200010,
        EffectDef {
            id: 0x40200010,
            name: "MultiTapD".to_string(),
            group: "DELAY".to_string(),
            title: "Several delay sounds with different delay times".to_string(),
            order: 705,
            install: 0,
            ver: 0x0122,
            dsp: 10.7911,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 2999,
                    max: 3018,
                    disp: time_display(
                        1,
                        3001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                            "&#x1D15F; x9",
                            "&#x1D15F; x10",
                            "&#x1D15F; x11",
                            "&#x1D15F; x12",
                        ],
                    ),
                },
                ParamDef {
                    name: "PTTRN".to_string(),
                    def: 1,
                    max: 7,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 20,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // DynaDelay
    db.insert(
        0x40400010,
        EffectDef {
            id: 0x40400010,
            name: "DynaDelay".to_string(),
            group: "DELAY".to_string(),
            title: "Delay with dynamics adjusting according to input level".to_string(),
            order: 706,
            install: 0,
            ver: 0x0122,
            dsp: 10.7911,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 359,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "Sense".to_string(),
                    def: 5,
                    max: 19,
                    disp: labels(&[
                        "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3",
                        "4", "5", "6", "7", "8", "9", "10",
                    ]),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // FilterDly
    db.insert(
        0x40600010,
        EffectDef {
            id: 0x40600010,
            name: "FilterDly".to_string(),
            group: "DELAY".to_string(),
            title: "Delay effect with filter".to_string(),
            order: 707,
            install: 0,
            ver: 0x0122,
            dsp: 10.7911,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 499,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 90,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 6,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Reso".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // PitchDly
    db.insert(
        0x00000110,
        EffectDef {
            id: 0x00000110,
            name: "PitchDly".to_string(),
            group: "DELAY".to_string(),
            title: "Delay effect with pitch-shifting".to_string(),
            order: 708,
            install: 0,
            ver: 0x0122,
            dsp: 7.5224,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 89,
                    max: 1999,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Pitch".to_string(),
                    def: 21,
                    max: 30,
                    disp: labels(&[
                        "-12", "-11", "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1",
                        "-0.15", "-0.10", "-0.05", "0", "0.05", "0.10", "0.15", "1", "2", "3", "4",
                        "5", "6", "7", "8", "9", "10", "11", "12",
                    ]),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // StereoDly
    db.insert(
        0x00200110,
        EffectDef {
            id: 0x00200110,
            name: "StereoDly".to_string(),
            group: "DELAY".to_string(),
            title: "Stereo delay with L/R separate delay times".to_string(),
            order: 709,
            install: 0,
            ver: 0x0122,
            dsp: 10.7911,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "TimeL".to_string(),
                    def: 164,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "TimeR".to_string(),
                    def: 504,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LchFB".to_string(),
                    def: 55,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "RchFB".to_string(),
                    def: 37,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LchLv".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "RchLv".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // PhaseDly
    db.insert(
        0x00400110,
        EffectDef {
            id: 0x00400110,
            name: "PhaseDly".to_string(),
            group: "DELAY".to_string(),
            title: "Delay effect with phaser".to_string(),
            order: 710,
            install: 0,
            ver: 0x0122,
            dsp: 8.0000,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 499,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 28,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 57,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 49,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Color".to_string(),
                    def: 3,
                    max: 3,
                    disp: labels(&["4 STG", "8 STG", "inv 4", "inv 8"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // TrgHldDly
    db.insert(
        0x00600110,
        EffectDef {
            id: 0x00600110,
            name: "TrgHldDly".to_string(),
            group: "DELAY".to_string(),
            title: "Delay effect with sample&hold by picking".to_string(),
            order: 711,
            install: 0,
            ver: 0x0102,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 9.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 40,
                    max: 990,
                    disp: ParamDisp::None, // JS had disp:10
                },
                ParamDef {
                    name: "Duty".to_string(),
                    def: 75,
                    max: 75,
                    disp: ParamDisp::None, // JS had disp:25
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "THRSH".to_string(),
                    def: 20,
                    max: 30,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // StompDly
    db.insert(
        0x40000110,
        EffectDef {
            id: 0x40000110,
            name: "StompDly".to_string(),
            group: "DELAY".to_string(),
            title: "Stomp style Self-Oscillable delay".to_string(),
            order: 712,
            install: 0,
            ver: 0x0111,
            dsp: 12.4737,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "E.LVL".to_string(),
                    def: 30,
                    max: 120,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 20,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Time".to_string(),
                    def: 359,
                    max: 599,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Sync".to_string(),
                    def: 0,
                    max: 15,
                    disp: labels(&[
                        "OFF", "1/16", "1/12", "3/32", "1/8", "1/6", "3/16", "1/4", "3/8", "1/2",
                        "3/4", "4/4", "5/4", "6/4", "7/4", "8/4",
                    ]),
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["MONO", "STR"]),
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 5,
                    max: 10,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // TapeEcho3
    db.insert(
        0x40200110,
        EffectDef {
            id: 0x40200110,
            name: "TapeEcho3".to_string(),
            group: "DELAY".to_string(),
            title: "MAESTRO ECHOPLEX EP-3 tape echo modiling".to_string(),
            order: 713,
            install: 0,
            ver: 0x0123,
            dsp: 5.4545,
            dsp_max: 23.0 / 125.0,
            dsp_min: 18.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "F.B".to_string(),
                    def: 20,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TIME".to_string(),
                    def: 350,
                    max: 990,
                    disp: ParamDisp::None, // JS had disp:10
                },
                ParamDef {
                    name: "RecLv".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "SYNC".to_string(),
                    def: 0,
                    max: 15,
                    disp: labels(&[
                        "OFF", "1/16", "1/12", "3/32", "1/8", "1/6", "3/16", "1/4", "3/8", "1/2",
                        "3/4", "4/4", "5/4", "6/4", "7/4", "8/4",
                    ]),
                },
                ParamDef {
                    name: "P-Amp".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // DualDigiD
    db.insert(
        0x40400110,
        EffectDef {
            id: 0x40400110,
            name: "DualDigiD".to_string(),
            group: "DELAY".to_string(),
            title: "Eventide TimeFactor DigitalDelay like cobination of 2 delays".to_string(),
            order: 714,
            install: 0,
            ver: 0x0123,
            dsp: 4.4444,
            dsp_max: 1.0 / 4.0,
            dsp_min: 22.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "TimeA".to_string(),
                    def: 500,
                    max: 2005,
                    disp: time_display(
                        0,
                        1991,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "TimeB".to_string(),
                    def: 375,
                    max: 2005,
                    disp: time_display(
                        0,
                        1991,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "FdbkA".to_string(),
                    def: 50,
                    max: 110,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FdbkB".to_string(),
                    def: 50,
                    max: 110,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 0,
                    max: 101,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Speed".to_string(),
                    def: 25,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FLTR".to_string(),
                    def: 100,
                    max: 200,
                    disp: offset(-100),
                },
                ParamDef {
                    name: "DlyMx".to_string(),
                    def: 25,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // CarbonDly
    db.insert(
        0x40600110,
        EffectDef {
            id: 0x40600110,
            name: "CarbonDly".to_string(),
            group: "DELAY".to_string(),
            title: "MXR Carbon Copy analog delay modeling".to_string(),
            order: 715,
            install: 0,
            ver: 0x0123,
            dsp: 4.4444,
            dsp_max: 1.0 / 4.0,
            dsp_min: 11.0 / 48.0,
            params: vec![
                ParamDef {
                    name: "DELAY".to_string(),
                    def: 387,
                    max: 562,
                    disp: ParamDisp::None, // JS had disp:19
                },
                ParamDef {
                    name: "REGEN".to_string(),
                    def: 47,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 69,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MID".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "WIDTH".to_string(),
                    def: 31,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "SPEED".to_string(),
                    def: 28,
                    max: 50,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Sync".to_string(),
                    def: 0,
                    max: 15,
                    disp: labels(&[
                        "OFF", "1/16", "1/12", "3/32", "1/8", "1/6", "3/16", "1/4", "3/8", "1/2",
                        "3/4", "4/4", "5/4", "6/4", "7/4", "8/4",
                    ]),
                },
            ],
        },
    );

    // DriveEcho
    db.insert(
        0x00000210,
        EffectDef {
            id: 0x00000210,
            name: "DriveEcho".to_string(),
            group: "DELAY".to_string(),
            title: "LINE6 M9 TubeEcho modeling".to_string(),
            order: 716,
            install: 0,
            ver: 0x0123,
            dsp: 1.7651,
            dsp_max: 429.0 / 750.0,
            dsp_min: 69.0 / 125.0,
            params: vec![
                ParamDef {
                    name: "DRIVE".to_string(),
                    def: 39,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TIME".to_string(),
                    def: 355,
                    max: 1985,
                    disp: time_display(
                        20,
                        1991,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "WOW".to_string(),
                    def: 25,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DRV".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["DRIV", "THRU"]),
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["MONO", "STR"]),
                },
            ],
        },
    );

    // SlapBackD
    db.insert(
        0x00200210,
        EffectDef {
            id: 0x00200210,
            name: "SlapBackD".to_string(),
            group: "DELAY".to_string(),
            title: "tc electonic FLASHBACK set for SLAP delay modeling".to_string(),
            order: 717,
            install: 0,
            ver: 0x0123,
            dsp: 5.4545,
            dsp_max: 23.0 / 125.0,
            dsp_min: 18.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "TIME".to_string(),
                    def: 98,
                    max: 300,
                    disp: time_display(1, 301, &["Sync"]),
                },
                ParamDef {
                    name: "SubDv".to_string(),
                    def: 0,
                    max: 2,
                    disp: labels(&["&#x1D15F;", "&#x1D160;.", "P-P"]),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 29,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FxLVL".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DRY".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // SmoothDly
    db.insert(
        0x00400210,
        EffectDef {
            id: 0x00400210,
            name: "SmoothDly".to_string(),
            group: "DELAY".to_string(),
            title: "BOSS DD-20 smooth mode delay modeling".to_string(),
            order: 718,
            install: 0,
            ver: 0x0123,
            dsp: 2.8111,
            dsp_max: 4.0 / 10.0,
            dsp_min: 1.0 / 3.0,
            params: vec![
                ParamDef {
                    name: "TIME".to_string(),
                    def: 322,
                    max: 3014,
                    disp: time_display(
                        1,
                        3001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 39,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "E.LVL".to_string(),
                    def: 49,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 83,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // LO-FI Dly
    db.insert(
        0x00600210,
        EffectDef {
            id: 0x00600210,
            name: "LO-FI Dly".to_string(),
            group: "DELAY".to_string(),
            title: "Strymon TIMELINE LO-FI mode deley modeling".to_string(),
            order: 719,
            install: 0,
            ver: 0x0123,
            dsp: 2.2588,
            dsp_max: 1.0 / 2.0,
            dsp_min: 9.0 / 20.0,
            params: vec![
                ParamDef {
                    name: "TIME".to_string(),
                    def: 248,
                    max: 1913,
                    disp: time_display(
                        2,
                        1901,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 25,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "SMPL".to_string(),
                    def: 5,
                    max: 14,
                    disp: labels(&[
                        "1/128", "1/64", "1/32", "1/24", "1/12", "1/10", "1/9", "1/8", "1/7",
                        "1/6", "1/5", "1/4", "1/3", "1/2", "1/1",
                    ]),
                },
                ParamDef {
                    name: "BITS".to_string(),
                    def: 10,
                    max: 20,
                    disp: labels(&[
                        "4", "4.5", "5", "5.5", "6", "6.5", "7", "7.5", "8", "9", "10", "11", "12",
                        "13", "14", "15", "16", "18", "20", "24", "32",
                    ]),
                },
                ParamDef {
                    name: "BLEND".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DAMP".to_string(),
                    def: 0,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FLT".to_string(),
                    def: 2,
                    max: 8,
                    disp: labels(&["OFF", "1", "2", "3", "4", "5", "6", "7", "8"]),
                },
                ParamDef {
                    name: "VINYL".to_string(),
                    def: 0,
                    max: 18,
                    disp: labels(&[
                        "OFF", "D:1", "D:2", "D:3", "D:4", "D:5", "D:6", "D:7", "D:8", "D:9",
                        "S:1", "S:2", "S:3", "S:4", "S:5", "S:6", "S:7", "S:8", "S:9",
                    ]),
                },
            ],
        },
    );

    // SlwAtkDly
    db.insert(
        0x40000210,
        EffectDef {
            id: 0x40000210,
            name: "SlwAtkDly".to_string(),
            group: "DELAY".to_string(),
            title: "LINE6 M9 Auto-Volume Echo delay modeling".to_string(),
            order: 720,
            install: 0,
            ver: 0x0123,
            dsp: 4.3243,
            dsp_max: 1.0 / 4.0,
            dsp_min: 22.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "TIME".to_string(),
                    def: 489,
                    max: 1914,
                    disp: time_display(
                        1,
                        1901,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 71,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 64,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DEPTH".to_string(),
                    def: 77,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "SWELL".to_string(),
                    def: 24,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["MONO", "STR"]),
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // TremDelay
    db.insert(
        0x40200210,
        EffectDef {
            id: 0x40200210,
            name: "TremDelay".to_string(),
            group: "DELAY".to_string(),
            title: "Strymon TIMELINE trem mode delay modeling".to_string(),
            order: 721,
            install: 0,
            ver: 0x0123,
            dsp: 1.9173,
            dsp_max: 3.0 / 5.0,
            dsp_min: 1.0 / 2.0,
            params: vec![
                ParamDef {
                    name: "TIME".to_string(),
                    def: 300,
                    max: 1855,
                    disp: time_display(
                        60,
                        1901,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LFO".to_string(),
                    def: 0,
                    max: 4,
                    disp: labels(&["TRI", "SQR", "SIN", "RAMP", "SAW"]),
                },
                ParamDef {
                    name: "DEPTH".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "SPEED".to_string(),
                    def: 19,
                    max: 34,
                    disp: labels(&[
                        "1/32", "1/24", "1/18", "1/16", "1/12", "1/10", "1/9", "1/8", "1/7", "1/6",
                        "1/5", "1/4", "1/3", "1/2", "2/3", "3/4", "1/1", "4/3", "3/2", "2/1",
                        "5/2", "3/1", "7/2", "4/1", "5/1", "6/1", "7/1", "8/1", "9/1", "10/1",
                        "12/1", "16/1", "18/1", "24/1", "32/1",
                    ]),
                },
                ParamDef {
                    name: "DAMP".to_string(),
                    def: 4,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HPF".to_string(),
                    def: 4,
                    max: 20,
                    disp: labels(&[
                        "OFF", "20", "40", "60", "80", "100", "120", "140", "160", "180", "200",
                        "230", "260", "300", "350", "400", "500", "600", "700", "800", "900",
                    ]),
                },
                ParamDef {
                    name: "GRIT".to_string(),
                    def: 2,
                    max: 10,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // FLTR PPD
    db.insert(
        0x40400210,
        EffectDef {
            id: 0x40400210,
            name: "FLTR PPD".to_string(),
            group: "DELAY".to_string(),
            title: "Eventide TimeFactor FilterPong delay modeling".to_string(),
            order: 722,
            install: 0,
            ver: 0x0123,
            dsp: 3.4286,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "TimeA".to_string(),
                    def: 500,
                    max: 1915,
                    disp: time_display(
                        0,
                        1901,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "TimeB".to_string(),
                    def: 250,
                    max: 1915,
                    disp: time_display(
                        0,
                        1901,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DlyMx".to_string(),
                    def: 25,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Fdbk".to_string(),
                    def: 75,
                    max: 110,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Slur".to_string(),
                    def: 3,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FLTR".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 8,
                    max: 21,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Wave".to_string(),
                    def: 35,
                    max: 47,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // A-Pan DLY
    db.insert(
        0x40600210,
        EffectDef {
            id: 0x40600210,
            name: "A-Pan DLY".to_string(),
            group: "DELAY".to_string(),
            title: "Combination of autopan and delay".to_string(),
            order: 723,
            install: 0,
            ver: 0x0123,
            dsp: 5.8065,
            dsp_max: 924.0 / 5400.0,
            dsp_min: 1.0 / 6.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 222,
                    max: 2014,
                    disp: time_display(
                        1,
                        2001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 87,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 53,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 26,
                    max: 52,
                    disp: offset(-2),
                },
                ParamDef {
                    name: "Width".to_string(),
                    def: 0,
                    max: 100,
                    disp: offset(-50),
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Clip".to_string(),
                    def: 0,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Link".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["P-D", "D-P"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 200,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // ICE Delay
    db.insert(
        0x00000310,
        EffectDef {
            id: 0x00000310,
            name: "ICE Delay".to_string(),
            group: "DELAY".to_string(),
            title: "Strymon TIMELINE ICE mode pitch shifting delay modeling".to_string(),
            order: 724,
            install: 0,
            ver: 0x0123,
            dsp: 2.9268,
            dsp_max: 4.0 / 10.0,
            dsp_min: 1.0 / 3.0,
            params: vec![
                ParamDef {
                    name: "TIME".to_string(),
                    def: 440,
                    max: 1255,
                    disp: time_display(
                        60,
                        1301,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                            "&#x1D15F; x5",
                            "&#x1D15F; x6",
                            "&#x1D15F; x7",
                            "&#x1D15F; x8",
                        ],
                    ),
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 64,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "INTVL".to_string(),
                    def: 23,
                    max: 30,
                    disp: labels(&[
                        "-Oct", "-M7", "-m7", "-M6", "-m6", "-P5", "-Tri", "-P4", "-M3", "-m3",
                        "-M2", "-m2", "-50c", "-25c", "Uni", "+25c", "+50c", "+m2", "+M2", "+m3",
                        "+M3", "+P4", "+Tri", "+P5", "+m6", "+M6", "+m7", "+M7", "+Oct", "Oc+5",
                        "2Oct",
                    ]),
                },
                ParamDef {
                    name: "SLICE".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["SHORT", "LONG"]),
                },
                ParamDef {
                    name: "BLEND".to_string(),
                    def: 12,
                    max: 20,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "SMEAR".to_string(),
                    def: 7,
                    max: 20,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DAMP".to_string(),
                    def: 2,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HPF".to_string(),
                    def: 1,
                    max: 20,
                    disp: labels(&[
                        "OFF", "20", "40", "60", "80", "100", "120", "140", "160", "180", "200",
                        "230", "260", "300", "350", "400", "500", "600", "700", "800", "900",
                    ]),
                },
            ],
        },
    );

    // REVERB Section - HD Hall
    db.insert(
        0x00100012,
        EffectDef {
            id: 0x00100012,
            name: "HD Hall".to_string(),
            group: "REVERB".to_string(),
            title: "Dense hall reverb".to_string(),
            order: 800,
            install: 0,
            ver: 0x0111,
            dsp: 2.3356,
            dsp_max: 9.0 / 20.0,
            dsp_min: 3.0 / 8.0,
            params: vec![
                ParamDef {
                    name: "PreD".to_string(),
                    def: 80,
                    max: 199,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Decay".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 62,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LoDMP".to_string(),
                    def: 32,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // HD Reverb
    db.insert(
        0x00180012,
        EffectDef {
            id: 0x00180012,
            name: "HD Reverb".to_string(),
            group: "REVERB".to_string(),
            title: "High definition reverb".to_string(),
            order: 801,
            install: 0,
            ver: 0x0122,
            dsp: 4.5272,
            dsp_max: 277.0 / 1200.0,
            dsp_min: 1.0 / 5.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 10,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 53,
                    max: 199,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "HPF".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Hall
    db.insert(
        0x00200012,
        EffectDef {
            id: 0x00200012,
            name: "Hall".to_string(),
            group: "REVERB".to_string(),
            title: "Concert hall simulation".to_string(),
            order: 802,
            install: 0,
            ver: 0x0121,
            dsp: 8.7273,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 9,
                    max: 29,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 5,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 48,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Room
    db.insert(
        0x00400012,
        EffectDef {
            id: 0x00400012,
            name: "Room".to_string(),
            group: "REVERB".to_string(),
            title: "A room simulation".to_string(),
            order: 803,
            install: 0,
            ver: 0x0111,
            dsp: 10.7911,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 9,
                    max: 29,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 4,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // TiledRoom
    db.insert(
        0x00600012,
        EffectDef {
            id: 0x00600012,
            name: "TiledRoom".to_string(),
            group: "REVERB".to_string(),
            title: "Tiled room simulation".to_string(),
            order: 804,
            install: 0,
            ver: 0x0122,
            dsp: 9.6000,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 19,
                    max: 29,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 4,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 9,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Spring
    db.insert(
        0x40000012,
        EffectDef {
            id: 0x40000012,
            name: "Spring".to_string(),
            group: "REVERB".to_string(),
            title: "Spring reverb simulation".to_string(),
            order: 805,
            install: 0,
            ver: 0x0121,
            dsp: 8.8933,
            dsp_max: 1.0 / 6.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 19,
                    max: 29,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 0,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Arena
    db.insert(
        0x40200012,
        EffectDef {
            id: 0x40200012,
            name: "Arena".to_string(),
            group: "REVERB".to_string(),
            title: "Sports arena like large enclosure simulation".to_string(),
            order: 806,
            install: 0,
            ver: 0x0122,
            dsp: 10.7911,
            dsp_max: 1.0 / 10.0,
            dsp_min: 1.0 / 15.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 14,
                    max: 29,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 7,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 56,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 89,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // EarlyRef
    db.insert(
        0x40400012,
        EffectDef {
            id: 0x40400012,
            name: "EarlyRef".to_string(),
            group: "REVERB".to_string(),
            title: "Only the early reflections of reverb".to_string(),
            order: 807,
            install: 0,
            ver: 0x0122,
            dsp: 7.4071,
            dsp_max: 1.0 / 6.0,
            dsp_min: 16.0 / 100.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 14,
                    max: 29,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Shape".to_string(),
                    def: 20,
                    max: 20,
                    disp: offset(-10),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Air
    db.insert(
        0x40600012,
        EffectDef {
            id: 0x40600012,
            name: "Air".to_string(),
            group: "REVERB".to_string(),
            title: "A room ambience with spatial depth".to_string(),
            order: 808,
            install: 0,
            ver: 0x0122,
            dsp: 15.1257,
            dsp_max: 1.0 / 12.0,
            dsp_min: 2.0 / 25.0,
            params: vec![
                ParamDef {
                    name: "Size".to_string(),
                    def: 19,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 8,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Ref".to_string(),
                    def: 5,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Plate
    db.insert(
        0x00200112,
        EffectDef {
            id: 0x00200112,
            name: "Plate".to_string(),
            group: "REVERB".to_string(),
            title: "Plate reverb simulation".to_string(),
            order: 809,
            install: 0,
            ver: 0x0113,
            dsp: 3.4565,
            dsp_max: 28.0 / 100.0,
            dsp_min: 10.0 / 36.0,
            params: vec![
                ParamDef {
                    name: "PreD".to_string(),
                    def: 8,
                    max: 199,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Decay".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 44,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Color".to_string(),
                    def: 58,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LoDMP".to_string(),
                    def: 97,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 95,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // ModReverb
    db.insert(
        0x00400112,
        EffectDef {
            id: 0x00400112,
            name: "ModReverb".to_string(),
            group: "REVERB".to_string(),
            title: "Fluctuating echoes".to_string(),
            order: 810,
            install: 0,
            ver: 0x0113,
            dsp: 4.4893,
            dsp_max: 1.0 / 4.0,
            dsp_min: 1.0 / 5.0,
            params: vec![
                ParamDef {
                    name: "Depth".to_string(),
                    def: 38,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Decay".to_string(),
                    def: 19,
                    max: 29,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Rate".to_string(),
                    def: 19,
                    max: 49,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 6,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 29,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // SlapBack
    db.insert(
        0x00600112,
        EffectDef {
            id: 0x00600112,
            name: "SlapBack".to_string(),
            group: "REVERB".to_string(),
            title: "Reverb with repeating echo".to_string(),
            order: 811,
            install: 0,
            ver: 0x0113,
            dsp: 4.6142,
            dsp_max: 1108.0 / 4800.0,
            dsp_min: 1.0 / 5.0,
            params: vec![
                ParamDef {
                    name: "Time".to_string(),
                    def: 379,
                    max: 1010,
                    disp: time_display(
                        1,
                        1001,
                        &[
                            "&#x1D161;",
                            "&#x1D15F; 3",
                            "&#x1D161;.",
                            "&#x1D160;",
                            "&#x1D15E; 3",
                            "&#x1D160;.",
                            "&#x1D15F;",
                            "&#x1D15F;.",
                            "&#x1D15F; x2",
                            "&#x1D15F; x3",
                            "&#x1D15F; x4",
                        ],
                    ),
                },
                ParamDef {
                    name: "Decay".to_string(),
                    def: 9,
                    max: 29,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 43,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 10,
                    max: 10,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DRBal".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Spring63
    db.insert(
        0x40000112,
        EffectDef {
            id: 0x40000112,
            name: "Spring63".to_string(),
            group: "REVERB".to_string(),
            title: "Fender Reverb ('63) spring reverb modeling".to_string(),
            order: 812,
            install: 0,
            ver: 0x0123,
            dsp: 2.6737,
            dsp_max: 4.0 / 10.0,
            dsp_min: 1.0 / 3.0,
            params: vec![
                ParamDef {
                    name: "DWELL".to_string(),
                    def: 35,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIXER".to_string(),
                    def: 51,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 58,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LEVEL".to_string(),
                    def: 100,
                    max: 150,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // Chamber
    db.insert(
        0x40200112,
        EffectDef {
            id: 0x40200112,
            name: "Chamber".to_string(),
            group: "REVERB".to_string(),
            title: "Chamber room simulation".to_string(),
            order: 813,
            install: 0,
            ver: 0x0123,
            dsp: 2.8535,
            dsp_max: 4.0 / 10.0,
            dsp_min: 1.0 / 3.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 73,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 48,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 24,
                    max: 200,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // LOFI Rev
    db.insert(
        0x40400112,
        EffectDef {
            id: 0x40400112,
            name: "LOFI Rev".to_string(),
            group: "REVERB".to_string(),
            title: "tc electronic HALL OF FAME lofi setting modeling".to_string(),
            order: 814,
            install: 0,
            ver: 0x0123,
            dsp: 2.3731,
            dsp_max: 9.0 / 20.0,
            dsp_min: 3.0 / 8.0,
            params: vec![
                ParamDef {
                    name: "DECAY".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 95,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "FxLVL".to_string(),
                    def: 44,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["SHORT", "LONG"]),
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Church
    db.insert(
        0x40600112,
        EffectDef {
            id: 0x40600112,
            name: "Church".to_string(),
            group: "REVERB".to_string(),
            title: "Reverbrations of a church simulation".to_string(),
            order: 815,
            install: 0,
            ver: 0x0123,
            dsp: 2.4828,
            dsp_max: 5.0 / 12.0,
            dsp_min: 3.0 / 8.0,
            params: vec![
                ParamDef {
                    name: "DECAY".to_string(),
                    def: 49,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 96,
                    max: 200,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 46,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 61,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 83,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Cave
    db.insert(
        0x00000212,
        EffectDef {
            id: 0x00000212,
            name: "Cave".to_string(),
            group: "REVERB".to_string(),
            title: "Reverbrations of a cave simulation".to_string(),
            order: 816,
            install: 0,
            ver: 0x0123,
            dsp: 3.0968,
            dsp_max: 1.0 / 3.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 52,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 54,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 62,
                    max: 200,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Ambience
    db.insert(
        0x00200212,
        EffectDef {
            id: 0x00200212,
            name: "Ambience".to_string(),
            group: "REVERB".to_string(),
            title: "Natural ambience reverb".to_string(),
            order: 817,
            install: 0,
            ver: 0x0123,
            dsp: 2.4175,
            dsp_max: 41.0 / 100.0,
            dsp_min: 7.0 / 18.0,
            params: vec![
                ParamDef {
                    name: "DECAY".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 29,
                    max: 200,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // GateRev
    db.insert(
        0x00400212,
        EffectDef {
            id: 0x00400212,
            name: "GateRev".to_string(),
            group: "REVERB".to_string(),
            title: "DigiTech RV-7(Lexicon) Gated setting modeling".to_string(),
            order: 818,
            install: 0,
            ver: 0x0123,
            dsp: 2.7079,
            dsp_max: 4.0 / 10.0,
            dsp_min: 1.0 / 3.0,
            params: vec![
                ParamDef {
                    name: "Level".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Decay".to_string(),
                    def: 55,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 1,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // ReverseRv
    db.insert(
        0x00600212,
        EffectDef {
            id: 0x00600212,
            name: "ReverseRv".to_string(),
            group: "REVERB".to_string(),
            title: "DigiTech RV-7(Lexicon) Reverse setting modeling".to_string(),
            order: 819,
            install: 0,
            ver: 0x0123,
            dsp: 3.4286,
            dsp_max: 28.0 / 100.0,
            dsp_min: 1.0 / 4.0,
            params: vec![
                ParamDef {
                    name: "Level".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tone".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Decay".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Dry".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // Echo
    db.insert(
        0x40000212,
        EffectDef {
            id: 0x40000212,
            name: "Echo".to_string(),
            group: "REVERB".to_string(),
            title: "Gorgeous echoes".to_string(),
            order: 820,
            install: 0,
            ver: 0x0123,
            dsp: 2.8805,
            dsp_max: 4.0 / 10.0,
            dsp_min: 1.0 / 3.0,
            params: vec![
                ParamDef {
                    name: "DECAY".to_string(),
                    def: 25,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TIME".to_string(),
                    def: 125,
                    max: 200,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "TONE".to_string(),
                    def: 70,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 80,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Mode".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // TremoloRv
    db.insert(
        0x40200212,
        EffectDef {
            id: 0x40200212,
            name: "TremoloRv".to_string(),
            group: "REVERB".to_string(),
            title: "EvenTide SPACE tremolo verb like reverb".to_string(),
            order: 821,
            install: 0,
            ver: 0x0123,
            dsp: 2.2236,
            dsp_max: 1.0 / 2.0,
            dsp_min: 3.0 / 8.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PDLY".to_string(),
                    def: 70,
                    max: 500,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Speed".to_string(),
                    def: 28,
                    max: 346,
                    disp: ParamDisp::None, // JS had disp:10
                },
                ParamDef {
                    name: "Shape".to_string(),
                    def: 0,
                    max: 5,
                    disp: labels(&["SINE", "TRI", "PEAK", "RNDM", "RAMP", "SQR"]),
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 199,
                    max: 200,
                    disp: ParamDisp::None, // JS had disp:0
                },
                ParamDef {
                    name: "Size".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Low".to_string(),
                    def: 100,
                    max: 200,
                    disp: offset(-100),
                },
                ParamDef {
                    name: "High".to_string(),
                    def: 100,
                    max: 200,
                    disp: offset(-100),
                },
            ],
        },
    );

    // HolyFLERB
    db.insert(
        0x40400212,
        EffectDef {
            id: 0x40400212,
            name: "HolyFLERB".to_string(),
            group: "REVERB".to_string(),
            title: "Electro-Harmonix Holy Grail FLERB reverb/flanger modeling".to_string(),
            order: 822,
            install: 0,
            ver: 0x0123,
            dsp: 2.4242,
            dsp_max: 1.0 / 2.0,
            dsp_min: 3.0 / 8.0,
            params: vec![
                ParamDef {
                    name: "RVRB".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // DynaRev
    db.insert(
        0x40600212,
        EffectDef {
            id: 0x40600212,
            name: "DynaRev".to_string(),
            group: "REVERB".to_string(),
            title: "tc electronic NOVA REVERB dynamics changing reverb modeling".to_string(),
            order: 823,
            install: 0,
            ver: 0x0123,
            dsp: 2.5714,
            dsp_max: 41.0 / 100.0,
            dsp_min: 3.0 / 8.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 82,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PreD".to_string(),
                    def: 0,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Color".to_string(),
                    def: 76,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Sense".to_string(),
                    def: 172,
                    max: 200,
                    disp: offset(-100),
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // ShimmerRv
    db.insert(
        0x00000312,
        EffectDef {
            id: 0x00000312,
            name: "ShimmerRv".to_string(),
            group: "REVERB".to_string(),
            title: "Strymon blueSky shimmer mode pitch-shifting delay/reverb modeling".to_string(),
            order: 824,
            install: 0,
            ver: 0x0103,
            dsp: 1.8605,
            dsp_max: 3.0 / 5.0,
            dsp_min: 133.0 / 250.0,
            params: vec![
                ParamDef {
                    name: "PreD".to_string(),
                    def: 39,
                    max: 99,
                    disp: ParamDisp::None, // JS had disp:1
                },
                ParamDef {
                    name: "DECAY".to_string(),
                    def: 90,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "LoDMP".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "HiDMP".to_string(),
                    def: 74,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // ParticleR
    db.insert(
        0x00200312,
        EffectDef {
            id: 0x00200312,
            name: "ParticleR".to_string(),
            group: "REVERB".to_string(),
            title: "LINE6 M9 Particle Verb complex reverb modeling".to_string(),
            order: 825,
            install: 0,
            ver: 0x0103,
            dsp: 1.7804,
            dsp_max: 3.0 / 5.0,
            dsp_min: 1.0 / 2.0,
            params: vec![
                ParamDef {
                    name: "DWELL".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "GAIN".to_string(),
                    def: 100,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MODE".to_string(),
                    def: 0,
                    max: 2,
                    disp: labels(&["STBL", "CRTCL", "HZD"]),
                },
                ParamDef {
                    name: "MIX".to_string(),
                    def: 60,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "MONO".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
                ParamDef {
                    name: "Tail".to_string(),
                    def: 0,
                    max: 1,
                    disp: labels(&["OFF", "ON"]),
                },
            ],
        },
    );

    // SpaceHole
    db.insert(
        0x00400312,
        EffectDef {
            id: 0x00400312,
            name: "SpaceHole".to_string(),
            group: "REVERB".to_string(),
            title: "Eventide SPACE BlackHole delay/reverb modeling".to_string(),
            order: 826,
            install: 0,
            ver: 0x0103,
            dsp: 2.2599,
            dsp_max: 19.0 / 40.0,
            dsp_min: 5.0 / 12.0,
            params: vec![
                ParamDef {
                    name: "Decay".to_string(),
                    def: 50,
                    max: 200,
                    disp: offset(-100),
                },
                ParamDef {
                    name: "PDLY".to_string(),
                    def: 80,
                    max: 1000,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 40,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "F.B".to_string(),
                    def: 45,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Depth".to_string(),
                    def: 58,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Speed".to_string(),
                    def: 39,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Size".to_string(),
                    def: 29,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Low".to_string(),
                    def: 87,
                    max: 200,
                    disp: offset(-100),
                },
                ParamDef {
                    name: "High".to_string(),
                    def: 82,
                    max: 200,
                    disp: offset(-100),
                },
            ],
        },
    );

    // MangledSp
    db.insert(
        0x00600312,
        EffectDef {
            id: 0x00600312,
            name: "MangledSp".to_string(),
            group: "REVERB".to_string(),
            title: "Eventide SPACE MangledVerb like wild echoes".to_string(),
            order: 827,
            install: 0,
            ver: 0x0103,
            dsp: 1.8983,
            dsp_max: 3.0 / 5.0,
            dsp_min: 1.0 / 2.0,
            params: vec![
                ParamDef {
                    name: "PDLY".to_string(),
                    def: 80,
                    max: 500,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Clip".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 38,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Decay".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mod".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Size".to_string(),
                    def: 50,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Low".to_string(),
                    def: 97,
                    max: 200,
                    disp: offset(-100),
                },
                ParamDef {
                    name: "High".to_string(),
                    def: 101,
                    max: 200,
                    disp: offset(-100),
                },
                ParamDef {
                    name: "Level".to_string(),
                    def: 85,
                    max: 200,
                    disp: ParamDisp::None,
                },
            ],
        },
    );

    // DualRev
    db.insert(
        0x40000312,
        EffectDef {
            id: 0x40000312,
            name: "DualRev".to_string(),
            group: "REVERB".to_string(),
            title: "Eventide SPACE DualVerb like Combination of two reverbs".to_string(),
            order: 828,
            install: 0,
            ver: 0x0103,
            dsp: 2.1132,
            dsp_max: 1.0 / 2.0,
            dsp_min: 3.0 / 8.0,
            params: vec![
                ParamDef {
                    name: "PDlyA".to_string(),
                    def: 350,
                    max: 750,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "PDlyB".to_string(),
                    def: 700,
                    max: 750,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Mix".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "ABMix".to_string(),
                    def: 14,
                    max: 18,
                    disp: labels(&[
                        "A9B0", "A9B1", "A9B2", "A9B3", "A9B4", "A9B5", "A9B6", "A9B7", "A9B8",
                        "A9B9", "A8B9", "A7B9", "A6B9", "A5B9", "A4B9", "A3B9", "A2B9", "A1B9",
                        "A0B9",
                    ]),
                },
                ParamDef {
                    name: "DCY A".to_string(),
                    def: 15,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "DCY B".to_string(),
                    def: 30,
                    max: 100,
                    disp: ParamDisp::None,
                },
                ParamDef {
                    name: "Size".to_string(),
                    def: 21,
                    max: 32,
                    disp: labels(&[
                        "A1B1", "A2B1", "A3B1", "A4B1", "A5B1", "A6B1", "A7B1", "A8B1", "A9B1",
                        "A9B2", "A9B3", "A9B4", "A9B5", "A9B6", "A9B7", "A9B8", "A9B9", "A8B9",
                        "A7B9", "A6B9", "A5B9", "A4B9", "A3B9", "A2B9", "A1B9", "A1B8", "A1B7",
                        "A1B6", "A1B5", "A1B4", "A1B3", "A1B2", "A1B1",
                    ]),
                },
                ParamDef {
                    name: "ToneA".to_string(),
                    def: 100,
                    max: 200,
                    disp: offset(-100),
                },
                ParamDef {
                    name: "ToneB".to_string(),
                    def: 70,
                    max: 200,
                    disp: offset(-100),
                },
            ],
        },
    );

    db
}
