use rusb::{
    ConfigDescriptor, DeviceDescriptor, DeviceHandle, DeviceList, EndpointDescriptor,
    InterfaceDescriptor, Language, Result, Device, Speed, UsbContext, Direction, GlobalContext,
};

use usb_ids::{self, FromId};

use core::time;
use std::{time::Duration, fmt::Error, thread};

const UEM_VID: u16 = 0xC251;
const UEM_PID: u16 = 0x130A;

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

        for _ in (1..8).rev() {    // Loop over each bit
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

fn byte_stuff(data: Vec<u8>) -> Vec<u8> {
    let mut stuffed_data: Vec<u8> = vec![];
    for data_byte in data {
        if (data_byte & 0xFF) < 0xFD {
            stuffed_data.push(data_byte);
        } else {
            stuffed_data.push(0xFF);
            stuffed_data.push(0xFF - (data_byte & 0xFF));
        }
    }
    return stuffed_data;
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

    let mut tmp_data = byte_stuff(raw_data);
    let mut raw_data: Vec<u8> = vec![];
    raw_data.reserve(2 + tmp_data.len());
    raw_data.push(0xFD);
    raw_data.append(&mut tmp_data);
    //raw_data.reserve(2);
    raw_data.push(0xFE);

    return raw_data;
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
            handle.write_bulk(opened_dev.ep_out_addr, wrap_command(&vec![0x05_u8, 0x01_u8]).as_slice(), TIMEOUT);
            thread::sleep(time::Duration::from_millis(200));
            //handle.write_bulk(opened_dev.ep_out_addr, &[0xFD, 0x00, 0x35, 0x05, 0x01, 0xFC, 0x2C, 0xFE], TIMEOUT);
            handle.write_bulk(opened_dev.ep_out_addr, wrap_command(&vec![0x05_u8, 0x01_u8]).as_slice(), TIMEOUT);
            
            handle.release_interface(0)?;
        }
    }

    Ok(())
}