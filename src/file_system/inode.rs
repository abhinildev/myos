#[derive(Clone, Copy,Default)]
pub struct Inode {
    pub  size: u32,
    pub block: u32,
    pub file_type: u16, // 1=file , 2=dir
}
