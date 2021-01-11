#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    NoOp = 0,
    Label = 1,
    Push0 = 2,
    Pop = 3,
    Bump = 4,
    Func = 5,
    Jump = 6,
    PushConst = 7
}

#[derive(Debug, Clone, Copy)]
pub struct ByteCode {
    pub op_code: OpCode,
    pub arg: i32
}
