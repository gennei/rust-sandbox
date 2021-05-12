use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use sed::path::Path;

pub const PAGE_SIZE: usize= 4096;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromBytes, AsBytes)]
#[repr(C)]
pub struct PageId(pub u64);

impl PageId {
    pub const INVALID_PAGE_ID: PageId = PageId(u64::MAX);

    pub fn valid(self) -> Option<PageId> {
        if self == Self::INVALID_PAGE_ID {
            None
        } else {
            Some(self)
        }
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }
}

impl Default for PageId {
    fn default()-> Self {
        Self::INVALID_PAGE_ID
    }
}

impl From<Option<PageId>> for PageId {
    fn from(page_id: Option<PageId>) -> Self {
        page_id.unwrap_or_default()
    }
}

impl From<&[u8]> for PageId {
    fn from(bytes: &[u8]) -> Self {
        let arr = bytes.try_into().unwrap();
        PageId(u64::from_ne_bytes(arr))
    }
}

pub struct DiskManager {
    
    heap_file: File;
    next_page_id: u64;
}

impl DiskManager {

    pub fn new(data_file: File) -> io::Result<Self> {
        let head_file_size = head_file.metadata()?.len();
        let next_page_id = head_file_size / PAGE_SIZE as u64;
        Ok(Self {
            headp_file,
            next_page_id,
        })
    }

    pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new().read(true).write(true).create(true).open(head_file_path)?;
        Self::new(heap_file)
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {

    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        
    }

    pub fn allocate_page(&mut self) -> PageId {

    }

    pub fn sync(&mut self) -> io::Result<()> {

    }
}
