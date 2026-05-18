use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Poses {
    pub frame: u32,

    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub v0: f32,

    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
    pub v1: f32,

    pub x2: f32,
    pub y2: f32,
    pub z2: f32,
    pub v2: f32,

    pub x3: f32,
    pub y3: f32,
    pub z3: f32,
    pub v3: f32,

    pub x4: f32,
    pub y4: f32,
    pub z4: f32,
    pub v4: f32,

    pub x5: f32,
    pub y5: f32,
    pub z5: f32,
    pub v5: f32,

    pub x6: f32,
    pub y6: f32,
    pub z6: f32,
    pub v6: f32,

    pub x7: f32,
    pub y7: f32,
    pub z7: f32,
    pub v7: f32,

    pub x8: f32,
    pub y8: f32,
    pub z8: f32,
    pub v8: f32,

    pub x9: f32,
    pub y9: f32,
    pub z9: f32,
    pub v9: f32,

    pub x10: f32,
    pub y10: f32,
    pub z10: f32,
    pub v10: f32,

    pub x11: f32,
    pub y11: f32,
    pub z11: f32,
    pub v11: f32,

    pub x12: f32,
    pub y12: f32,
    pub z12: f32,
    pub v12: f32,

    pub x13: f32,
    pub y13: f32,
    pub z13: f32,
    pub v13: f32,

    pub x14: f32,
    pub y14: f32,
    pub z14: f32,
    pub v14: f32,

    pub x15: f32,
    pub y15: f32,
    pub z15: f32,
    pub v15: f32,

    pub x16: f32,
    pub y16: f32,
    pub z16: f32,
    pub v16: f32,

    pub x17: f32,
    pub y17: f32,
    pub z17: f32,
    pub v17: f32,

    pub x18: f32,
    pub y18: f32,
    pub z18: f32,
    pub v18: f32,

    pub x19: f32,
    pub y19: f32,
    pub z19: f32,
    pub v19: f32,

    pub x20: f32,
    pub y20: f32,
    pub z20: f32,
    pub v20: f32,

    pub x21: f32,
    pub y21: f32,
    pub z21: f32,
    pub v21: f32,

    pub x22: f32,
    pub y22: f32,
    pub z22: f32,
    pub v22: f32,

    pub x23: f32,
    pub y23: f32,
    pub z23: f32,
    pub v23: f32,

    pub x24: f32,
    pub y24: f32,
    pub z24: f32,
    pub v24: f32,

    pub x25: f32,
    pub y25: f32,
    pub z25: f32,
    pub v25: f32,

    pub x26: f32,
    pub y26: f32,
    pub z26: f32,
    pub v26: f32,

    pub x27: f32,
    pub y27: f32,
    pub z27: f32,
    pub v27: f32,

    pub x28: f32,
    pub y28: f32,
    pub z28: f32,
    pub v28: f32,

    pub x29: f32,
    pub y29: f32,
    pub z29: f32,
    pub v29: f32,

    pub x30: f32,
    pub y30: f32,
    pub z30: f32,
    pub v30: f32,

    pub x31: f32,
    pub y31: f32,
    pub z31: f32,
    pub v31: f32,

    pub x32: f32,
    pub y32: f32,
    pub z32: f32,
    pub v32: f32,
}

pub fn read_csv(file_path: &str) -> Vec<Poses> {
    let mut rdr = csv::Reader::from_path(file_path).expect("Failed to open CSV file");
    let mut poses_data = Vec::new();

    for result in rdr.deserialize() {
        let record: Poses = result.expect("Failed to deserialize CSV record");
        poses_data.push(record);
    }

    poses_data
}
