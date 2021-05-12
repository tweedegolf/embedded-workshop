/// Blockingly delay computation for a given amount of milliseconds
pub fn delay_millis(ms: u32) {
    // `cortex_m::asm::delay`  takes an of clock cycles
    // for which to wait. As our nRF runst on a 64Mhz clock
    // frequency, we multiply the amount of milliseconds by 64000
    cortex_m::asm::delay(ms * 64000);
}
