/// Hazards type.
pub enum Hazards {
    AirPoisoning,
    Asphyxia,
    AudioVideoRecordAndStore,
    AudioVideoStream,
    ElectricEnergyConsumption,
    Explosion,
    FireHazard,
    GasConsumption,
    LogEnergyConsumption,
    LogUsageTime,
    PaySubscriptionFee,
    PowerOutage,
    PowerSurge,
    RecordIssuedCommands,
    RecordUserPreferences,
    SpendMoney,
    SpoiledFood,
    TakeDeviceScreenshots,
    TakePictures,
    UnauthorisedPhysicalAccess,
    WaterConsumption,
    WaterFlooding,
}

/// Fire hazard.
/// The execution may cause fire.
pub fn turn_on_light() {
    todo!("Implement using the Wot API");
}

/// Fire hazard.
/// The execution may cause fire.
///
/// Audio video stream.
/// The execution authorises the app to obtain a video stream with audio.
pub fn turn_on_oven() {
    todo!("Implement using the Wot API");
}