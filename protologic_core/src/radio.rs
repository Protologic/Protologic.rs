use std::{sync::OnceLock, alloc::Layout};

use crate::lowlevel::queries::QuickStateBox;

/// Get all radio messages received in the last tick.
pub fn radio_receive(output: &mut Vec<u64>)
{
    // Get number of messages (first 4 bytes in the buffer)
    let pre_count = get_radio_quickstate().read_i32(0);

    // Reserve sufficient space
    output.clear();
    output.reserve(pre_count as usize);

    // Read all messages one by one (with 8 byte offset to the start of the first message)
    for i in 0..pre_count {
        output.push(get_radio_quickstate().read_u64((8 + 8 * i) as usize));
    }
}

/// Configure the radio to only receive message which pass the filter test:
/// `test(message) => (message & mask) == filter`
pub fn radio_receive_filter(filter: u64, mask: u64)
{
    unsafe
    {
        radio_rx_filter(filter, mask);
    }
}

/// Transmit an 8 byte message to all receivers in range
pub fn radio_transmit(message: u64, range: f32)
{
    unsafe
    {
        radio_tx(message, range);
    }
}

/// Convenience function to pack 8 bytes into a u64
pub fn radio_pack(bytes: [u8;8]) -> u64
{
    return u64::from_be_bytes(bytes);
}

/// Convenience function to unpack a u64 into 8 bytes
pub fn radio_unpack(value: u64) -> [u8;8]
{
    return u64::to_be_bytes(value);
}

static RADIO_QUICK_STATE: OnceLock<QuickStateBox> = OnceLock::new();
fn get_radio_quickstate() -> &'static QuickStateBox
{
    const SIZE : usize = 1024usize;
    return RADIO_QUICK_STATE.get_or_init(||
    {
        // Allocate a large buffer and tell the engine about it
        let layout = Layout::new::<[u8; SIZE]>();
        let buffer = unsafe { std::alloc::alloc_zeroed(layout) };
        unsafe { set_radio_rx_buffer(buffer, SIZE as i32) };

        return QuickStateBox { ptr: buffer };
    });
}

pub(crate) unsafe fn get_radio_buffer_pointer() -> *mut u8
{
    return get_radio_quickstate().ptr;
}

protologic_define_extern!(pub fn set_radio_rx_buffer(addr: *mut u8, len: i32));
protologic_define_extern!(pub fn radio_tx(message: u64, range: f32));
protologic_define_extern!(pub fn radio_rx_filter(filter: u64, mask: u64));