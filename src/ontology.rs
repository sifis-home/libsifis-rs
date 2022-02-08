use crate::thing::Hazard as ThingHazard;

/// Hazards type.
pub enum Hazard {
    /// The execution may release toxic gases
    AirPoisoning,
    /// The execution may cause oxygen deficiency by gaseous substances
    Asphyxia,
    /// The execution authorises the app to record and save a video with audio on persistent storage
    AudioVideoRecordAndStore,
    /// The execution authorises the app to obtain a video stream with audio
    AudioVideoStream,
    /// The execution enables a device that consumes electricity
    ElectricEnergyConsumption,
    /// The execution may cause an explosion
    Explosion,
    /// The execution may cause fire
    FireHazard,
    /// The execution enables a device that consumes gas
    GasConsumption,
    /// The execution authorises the app to get and save information about the app's energy impact on the device the app runs on
    LogEnergyConsumption,
    /// The execution authorises the app to get and save information about the app's duration of use
    LogUsageTime,
    /// The execution authorises the app to use payment information and make a periodic payment
    PaySubscriptionFee,
    /// The execution may cause an interruption in the supply of electricity
    PowerOutage,
    /// The execution may lead to exposure to high voltages
    PowerSurge,
    /// The execution authorises the app to get and save user inputs
    RecordIssuedCommands,
    /// The execution authorises the app to get and save information about the user's preferences
    RecordUserPreferences,
    /// The execution authorises the app to use payment information and make a payment transaction
    SpendMoney,
    /// The execution may lead to rotten food
    SpoiledFood,
    /// The execution authorises the app to read the display output and take screenshots of it
    TakeDeviceScreenshots,
    /// The execution authorises the app to use a camera and take photos
    TakePictures,
    /// The execution disables a protection mechanism and unauthorised individuals may physically enter home
    UnauthorisedPhysicalAccess,
    /// The execution enables a device that consumes water
    WaterConsumption,
    /// The execution allows water usage which may lead to flood
    WaterFlooding,
}

impl Hazard {
    /// Returns the `Hazard` type associated to a Thing.
    ///
    /// It returns `None` if any type has been found for the given
    /// Thing.
    pub fn has_hazard(thing_hazard: &ThingHazard) -> Option<Self> {
        match thing_hazard.name.as_str() {
            "AirPoisoning" => Some(Hazard::AirPoisoning),
            "Asphyxia" => Some(Hazard::Asphyxia),
            "AudioVideoRecordAndStore" => Some(Hazard::AudioVideoRecordAndStore),
            "AudioVideoStream" => Some(Hazard::AudioVideoStream),
            "ElectricEnergyConsumption" => Some(Hazard::ElectricEnergyConsumption),
            "Explosion" => Some(Hazard::Explosion),
            "FireHazard" => Some(Hazard::FireHazard),
            "GasConsumption" => Some(Hazard::GasConsumption),
            "LogEnergyConsumption" => Some(Hazard::LogEnergyConsumption),
            "LogUsageTime" => Some(Hazard::LogUsageTime),
            "PaySubscriptionFee" => Some(Hazard::PaySubscriptionFee),
            "PowerOutage" => Some(Hazard::PowerOutage),
            "PowerSurge" => Some(Hazard::PowerSurge),
            "RecordIssuedCommands" => Some(Hazard::RecordIssuedCommands),
            "RecordUserPreferences" => Some(Hazard::RecordUserPreferences),
            "SpendMoney" => Some(Hazard::SpendMoney),
            "SpoiledFood" => Some(Hazard::SpoiledFood),
            "TakeDeviceScreenshots" => Some(Hazard::TakeDeviceScreenshots),
            "TakePictures" => Some(Hazard::TakePictures),
            "UnauthorisedPhysicalAccess" => Some(Hazard::UnauthorisedPhysicalAccess),
            "WaterConsumption" => Some(Hazard::WaterConsumption),
            "WaterFlooding" => Some(Hazard::WaterFlooding),
            _ => None,
        }
    }

