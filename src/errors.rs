use enum_iterator::{all, Sequence};//, cardinality, first, last, next, previous, reverse_all};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UemGeneralError {
    #[error("Operation in progress")]
    OperationPending,
    #[error("Feature not supported")]
    UnsupportedFeature,
    #[error("Communication data lost")]
    CommunicationDataLost,
    #[error("Incorrect parameter")]
    IncorrectParameter,
    #[error("Unexpected error")]
    Unexpected,
    #[error("Access error")]
    Access,
    #[error("Not transacted")]
    NotTransacted,
    #[error("Incorrect reader name")]
    ReaderIncorrectName,
    #[error("Failed to connect to the reader")]
    ReaderConnectionFailed,
    #[error("Reader not connected")]
    ReaderNotConnected,
    #[error("Reader already connected")]
    ReaderAlreadyConnected,
    #[error("Incorrect reader response")]
    ReaderIncorrectResponse,
    #[error("Reader not responding")]
    ReaderResponseFailure,
    #[error("Reader returned error code")]
    ReaderUnsuccessful(UemInternalError, Option<Vec<u8>>),
    #[error("SAM: APDU error")]
    SamApdu,
    #[error("SAM: Invalid MAC")]
    SamInvalidMac,
    #[error("SAM: Authentication failed")]
    SamAuthenticationFailed,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Sequence, Clone)]
pub enum UemInternalError {
    NoTag = 0xFF,
    Crc = 0xFE,
    WrongKey = 0xFC,
    Parity = 0xFB,
    ResultCode = 0xFA,
    Protocol = 0xF9,
    SerialNumber = 0xF8,
    LoadKey = 0xF7,
    NotAuthenticated = 0xF6,
    BitCount = 0xF5,
    ByteCount = 0xF4,
    WriteData = 0xF1,
    Increment = 0xF0,
    Decrement = 0xEF,
    ReadData = 0xEE,
    Overflow = 0xED,
    Framing = 0xEB,
    UnknownOperation = 0xE9,
    Collision = 0xE8,
    Reset = 0xE7,
    Interface = 0xE6,
    NoBitwiseAnticoll = 0xE4,
    Coding = 0xE1,
    HardwareAbsent = 0xD8,
    UnknownCommand = 0xD7,
    CommandNotSupported = 0xD6,
    WrongMfrcMode = 0xD5,
    WrongCryptoMode = 0xD4,
    FlashEraseRequired = 0xD3,
    FlashKeyAbsent = 0xD2,
    Transceive = 0xD1,
    IcodeStackOverflow = 0xD0,
    HaltB = 0xCF,
    FlashOperating = 0xCE,
    InternalCall = 0xCD,
    CascadeLevel10 = 0xCC,
    BaudrateNotSupported = 0xCA,
    SamTimeout = 0xC9,
    SamApdu = 0xC8,
    SamCardMac = 0xC7,
    SamAuthentication = 0xC6,
    SamByteCount = 0xC5,
    ParameterValue = 0xC4,
    MifareClassicNacF = 0xBF,
    MifareClassicNacE = 0xBE,
    MifareClassicNacD = 0xBD,
    MifareClassicNacC = 0xBC,
    MifareClassicNacB = 0xBB,
    MifareClassicNacA = 0xBA,
    MifareClassicNac9 = 0xB9,
    MifareClassicNac8 = 0xB8,
    MifareClassicNac7 = 0xB7,
    MifareClassicNac6 = 0xB6,
    MifareClassicNac5 = 0xB5,
    MifareClassicNac4 = 0xB4,
    MifareClassicNac3 = 0xB3,
    MifareClassicNac2 = 0xB2,
    MifareClassicNac1 = 0xB1,
    MifareClassicNac0 = 0xB0,
    MifarePlusGeneralManipulate = 0xAF,
    MifarePlusCardMac = 0xAE,
    MifarePlusEv1NotSupported = 0xAD,
    MifarePlusLength = 0xAC,
    MifarePlusNoStateForCommand = 0xAB,
    MifarePlusNotExistingBlock = 0xAA,
    MifarePlusBlockNumber = 0xA9,
    MifarePlusMac = 0xA8,
    MifarePlusCommandOverflow = 0xA7,
    MifarePlusAuthentication = 0xA6,
    MifarePlusEv1Tmac = 0xA5,
    NotYetImplemented = 0x9C,
    Crc16 = 0x9B,
    ReceiveBufferOverflow = 0x90,
    InternalReaderLibrary = 0x85,
    ValueBlockFormat = 0x84,
    UnsupportedParameter = 0x83,
    IncompleteChaining = 0x82,
    Temperature = 0x81,
    Unknown = 0x80,
}

impl UemInternalError {
    pub fn from_byte(code: u8) -> Self {
        for err in all::<Self>() {
            if err.clone() as u8 == code {
                return err;
            }
        }
        Self::Unknown
    }
}