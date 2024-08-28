pub(crate) fn harden(index: u32) -> u32 {
    index | 0x80_00_00_00
}
