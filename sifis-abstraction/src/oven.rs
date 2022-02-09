use anyhow::{anyhow, Result};
use sifis::Thing;

use crate::{ConnectedObject, Percentage};

pub struct Oven(Thing);

impl ConnectedObject for Oven {
    const AT_TYPE: &'static str = "Oven";
}

impl TryFrom<Thing> for Oven {
    type Error = &'static str;

    fn try_from(t: Thing) -> Result<Self, Self::Error> {
        if t.attype.contains(&Oven::AT_TYPE.to_owned()) {
            Ok(Oven(t))
        } else {
            Err("The Thing is not a Oven!")
        }
    }
}

impl Oven {
    /// Turns an oven on.
    ///
    /// # Hazards
    ///
    /// * Fire hazard\
    ///   The execution may cause fire
    /// * Audio video stream\
    ///   The execution authorises the app to obtain a video stream with audio
    pub fn _turn_oven_on(&mut self, temperature: Percentage, enable_camera: bool) -> Result<()> {
        self.0
            .properties_mut()
            .find(|p| p.1.attype().contains(&"OnOff".to_owned()))
            .map(|p| p.1.set(true))
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties_mut()
            .find(|p| p.1.attype().contains(&"Temperature".to_owned()))
            .map(|p| p.1.set(temperature.0))
            .ok_or(anyhow!("Error"))?;
        self.0
            .properties_mut()
            .find(|p| p.1.attype().contains(&"Camera".to_owned()))
            .map(|p| p.1.set(enable_camera))
            .ok_or(anyhow!("Error"))
    }

    /// Turns an oven off.
    pub fn _turn_oven_off(&mut self) -> Result<()> {
        self.0
            .properties_mut()
            .find(|p| p.1.attype().contains(&"OnOff".to_owned()))
            .map(|p| p.1.set(false))
            .ok_or(anyhow!("Error"))
    }
}
