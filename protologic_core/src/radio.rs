use crate::lowlevel::quickstate::get_general_quickstate;

/// Get all radio messages received in the last tick.
pub fn radio_receive(output: &mut Vec<u64>)
{
    // Reserve some space
    let inbox_count = get_general_quickstate().read_i32(672);
    output.clear();
    output.reserve(inbox_count as usize);

    // Do nothing if there are no pending messages
    if inbox_count == 0 {
        return;
    }

    // Read messages into buffer
    unsafe
    {
        let start = output.as_mut_ptr();
        let count = radio_rx(start, (output.capacity() as i32) * 8);
        assert_eq!(inbox_count, count);
        output.set_len(count as usize);
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

protologic_define_extern!(pub fn radio_tx(message: u64, range: f32));
protologic_define_extern!(pub fn radio_rx(addr: *mut u64, bytes: i32) -> i32);
protologic_define_extern!(pub fn radio_rx_filter(filter: u64, mask: u64));