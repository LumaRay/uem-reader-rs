use rusb::{
    ConfigDescriptor, DeviceDescriptor, DeviceHandle, DeviceList, EndpointDescriptor,
    InterfaceDescriptor, Language, Result, Device, Speed, UsbContext, Direction, GlobalContext,
};

use usb_ids::{self, FromId};

use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};

use core::{time};//, slice::SlicePattern};
use std::{time::Duration, fmt::Error, thread};

const UEM_VID: u16 = 0xC251;
const UEM_PID: u16 = 0x130A;

#[repr(u8)]
#[derive(Debug, PartialEq, Sequence, Clone)]
pub enum UemError {
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
    Unknown = 0x80
}

impl UemError {
    pub fn from_byte(code: u8) -> UemError {
        for err in all::<UemError>() {
            if err.clone() as u8 == code {
                return err;
            }
        }
        UemError::Unknown
    }
}

#[derive(Default)]
struct UsbDevice<T: UsbContext> {
    handle: Option<DeviceHandle<T>>,
    device: Option<Device<T>>,
    language: Option<Language>,
    timeout: Duration,
    ep_in_addr: u8,
    ep_out_addr: u8,
}

fn find_readers() -> Result<Vec<UsbDevice<GlobalContext>>> {
    let mut uem_devices: Vec<UsbDevice<GlobalContext>> = Vec::new();
    for device in DeviceList::new()?.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if  device_desc.vendor_id() != UEM_VID || 
            device_desc.product_id() != UEM_PID {
            continue
        }

        let mut usb_device = UsbDevice {
            ..Default::default()
        };

        for n in 0..device_desc.num_configurations() {
            let config_desc = match device.config_descriptor(n) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for interface in config_desc.interfaces() {
                for interface_desc in interface.descriptors() {
                    for endpoint_desc in interface_desc.endpoint_descriptors() {
                        match endpoint_desc.direction() {
                            Direction::In => usb_device.ep_in_addr = endpoint_desc.address(),
                            Direction::Out => usb_device.ep_out_addr = endpoint_desc.address()
                        }
                    }
                }
            }
        }
        usb_device.device = Some(device);
        uem_devices.push(usb_device);       
    }

    Ok(uem_devices)
}

fn crc16_ex(buf: &Vec<u8>, start: usize, count: usize) -> Vec<u8> {
    let mut crc: u16 = 0xFFFF;

    for pos in start..start + count {
        crc ^= buf[pos] as u16 & 0x00FF_u16;   // XOR byte into least sig. byte of crc

        for _ in 0..7 {    // Loop over each bit
            if (crc & 0x0001) != 0 {      // If the LSB is set
                crc >>= 1;                    // Shift right and XOR 0x8408
                crc ^= 0x8408;
            } else {                           // Else LSB is not set
                crc >>= 1;                    // Just shift right
            }
        }
    }

    vec![((crc ^ 0xFFFF_u16) & 0x00FF_u16) as u8, 
    (((crc ^ 0xFFFF_u16) >> 8) & 0x00FF_u16) as u8]
}

fn crc16(buf: &Vec<u8>) -> Vec<u8> {
    let buf_len = buf.len();
    crc16_ex(buf, 0, buf_len)
}

fn byte_stuff(data: &Vec<u8>) -> Vec<u8> {
    let mut stuffed_data: Vec<u8> = vec![];
    for data_byte in data {
        if (data_byte & 0xFF) < 0xFD {
            stuffed_data.push(*data_byte);
        } else {
            stuffed_data.push(0xFF);
            stuffed_data.push(0xFF - (data_byte & 0xFF));
        }
    }
    return stuffed_data;
}

fn unbyte_stuff(stuffed_data: &Vec<u8>) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];
    let mut invert_next = false;
    for data_byte in stuffed_data {
        if (data_byte & 0xFF) == 0xFF {
            invert_next = true;
            continue;
        }
        if invert_next {
            data.push(0xFF - (data_byte & 0xFF));
            invert_next = false;
        } else {
            data.push(data_byte & 0xFF);
        }
    }
    return data;
}

