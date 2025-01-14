use crate::{helpers, properties};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stroke {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mn")]
    pub match_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nm")]
    pub name: Option<String>,
    #[serde(rename = "lc")]
    pub line_cap: helpers::LineCap,
    #[serde(rename = "lj")]
    pub line_join: helpers::LineJoin,
    #[serde(rename = "ml")]
    pub miter_limit: Option<f64>,
    #[serde(rename = "o")]
    pub opacity: properties::Scalar,
    #[serde(rename = "w")]
    pub width: properties::Scalar,
    #[serde(rename = "c")]
    pub color: properties::MultiDimensional,
}
