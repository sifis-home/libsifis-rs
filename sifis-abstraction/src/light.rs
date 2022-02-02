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
        if t.attype().contains(&Light::AT_TYPE) {
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
            .properties_mut()
            .find(|p| p.1.attype().contains(&"OnOff"))
            .map(|p| p.1.set(true))
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties_mut()
            .find(|p| p.1.attype().contains(&"BrightnessProperty"))
            .map(|p| p.1.set(brightness.0))
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties_mut()
            .find(|p| p.1.attype().contains(&"ColorProperty"))
            .map(|p| p.1.set(color.to_string()))
            .ok_or(anyhow!("Error"))
    }

    /// Turns a light off.
    pub fn _turn_light_off(&mut self) -> Result<()> {
        self.0
            .properties_mut()
            .find(|p| p.1.attype().contains(&"OnOff"))
            .map(|p| p.1.set(false))
            .ok_or(anyhow!("Error"))
    }
}
