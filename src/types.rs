#[derive(Debug)]
pub struct JString {
    pub value: String,
}
#[derive(Debug)]
pub struct JInteger {
    pub value: i32,
    pub bytes: u32,
}
#[derive(Debug)]
pub struct JLong {
    pub value: i64,
    pub high_bytes: u32,
    pub low_bytes: u32,
}
#[derive(Debug)]
pub struct JFloat {
    pub value: f32,
    pub bytes: u32,
}
#[derive(Debug)]
pub struct JDouble {
    pub value: f64,
    pub high_bytes: u32,
    pub low_bytes: u32,
}

#[derive(Debug)]
pub enum JVMTypes {
    JString(JString),
    JInteger(JInteger),
    JLong(JLong),
    JFloat(JFloat),
    JDouble(JDouble),
}
