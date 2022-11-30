#[derive(Debug, PartialEq, Clone)]
pub struct JString {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JByte {
    pub value: i8,
    pub bytes: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JShort {
    pub value: i16,
    pub bytes: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JInteger {
    pub value: i32,
    pub bytes: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JLong {
    pub value: i64,
    pub high_bytes: u32,
    pub low_bytes: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JFloat {
    pub value: f32,
    pub bytes: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JDouble {
    pub value: f64,
    pub high_bytes: u32,
    pub low_bytes: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JVMTypes {
    JNull,
    JString(JString),
    JByte(JByte),
    JShort(JShort),
    JInteger(JInteger),
    JLong(JLong),
    JFloat(JFloat),
    JDouble(JDouble),
}

impl JVMTypes {
    pub fn create_string(value: &str) -> JVMTypes {
        let val = JString {
            value: value.to_string(),
        };
        JVMTypes::JString(val)
    }

    // TODO: &u32 -> u32?
    pub fn create_byte(bytes: &u32) -> JVMTypes {
        let val = JByte {
            value: *bytes as i8,
            bytes: *bytes,
        };
        JVMTypes::JByte(val)
    }

    pub fn create_short(bytes: &u32) -> JVMTypes {
        let val = JShort {
            value: *bytes as i16,
            bytes: *bytes,
        };
        JVMTypes::JShort(val)
    }

    pub fn create_integer(bytes: &u32) -> JVMTypes {
        let val = JInteger {
            value: *bytes as i32,
            bytes: *bytes,
        };
        JVMTypes::JInteger(val)
    }

    pub fn create_long(high_bytes: &u32, low_bytes: &u32) -> JVMTypes {
        let value = ((*high_bytes as u64) << 32) + *low_bytes as u64;
        let long = JLong {
            value: value as i64,
            high_bytes: *high_bytes,
            low_bytes: *low_bytes,
        };
        JVMTypes::JLong(long)
    }

    pub fn create_float(bytes: &u32) -> JVMTypes {
        let val = JFloat {
            value: f32::from_bits(*bytes),
            bytes: *bytes,
        };
        JVMTypes::JFloat(val)
    }

    pub fn create_double(high_bytes: &u32, low_bytes: &u32) -> JVMTypes {
        let bytes = ((*high_bytes as u64) << 32) + *low_bytes as u64;
        let long = JDouble {
            value: f64::from_bits(bytes),
            high_bytes: *high_bytes,
            low_bytes: *low_bytes,
        };
        JVMTypes::JDouble(long)
    }
}
