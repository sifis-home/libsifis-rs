use anyhow::{anyhow, Result};
use sifis::Thing;

use crate::{ConnectedObject, Percentage, Rgb};

pub struct Light(Thing);

impl ConnectedObject for Light {
    const AT_TYPE: &'static str = "Light";
}

impl TryFrom<Thing> for Light {
    type Error = &'static str;

    fn try_from(t: Thing) -> Result<Self, Self::Error> {
        if t.attype.contains(&Light::AT_TYPE.to_owned()) {
            Ok(Light(t))
        } else {
            Err("The Thing is not a Light!")
        }
    }
}

impl Light {
    /// Turns a light on.
    ///
    /// # Hazards
    ///
    /// * Fire hazard\
    ///   The execution may cause fire
    pub fn _turn_light_on(&mut self, brightness: Percentage, color: Rgb) -> Result<()> {
        self.0
            .properties
            .iter_mut()
            .find(|p| p.1.attype().contains(&"OnOff".to_owned()))
            .map(|p| p.1.set(&true).ok())
            .flatten()
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties
            .iter_mut()
            .find(|p| p.1.attype().contains(&"BrightnessProperty".to_owned()))
            .map(|p| p.1.set(&brightness.0).ok())
            .flatten()
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties
            .iter_mut()
            .find(|p| p.1.attype().contains(&"ColorProperty".to_owned()))
            .map(|p| p.1.set(&color.to_string()).ok())
            .flatten()
            .ok_or(anyhow!("Error"))
    }

    /// Turns a light off.
    pub fn _turn_light_off(&mut self) -> Result<()> {
        self.0
            .properties
            .iter_mut()
            .find(|p| p.1.attype().contains(&"OnOff".to_owned()))
            .map(|p| p.1.set(&false).ok())
            .flatten()
            .ok_or(anyhow!("Error"))
    }
}
