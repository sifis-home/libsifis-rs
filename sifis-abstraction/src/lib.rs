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

/// Categories.
pub enum Categories {
    Financial,
    Privacy,
    Safety,
}

/// Percentage in the range [0, 100].
pub struct Percentage(u8);

impl Percentage {
    /// Creates a new `Percentage`.
    pub fn new(val: u8) -> Self {
        Self(val.min(100))
    }
}

/// Colour of a light.
pub struct Rgb(u8, u8, u8);

impl Rgb {
    /// Creates a new `Rgb`.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }
}

/// Turns a light on.
///
/// If the `id` parameter is missing all available lights in a room
/// will be turned on.
///
/// # Hazards
///
/// * Fire hazard\
///   The execution may cause fire
pub fn turn_light_on(id: Option<u32>, room_id: u32, intensity: Percentage, colour: Rgb) -> bool {
    todo!("Implement using the Wot API");
}

/// Turns an oven on.
///
/// # Hazards
///
/// * Fire hazard\
///   The execution may cause fire
/// * Audio video stream\
///   The execution authorises the app to obtain a video stream with audio
pub fn turn_oven_on(id: u32, temperature: Percentage, enable_camera: bool) -> bool {
    todo!("Implement using the Wot API");
}