    /// Returns the description associated to an `Hazard` type.
    pub fn get_description(&self) -> &str {
        match self {
                Self::AirPoisoning => "The execution may release toxic gases",
                Self::Asphyxia => "The execution may cause oxygen deficiency by gaseous substances",
                Self::AudioVideoRecordAndStore => "The execution authorises the app to record and save a video with audio on persistent storage",
                Self::AudioVideoStream => "The execution authorises the app to obtain a video stream with audio",
                Self::ElectricEnergyConsumption => "The execution enables a device that consumes electricity",
                Self::Explosion => "The execution may cause an explosion",
                Self::FireHazard => "The execution may cause fire",
                Self::GasConsumption => "The execution enables a device that consumes gas",
                Self::LogEnergyConsumption => "The execution authorises the app to get and save information about the app's energy impact on the device the app runs on",
                Self::LogUsageTime => "The execution authorises the app to get and save information about the app's duration of use",
                Self::PaySubscriptionFee => "The execution authorises the app to use payment information and make a periodic payment",
                Self::PowerOutage => "The execution may cause an interruption in the supply of electricity",
                Self::PowerSurge => "The execution may lead to exposure to high voltages",
                Self::RecordIssuedCommands => "The execution authorises the app to get and save user inputs",
                Self::RecordUserPreferences => "The execution authorises the app to get and save information about the user's preferences",
                Self::SpendMoney => "The execution authorises the app to use payment information and make a payment transaction",
                Self::SpoiledFood => "The execution may lead to rotten food",
                Self::TakeDeviceScreenshots => "The execution authorises the app to read the display output and take screenshots of it",
                Self::TakePictures => "The execution authorises the app to use a camera and take photos",
                Self::UnauthorisedPhysicalAccess => "The execution disables a protection mechanism and unauthorised individuals may physically enter home",
                Self::WaterConsumption => "The execution enables a device that consumes water",
                Self::WaterFlooding => "The execution allows water usage which may lead to flood",
        }
    }

    /// Returns the `Category`s associated to an `Hazard`.
    pub fn has_category(&self) -> Category {
        match self {
            Self::AirPoisoning => Category::Safety,
            Self::Asphyxia => Category::Safety,
            Self::AudioVideoRecordAndStore => Category::Privacy,
            Self::AudioVideoStream => Category::Privacy,
            Self::ElectricEnergyConsumption => Category::Financial,
            Self::Explosion => Category::Safety,
            Self::FireHazard => Category::Safety,
            Self::GasConsumption => Category::Financial,
            Self::LogEnergyConsumption => Category::Privacy,
            Self::LogUsageTime => Category::Privacy,
            Self::PaySubscriptionFee => Category::Financial,
            Self::PowerOutage => Category::Safety,
            Self::PowerSurge => Category::Safety,
            Self::RecordIssuedCommands => Category::Privacy,
            Self::RecordUserPreferences => Category::Privacy,
            Self::SpendMoney => Category::Financial,
            Self::SpoiledFood => Category::Safety,
            Self::TakeDeviceScreenshots => Category::Privacy,
            Self::TakePictures => Category::Privacy,
            Self::UnauthorisedPhysicalAccess => Category::Safety,
            Self::WaterConsumption => Category::Financial,
            Self::WaterFlooding => Category::Safety,
        }
    }

    /// Returns all `Hazard` types as immutable strings.
    pub fn all_hazards() -> &'static [&'static str] {
        &[
            "AirPoisoning",
            "Asphyxia",
            "AudioVideoRecordAndStore",
            "AudioVideoStream",
            "ElectricEnergyConsumption",
            "Explosion",
            "FireHazard",
            "GasConsumption",
            "LogEnergyConsumption",
            "LogUsageTime",
            "PaySubscriptionFee",
            "PowerOutage",
            "PowerSurge",
            "RecordIssuedCommands",
            "RecordUserPreferences",
            "SpendMoney",
            "SpoiledFood",
            "TakeDeviceScreenshots",
            "TakePictures",
            "UnauthorisedPhysicalAccess",
            "WaterConsumption",
            "WaterFlooding",
        ]
    }
}

/// Categories associated to an hazard.
pub enum Category {
    /// Category which includes all the financial-related hazards.
    Financial,
    /// Category which includes all the privacy-related hazards.
    Privacy,
    /// Category which includes all the safety-related hazards.
    Safety,
}

impl Category {
    /// Returns all `Hazard`s associated to a `Category`.
    pub fn all_hazards(&self) -> &[Hazard] {
        match self {
            Self::Financial => &[
                Hazard::ElectricEnergyConsumption,
                Hazard::GasConsumption,
                Hazard::PaySubscriptionFee,
                Hazard::SpendMoney,
                Hazard::WaterConsumption,
            ],
            Self::Privacy => &[
                Hazard::AudioVideoRecordAndStore,
                Hazard::AudioVideoStream,
                Hazard::LogEnergyConsumption,
                Hazard::LogUsageTime,
                Hazard::RecordIssuedCommands,
                Hazard::RecordUserPreferences,
                Hazard::TakeDeviceScreenshots,
                Hazard::TakePictures,
            ],
            Self::Safety => &[
                Hazard::AirPoisoning,
                Hazard::Asphyxia,
                Hazard::Explosion,
                Hazard::FireHazard,
                Hazard::PowerOutage,
                Hazard::PowerSurge,
                Hazard::SpoiledFood,
                Hazard::UnauthorisedPhysicalAccess,
                Hazard::WaterFlooding,
            ],
        }
    }

    /// Returns all `Category` as immutable strings.
    pub fn all_categories() -> &'static [&'static str] {
        &["Financial", "Privacy", "Safety"]
    }
}
