/// # Safety
///
/// The `address` must be a valid memory address that points to a `u32` value.
/// This function is unsafe because it operates directly on raw pointers, and
/// the caller is responsible for ensuring that the provided `address` is
/// valid and correctly aligned. It should only be used with caution in cases
/// where you can guarantee memory safety.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The caller must ensure that the provided address points to a
    // valid `u32` and respects the alignment requirements of `u32`.
    let value = address as *mut u32;
    *value = 0xAABBCCDD;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
