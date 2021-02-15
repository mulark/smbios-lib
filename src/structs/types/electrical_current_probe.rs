use crate::*;

/// # Electrical Current Probe (Type 29)
///
/// This structure describes the attributes for an electrical current probe in the system. Each structure describes a single electrical current probe.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosElectricalCurrentProbe<'a> {
    parts: &'a UndefinedStruct,
}

impl<'a> SMBiosStruct<'a> for SMBiosElectricalCurrentProbe<'a> {
    const STRUCT_TYPE: u8 = 29u8;

    fn new(parts: &'a UndefinedStruct) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a UndefinedStruct {
        self.parts
    }
}

impl<'a> SMBiosElectricalCurrentProbe<'a> {
    ///  A string that contains additional descriptive information about the probe or its location
    pub fn description(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Probe’s physical location and status of the current monitored by this current probe
    pub fn location_and_status(&self) -> Option<CurrentProbeLocationAndStatus> {
        self.parts
            .get_field_byte(0x05)
            .and_then(|raw| Some(CurrentProbeLocationAndStatus::from(raw)))
    }

    /// Maximum current level readable by this probe, in milliamps
    pub fn maximum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x06)
    }

    /// Minimum temperature level readable by this probe, in milliamps
    pub fn minimum_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x08)
    }

    /// Resolution for the probe’s reading, in tenths of milliamps
    pub fn resolution(&self) -> Option<u16> {
        self.parts.get_field_word(0x0A)
    }

    /// Tolerance for reading from this probe, in plus/minus milliamps
    pub fn tolerance(&self) -> Option<u16> {
        self.parts.get_field_word(0x0C)
    }

    /// Accuracy for reading from this probe, in plus/minus 1/100th of a percent
    pub fn accuracy(&self) -> Option<u16> {
        self.parts.get_field_word(0x0E)
    }

    /// OEM- or BIOS vendor-specific information.
    pub fn oem_defined(&self) -> Option<u32> {
        self.parts.get_field_dword(0x10)
    }

    /// Nominal value for the probe’s reading in milliamps
    pub fn nominal_value(&self) -> Option<u16> {
        self.parts.get_field_word(0x14)
    }
}

impl fmt::Debug for SMBiosElectricalCurrentProbe<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosElectricalCurrentProbe<'_>>())
            .field("header", &self.parts.header)
            .field("description", &self.description())
            .field("location_and_status", &self.location_and_status())
            .field("maximum_value", &self.maximum_value())
            .field("minimum_value", &self.minimum_value())
            .field("resolution", &self.resolution())
            .field("tolerance", &self.tolerance())
            .field("accuracy", &self.accuracy())
            .field("oem_defined", &self.oem_defined())
            .field("nominal_value", &self.nominal_value())
            .finish()
    }
}

/// # Electrical Current Probe Location and Status
#[derive(PartialEq, Eq)]
pub struct CurrentProbeLocationAndStatus {
    /// Raw value
    ///
    /// _raw_ is most useful when _value_ is None.
    /// This is most likely to occur when the standard was updated but
    /// this library code has not been updated to match the current
    /// standard.
    pub raw: u8,
    /// The [CurrentProbeStatus]
    pub status: CurrentProbeStatus,
    /// The [CurrentProbeLocation]
    pub location: CurrentProbeLocation,
}

impl fmt::Debug for CurrentProbeLocationAndStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<CurrentProbeLocationAndStatus>())
            .field("raw", &self.raw)
            .field("status", &self.status)
            .field("location", &self.location)
            .finish()
    }
}

/// # Electrical Current Probe Status
#[derive(Debug, PartialEq, Eq)]
pub enum CurrentProbeStatus {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// OK
    OK,
    /// Non-critical
    NonCritical,
    /// Critical
    Critical,
    /// Non-recoverable
    NonRecoverable,
    /// A value unknown to this standard, check the raw value
    None,
}

/// # Electrical Current Probe Location
#[derive(Debug, PartialEq, Eq)]
pub enum CurrentProbeLocation {
    /// Other
    Other,
    /// Unknown
    Unknown,
    /// Processor
    Processor,
    /// Disk
    Disk,
    /// Peripheral Bay
    PeripheralBay,
    /// System Management Module
    SystemManagementModule,
    /// Motherboard
    Motherboard,
    /// Memory Module
    MemoryModule,
    /// Processor Module
    ProcessorModule,
    /// Power Unit
    PowerUnit,
    /// Add-in Card
    AddInCard,
    /// A value unknown to this standard, check the raw value
    None,
}

impl From<u8> for CurrentProbeLocationAndStatus {
    fn from(raw: u8) -> Self {
        CurrentProbeLocationAndStatus {
            status: match raw & 0b111_00000 {
                0b001_00000 => CurrentProbeStatus::Other,
                0b010_00000 => CurrentProbeStatus::Unknown,
                0b011_00000 => CurrentProbeStatus::OK,
                0b100_00000 => CurrentProbeStatus::NonCritical,
                0b101_00000 => CurrentProbeStatus::Critical,
                0b110_00000 => CurrentProbeStatus::NonRecoverable,
                _ => CurrentProbeStatus::None,
            },
            location: match raw & 0b000_11111 {
                0b000_00001 => CurrentProbeLocation::Other,
                0b000_00010 => CurrentProbeLocation::Unknown,
                0b000_00011 => CurrentProbeLocation::Processor,
                0b000_00100 => CurrentProbeLocation::Disk,
                0b000_00101 => CurrentProbeLocation::PeripheralBay,
                0b000_00110 => CurrentProbeLocation::SystemManagementModule,
                0b000_00111 => CurrentProbeLocation::Motherboard,
                0b000_01000 => CurrentProbeLocation::MemoryModule,
                0b000_01001 => CurrentProbeLocation::ProcessorModule,
                0b000_01010 => CurrentProbeLocation::PowerUnit,
                0b000_01011 => CurrentProbeLocation::AddInCard,
                _ => CurrentProbeLocation::None,
            },
            raw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let struct_type29 = vec![
            0x1D, 0x16, 0x33, 0x00, 0x01, 0x67, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
            0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x41, 0x42, 0x43, 0x00, 0x00,
        ];

        let parts = UndefinedStruct::new(&struct_type29);
        let test_struct = SMBiosElectricalCurrentProbe::new(&parts);

        assert_eq!(test_struct.description(), Some("ABC".to_string()));
        let location_and_status = test_struct.location_and_status().unwrap();
        assert_eq!(location_and_status.status, CurrentProbeStatus::OK);
        assert_eq!(
            location_and_status.location,
            CurrentProbeLocation::Motherboard
        );
        assert_eq!(
            test_struct.location_and_status(),
            Some(CurrentProbeLocationAndStatus::from(103))
        );
        assert_eq!(test_struct.maximum_value(), Some(32768));
        assert_eq!(test_struct.minimum_value(), Some(32768));
        assert_eq!(test_struct.resolution(), Some(32768));
        assert_eq!(test_struct.tolerance(), Some(32768));
        assert_eq!(test_struct.accuracy(), Some(32768));
        assert_eq!(test_struct.oem_defined(), Some(0));
        assert_eq!(test_struct.nominal_value(), Some(32768));
    }
}
