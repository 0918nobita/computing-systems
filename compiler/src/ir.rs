#[derive(Debug, Default)]
pub struct Ir {
    pub num_globals: i32,
    pub string_pool: Vec<String>,
    pub insts: Vec<IrInst>,
}

#[derive(Debug)]
pub enum IrInst {
    GetStaticStr(i32),
    GetGlobal(i32),
    SetGlobal(i32),
    Print,
}
