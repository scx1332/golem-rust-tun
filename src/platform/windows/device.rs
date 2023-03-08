//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

// This library won't work on windows, it's just dummy implementation so that it
// compiles on windows. It's convenient for people that are developing on Windows machine
// also it is convenient to include library in cross platform projects.

#![allow(unused_variables)]

use std::io::{self, Read, Write};
use std::net::Ipv4Addr;

use crate::configuration::Configuration;
use crate::device::Device as D;
use crate::error::*;

pub type RawFd = i32;

pub struct Fd(pub RawFd);

impl Fd {
    pub fn new(fd: RawFd) -> Result<Self> {
        Ok(Fd(fd))
    }
}

pub trait AsRawFd {
    fn as_raw_fd(&self) -> RawFd;
}

pub trait IntoRawFd {
    fn into_raw_fd(self) -> RawFd;
}

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.clone()
    }
}

impl IntoRawFd for Fd {
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

/// A TUN device for iOS.
pub struct Device {
    queue: Queue,
}

impl Device {
    /// Create a new `Device` for the given `Configuration`.
    pub fn new(config: &Configuration) -> Result<Self> {
        let fd = match config.raw_fd {
            Some(raw_fd) => raw_fd,
            _ => return Err(Error::InvalidConfig),
        };
        let device = {
            let tun = Fd::new(fd).map_err(|_| io::Error::last_os_error())?;

            Device {
                queue: Queue { tun: tun },
            }
        };
        Ok(device)
    }

    /// Split the interface into a `Reader` and `Writer`.
    pub fn split(self) -> () {
    }

    /// Return whether the device has packet information
    pub fn has_packet_information(&self) -> bool {
        self.queue.has_packet_information()
    }

    /// Set non-blocking mode
    pub fn set_nonblock(&self) -> io::Result<()> {
        self.queue.set_nonblock()
    }
}

impl Read for Device {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Write for Device {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        Ok(0)
    }
}

impl D for Device {
    type Queue = Queue;

    fn name(&self) -> &str {
        "WindowsDummyDevice"
    }

    fn set_name(&mut self, value: &str) -> Result<()> {
        Err(Error::NotImplemented)
    }

    fn enabled(&mut self, value: bool) -> Result<()> {
        Ok(())
    }

    fn address(&self) -> Result<Ipv4Addr> {
        Err(Error::NotImplemented)
    }

    fn set_address(&mut self, value: Ipv4Addr) -> Result<()> {
        Ok(())
    }

    fn destination(&self) -> Result<Ipv4Addr> {
        Err(Error::NotImplemented)
    }

    fn set_destination(&mut self, value: Ipv4Addr) -> Result<()> {
        Ok(())
    }

    fn broadcast(&self) -> Result<Ipv4Addr> {
        Err(Error::NotImplemented)
    }

    fn set_broadcast(&mut self, value: Ipv4Addr) -> Result<()> {
        Ok(())
    }

    fn netmask(&self) -> Result<Ipv4Addr> {
        Err(Error::NotImplemented)
    }

    fn set_netmask(&mut self, value: Ipv4Addr) -> Result<()> {
        Ok(())
    }

    fn mtu(&self) -> Result<i32> {
        Err(Error::NotImplemented)
    }

    fn set_mtu(&mut self, value: i32) -> Result<()> {
        Ok(())
    }

    fn queue(&mut self, index: usize) -> Option<&mut Self::Queue> {
        if index > 0 {
            return None;
        }

        Some(&mut self.queue)
    }
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.queue.as_raw_fd()
    }
}

impl IntoRawFd for Device {
    fn into_raw_fd(self) -> RawFd {
        self.queue.into_raw_fd()
    }
}

pub struct Queue {
    tun: Fd,
}

impl Queue {
    pub fn has_packet_information(&self) -> bool {
        // on ios this is always the case
        true
    }

    pub fn set_nonblock(&self) -> io::Result<()> {
        //self.tun.set_nonblock()
        Ok(())
    }
}



impl AsRawFd for Queue {
    fn as_raw_fd(&self) -> RawFd {
        self.tun.as_raw_fd()
    }
}

impl IntoRawFd for Queue {
    fn into_raw_fd(self) -> RawFd {
        self.tun.into_raw_fd()
    }
}

impl Read for Queue {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        //self.tun.read(buf)
        Ok(0)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        //self.tun.read_vectored(bufs)
        Ok(0)
    }
}

impl Write for Queue {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        //self.tun.write(buf)
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {

        Ok(())
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        Ok(0)
        //self.tun.write_vectored(bufs)
    }
}
