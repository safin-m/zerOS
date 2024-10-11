use x86_64::instructions::port::Port;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum OSExitCode {
    Success = 0x10,
    Fail = 0x11,
}

#[allow(dead_code)]
pub fn exit_os(exit_code: OSExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
