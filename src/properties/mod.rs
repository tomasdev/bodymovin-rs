mod double_keyframe;
mod gradient;
mod multi_dimensional;
mod scalar;
mod shape;

pub use self::{double_keyframe::*, gradient::*, multi_dimensional::*, scalar::*, shape::*};
use serde::{de::Deserializer, Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Destructurer {
    Bare(f64),
    Tuple((f64,)),
    Array([f64; 1]),
}

impl Into<f64> for Destructurer {
    fn into(self) -> f64 {
        match self {
            Self::Bare(value) | Self::Tuple((value,)) | Self::Array([value]) => value,
        }
    }
}

fn destructure<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Destructurer::deserialize(deserializer).map(Into::into)
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ControlPoint2d {
    #[serde(deserialize_with = "destructure")]
    pub x: f64,
    #[serde(deserialize_with = "destructure")]
    pub y: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControlPoint3d {
    pub x: [f64; 3],
    pub y: [f64; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bezier2d {
    #[serde(rename = "i")]
    pub in_value: ControlPoint2d,
    #[serde(rename = "o")]
    pub out_value: ControlPoint2d,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bezier3d {
    #[serde(rename = "i")]
    pub in_value: ControlPoint3d,
    #[serde(rename = "o")]
    pub out_value: ControlPoint3d,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BezierEase {
    _2D(Bezier2d),
    _3D(Bezier3d),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpatialBezier {
    #[serde(rename = "ti")]
    pub in_value: Vec<f64>,
    #[serde(rename = "to")]
    pub out_value: Vec<f64>,
}

impl SpatialBezier {
    pub fn new(in_value: Vec<f64>, out_value: Vec<f64>) -> Self {
        assert_eq!(
            in_value.len(),
            out_value.len(),
            "Attempted to construct Bezier spline from points with different dimensionalities."
        );
        Self {
            in_value,
            out_value,
        }
    }
    pub fn scaled(&self, scale: &Vec<f64>) -> Self {
        Self {
            in_value: self
                .in_value
                .iter()
                .zip(scale.iter())
                .map(|(val, scale)| val * scale)
                .collect(),
            out_value: self
                .out_value
                .iter()
                .zip(scale.iter())
                .map(|(val, scale)| val * scale)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value<F, A> {
    Fixed(F),
    Animated(Vec<A>),
}

impl<F, A> Default for Value<F, A>
where
    F: Default,
{
    fn default() -> Self {
        Self::Fixed(F::default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Property<F, A> {
    #[serde(rename = "a")]
    pub animated: i8,

    #[serde(rename = "k")]
    pub value: Value<F, A>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "x")]
    pub expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ix")]
    pub index: Option<i64>,
}

impl<F, A> Default for Property<F, A>
where
    F: Default,
{
    fn default() -> Self {
        Self {
            animated: 0,
            value: Value::default(),
            expression: None,
            index: None,
        }
    }
}

impl<F, A> Property<F, A> {
    pub(crate) fn fixed(value: F) -> Self {
        Self {
            animated: 0,
            value: Value::Fixed(value),
            expression: None,
            index: None,
        }
    }
}
