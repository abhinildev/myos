#[derive(Clone, Copy)]
pub struct DirEntry {
    pub inode:u32,
    pub name_len:u8,
    pub name: [u8;16],
}
impl DirEntry {
    pub fn new(name: &str, inode:u32)->Self {
        let mut name_arr=[0u8;16];
        let bytes=name.as_bytes();
        for i in 0..bytes.len().min(16){
            name_arr[i]=bytes[i];
        }
        Self {
            inode,
            name_len:bytes.len() as u8,
            name: name_arr,
        }
    }
}