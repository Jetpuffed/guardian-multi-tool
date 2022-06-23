use serde::{Deserialize, Serialize};

/// https://bungie-net.github.io/#/components/schemas/Interpolation.InterpolationPoint
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterpolationPoint {
    value: Option<i32>,
    weight: Option<i32>,
}

impl InterpolationPoint {
    pub fn value(&self) -> Option<i32> {
        self.value
    }

    pub fn weight(&self) -> Option<i32> {
        self.weight
    }
}

/// https://bungie-net.github.io/#/components/schemas/Interpolation.InterpolationPointFloat
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterpolationPointFloat {
    value: Option<f32>,
    weight: Option<f32>,
}

impl InterpolationPointFloat {
    pub fn value(&self) -> Option<f32> {
        self.value
    }

    pub fn weight(&self) -> Option<f32> {
        self.weight
    }
}
