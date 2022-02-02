/// Hazards type.
pub enum Hazards {
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

/// Categories.
pub enum Categories {
    /// Category which includes all the financial-related hazards.
    Financial,
    /// Category which includes all the privacy-related hazards.
    Privacy,
    /// Category which includes all the safety-related hazards.
    Safety,
}