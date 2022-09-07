pub struct ClassLoader {}

impl ClassLoader {
    pub fn loadClass(self, className: &String) -> Vec<u8> {
        return vec![0x04, 0x3C, 0x05, 0x3D, 0x1B, 0x1C, 0x60, 0xAC];
    }
}
