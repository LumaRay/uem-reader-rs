//! Crate error types

use enum_iterator::{all, Sequence};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
/// General errors for library methods
pub enum UemError {
    #[error("Operation in progress")]
    /// Indicates that current operation
    /// is still in progress
    PendingOperation,
    #[error("Feature not supported")]
    /// Indicates that the requested feature
    /// is not supported yet
    UnsupportedFeature,
    #[error("Communication data lost")]
    /// Indicates that communication channel
    /// has detected a problem with
    /// data consistency
    LostCommunicationData,
    #[error("Incorrect parameter")]
    /// A method parameter has incorrect value
    IncorrectParameter,
    #[error("Unexpected error")]
    /// Some unexpected error occured
    Unexpected,
    #[error("Access error")]
    /// Access to the requested feature
    /// is not granted
    Access,
    #[error("Not transacted")]
    /// Failed to transact data with a remote device
    NotTransacted,
    #[error("Incorrect reader name")]
    /// The supplied reader name is incorrect
    IncorrectReaderName,
    #[error("Failed to connect to the reader")]
    /// Reader connection has been unsuccessful
    ReaderConnectionFailed,
    #[error("Reader not connected")]
    /// Reader is not in connected state
    ReaderNotConnected,
    #[error("Reader already connected")]
    /// Reader is already connected
    ReaderAlreadyConnected,
    #[error("Incorrect reader response")]
    /// There were errors in a reader
    /// response data
    ReaderIncorrectResponse,
    #[error("Reader not responding")]
    /// Waiting for a reader response
    /// timed out
    ReaderResponseFailure,
    #[error("Reader returned error code")]
    /// There is an internal error code received
    /// from a reader, followed by response vector (optional). 
    /// Check [error value](UemInternalError).
    ReaderUnsuccessful(UemInternalError, Option<Vec<u8>>),
    #[error("SAM: APDU error")]
    /// There was an APDU responde error.
    /// It can be decoded using SAM documentation.
    SamApdu,
    #[error("SAM: Invalid MAC")]
    /// Failed to check MAC signature
    /// while interacting with SAM module
    SamInvalidMac,
    #[error("SAM: Authentication failed")]
    /// Failed to authenticate with SAM module
    SamAuthenticationFailed,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Sequence, Clone)]
/// Error codes returned by a reader
pub enum UemInternalError {
    /// No RFID card found in vicinity
    NoTag = 0xFF,
    /// CRC radio transaction check failed
    Crc = 0xFE,
    /// Failed to perform operation with
    /// supplied key
    WrongKey = 0xFC,
    /// Parity bit transaction check failed
    Parity = 0xFB,
    /// Received an error result code from a card
    ResultCode = 0xFA,
    /// Wrong protocol selected for this command
    Protocol = 0xF9,
    /// Unknown serial number of a device
    SerialNumber = 0xF8,
    /// Failed to load key into flash memory
    LoadKey = 0xF7,
    /// The card is not in a valid 
    /// authentication state
    NotAuthenticated = 0xF6,
    /// Incorrect bits count during transation
    BitCount = 0xF5,
    /// Incorrect byte count during transation
    ByteCount = 0xF4,
    /// Failed to write data block on a card
    WriteData = 0xF1,
    /// Failed to increment card counter
    Increment = 0xF0,
    /// Failed to decrement card counter
    Decrement = 0xEF,
    /// Failed to read card data block
    ReadData = 0xEE,
    /// Internal radio exchange buffer overflow
    Overflow = 0xED,
    /// Reader-card transaction frame is malformed
    Framing = 0xEB,
    /// The requested operation is not implemented
    /// is a card
    UnknownOperation = 0xE9,
    /// Collision detected during card activation
    Collision = 0xE8,
    Reset = 0xE7,
    Interface = 0xE6,
    /// Bitwise avticollision is not supported
    /// by a card
    NoBitwiseAnticoll = 0xE4,
    /// Error with coding of bytes in transaction
    Coding = 0xE1,
    /// The required hardware is absent in a reader
    HardwareAbsent = 0xD8,
    /// The requested command is not implemented
    /// in a reader
    UnknownCommand = 0xD7,
    /// The requested command is not supported 
    /// by a reader
    CommandNotSupported = 0xD6,
    /// The radio chip is in wrong mode.
    /// Check that you selected right mode:
    /// ISO14443A/B or ISO15693
    WrongMfrcMode = 0xD5,
    /// Both reader and host should be in plain,
    /// authenticated and/or encrypted 
    /// transaction modes
    WrongCryptoMode = 0xD4,
    /// Please erase reader flash memory before
    /// writing new data in it
    FlashEraseRequired = 0xD3,
    /// The required key is absent in a flash memory
    FlashKeyAbsent = 0xD2,
    /// Failed to transceive data with a card
    Transceive = 0xD1,
    /// Stack overflow for ISO 15693 card
    IcodeStackOverflow = 0xD0,
    /// Failed to perform Halt operation 
    /// with ISO 14443B card
    HaltB = 0xCF,
    /// Flash reader memory is in use
    FlashOperating = 0xCE,
    /// Some internal call within reader is failed
    InternalCall = 0xCD,
    /// Error while performing cascade level 3
    /// activation with a card
    CascadeLevel10 = 0xCC,
    /// The requested card speed is not supported
    BaudrateNotSupported = 0xCA,
    /// SAM module has timed out to respond
    SamTimeout = 0xC9,
    /// There was an error with SAM APDU response
    SamApdu = 0xC8,
    /// Wrong RFID card MAC response
    /// while interacting with SAM module
    SamCardMac = 0xC7,
    /// Failed to authenticate with SAM module
    SamAuthentication = 0xC6,
    /// Incorrect bytes count sent/received from
    /// SAM module
    SamByteCount = 0xC5,
    /// Incorrect command parameter value
    ParameterValue = 0xC4,
    /// Series of error responses for
    /// Mifare Classic card type
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
    /// Series of error responses for 
    /// Mifare Plus type cards
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
    /// The requested feature is not yet implemented
    NotYetImplemented = 0x9C,
    /// CRC16 command/response check failed
    Crc16 = 0x9B,
    /// The reception buffer on reader radio chip
    /// has overflowed
    ReceiveBufferOverflow = 0x90,
    /// Error in internal reader library
    InternalReaderLibrary = 0x85,
    /// Incorrect format of a card value block
    ValueBlockFormat = 0x84,
    /// A command parameter is not supported
    UnsupportedParameter = 0x83,
    /// Commands in a chain are malformed
    IncompleteChaining = 0x82,
    /// Reader is overheating
    Temperature = 0x81,
    /// Unknown error
    Unknown = 0x80,
}

impl UemInternalError {
    pub(crate) fn from_byte(code: u8) -> Self {
        for err in all::<Self>() {
            if err.clone() as u8 == code {
                return err;
            }
        }
        Self::Unknown
    }
}