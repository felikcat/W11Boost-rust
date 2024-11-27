pub fn make_int_resource(i: u16) -> *const u16 {
    i as usize as *const u16
}
