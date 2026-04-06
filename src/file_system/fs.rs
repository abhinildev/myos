const INODE_START: usize = 1;
const INODE_COUNT: usize = 16;
const DATA_START: usize = INODE_START + INODE_COUNT;



use myos::println;

use crate::file_system::block::*;
use crate::file_system::inode::*;
use crate::file_system::dir::*;
pub struct FileSystem{
    pub disk: Disk,
}
impl FileSystem{
    pub fn new()-> Self{
        Self{
            disk: Disk::new(),
        }
    }
    pub fn init(&mut self){
        let root_inode=Inode{
            size:0,
            block: DATA_START as u32,
            file_type:2,
        };
        self.write_inode(0,root_inode)
    }
    pub fn read_inode(&self, inode_num:usize)->Inode{
        let block=INODE_START+inode_num;
        let data=self.disk.read_block(block);
        unsafe {*(data.as_ptr() as *const Inode)}
    }
    pub fn write_inode(&mut self, inode_num:usize,inode:Inode){
        let block=INODE_START + inode_num;
        let mut buf=[0u8; BLOCK_SIZE];
        unsafe {
            *(buf.as_mut_ptr() as *mut Inode)= inode;
        }
        self.disk.write_block(block,buf) ;
    }
    pub fn add_dir_entry(&mut self, name:&str, inode_num:u32){
        let root=self.read_inode(0);
        let block=root.block as usize;
        let mut data=self.disk.read_block(block);
        let entry=DirEntry::new(name,inode_num);
        let entry_size=core::mem::size_of::<DirEntry>();
        for i in 0..(BLOCK_SIZE/entry_size){
            let offset=i* entry_size;
            let ptr=&data[offset] as *const u8;
            let existing =unsafe {*(ptr as *const DirEntry)};
            if existing.inode==0{
                unsafe{
                    let dst=data.as_mut_ptr().add(offset) as *mut DirEntry;
                    *dst=entry;
                }
                self.disk.write_block(block, data);
                return;
            }
        }
    }
    pub fn create(&mut self, name:&str){
        let inode_num=1;
        let inode=Inode{
            size:0,
            block:0,
            file_type:1
        };
        self.write_inode(inode_num, inode);
        self.add_dir_entry(name, inode_num as u32);
    }
    pub fn find_inode(&self, name:&str)->Option<u32> {
        let root=self.read_inode(0);
        let block =root.block as usize;
        let data=self.disk.read_block(block);
        let entry_size=core::mem::size_of::<DirEntry>();
        for i in 0..(BLOCK_SIZE / entry_size){
            let offset=i * entry_size;
            let entry= unsafe {
                *(data.as_ptr().add(offset) as *const DirEntry)
            };
                    if entry.inode != 0 {
            let name_bytes = &entry.name[..entry.name_len as usize];

            if name_bytes == name.as_bytes() {
                return Some(entry.inode);
            }

        }
    }
    None
}
pub fn write_file(&mut self, name: &str, content: &str) {
    let inode_num = self.find_inode(name).expect("File not found");

    let mut inode = self.read_inode(inode_num as usize);

    let block = DATA_START + 1; // simple allocation

    let mut buf = [0u8; BLOCK_SIZE];

    let bytes = content.as_bytes();

    for i in 0..bytes.len() {
        buf[i] = bytes[i];
    }

    self.disk.write_block(block, buf);

    inode.size = bytes.len() as u32;
    inode.block = block as u32;

    self.write_inode(inode_num as usize, inode);
}
pub fn read_file(&self, name: &str) {
    let inode_num = self.find_inode(name).expect("File not found");

    let inode = self.read_inode(inode_num as usize);

    let data = self.disk.read_block(inode.block as usize);

    let size = inode.size as usize;

    let content = core::str::from_utf8(&data[..size]).unwrap();

    println!("File content: {}", content);
}

}