fn wrap_command(data: &Vec<u8>) -> Vec<u8> {
    static mut n_command: u8 = 0x00;

    let mut raw_data: Vec<u8> = vec![];

    raw_data.push(0x00);
    unsafe {
        raw_data.push(n_command);
        n_command += 1;
    }
    //if ((reader != null) && reader.Reader.encryptedMode) {
    //    rawData.write(0x00);
    //    data = AES.encryptChannel(data, reader);
    //    if (data == null)
    //        return null;
    //}
    let mut tmp_v = vec![];
    data.clone_into(&mut tmp_v);
    raw_data.append(&mut tmp_v);

    let mut fsc = crc16(&raw_data);
    //fsc.clone_into(&mut tmp_v);
    raw_data.append(&mut fsc);

    let mut tmp_data = byte_stuff(&raw_data);
    let mut raw_data: Vec<u8> = vec![];
    raw_data.reserve(2 + tmp_data.len());
    raw_data.push(0xFD);
    raw_data.append(&mut tmp_data);
    //raw_data.reserve(2);
    raw_data.push(0xFE);
    println!("{:?}", raw_data);
    return raw_data;
}

fn unwrap_response(raw_data: &Vec<u8>) -> core::result::Result<Vec<u8>, UemError> {
    let mut raw_data = unbyte_stuff(raw_data);
    if (raw_data[0] & 0xFF) != 0xFD {
        return Err(UemError::NoTag);
    }
    if (raw_data[raw_data.len()-1] & 0xFF) != 0xFE {
        return Err(UemError::NoTag);
    }
    let mut fsc = crc16(&raw_data[1..raw_data.len()-3].to_vec());
    if (fsc[0] & 0xFF) != (raw_data[raw_data.len()-3] & 0xFF) {
        return Err(UemError::CRC);//  Err(UemError::CRC);
    }
    if  (fsc[1] & 0xFF) != (raw_data[raw_data.len()-2] & 0xFF) {
        return Err(UemError::CRC);
    }
    let data = raw_data[3..raw_data.len()-3].to_vec();
    //if (reader != null) && reader.Reader._encryptedMode && (data[0] == 0x00) {
    //    data = AES.decryptChannel(Arrays.copyOfRange(data, 1, data.length), reader);
    //}
    return Ok(data);
}

const TIMEOUT: Duration = Duration::from_secs(1);

fn main() -> Result<()> {
    let mut uem_readers = find_readers().unwrap();

    if uem_readers.is_empty() {
        return Err(rusb::Error::NoDevice);
    }

    if let Some(mut usb_device) = uem_readers.get_mut(0) {
        //usb_device.handle = usb_device.device.take().unwrap().open().ok();
        if let Ok(h) = usb_device.device.take().unwrap().open() {
            //if let Ok(l) = h.read_languages(TIMEOUT) {
            //    if !l.is_empty() {
            //        usb_device.language = Some(l[0]);
            //    }
            //}
            usb_device.handle = Some(h);
            usb_device.timeout = TIMEOUT;
        }
    }

    if let Some(mut opened_dev) = uem_readers.get_mut(0) {
        if let Some(mut handle) = opened_dev.handle.as_mut() {
            //if opened_dev.handle.kernel_driver_active(0).unwrap() {
                //IsSystemDriver = true;
            //    opened_dev.handle.detach_kernel_driver(0);
            //}
            handle.claim_interface(0)?;

            //handle.write_bulk(opened_dev.ep_out_addr, &[0xFD, 0x00, 0x32, 0x05, 0x01, 0xF9, 0xA0, 0xFE], TIMEOUT);
            let mut res = handle.write_bulk(opened_dev.ep_out_addr, wrap_command(&vec![0x05_u8, 0x01_u8]).as_slice(), TIMEOUT);
            println!("{:?}", res);
            let mut buf = vec![0u8; 256];
            res = handle.read_bulk(opened_dev.ep_in_addr, &mut buf, TIMEOUT);
            println!("{:?}", res);
            if let Ok(count) = res {
                println!("{:?}", buf[..count].to_vec());
                let res = unwrap_response(&buf[..count].to_vec());
                println!("{:?}", res);
            }
            thread::sleep(time::Duration::from_millis(200));
            //handle.write_bulk(opened_dev.ep_out_addr, &[0xFD, 0x00, 0x35, 0x05, 0x01, 0xFC, 0x2C, 0xFE], TIMEOUT);
            res = handle.write_bulk(opened_dev.ep_out_addr, wrap_command(&vec![0x05_u8, 0x01_u8]).as_slice(), TIMEOUT);
            println!("{:?}", res);
            handle.release_interface(0)?;
        }
    }

    Ok(())
}