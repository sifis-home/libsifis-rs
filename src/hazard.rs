use serde::Deserialize;

/// SIFIS Hazard
///
/// Describes a possible hazard.
///
/// A risk score can *only* assume values in the range [0, 10].
/// Values outside of the defined range are invalid.
#[derive(Clone, Deserialize, Debug)]
pub struct Hazard {
    #[serde(rename = "@id")]
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "riskScore")]
    risk_score: Option<usize>,
}

impl Hazard {
    pub fn has_valid_risk_score(&self) -> bool {
        self.risk_score.map_or(false, |v| (0..11).contains(&v))
    }
}

pub trait WithLabel {
    fn has_label(&self, label: &Hazard) -> bool;
}
