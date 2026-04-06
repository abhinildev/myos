pub const BLOCK_SIZE: usize=512;
pub const NUM_BLOCKS: usize=1024;

pub struct Disk {
    pub data: [[u8; BLOCK_SIZE]; NUM_BLOCKS],
}
impl Disk {
    pub fn new()->Self{
        Self{
            data:[[0; BLOCK_SIZE]; NUM_BLOCKS],
        }
    }
    pub fn read_block(&self, block_num: usize)-> [u8; BLOCK_SIZE]{
        self.data[block_num]
    }
    pub fn write_block(&mut self, block_num: usize, buf:[u8; BLOCK_SIZE]){
        self.data[block_num]=buf;
    }
}
