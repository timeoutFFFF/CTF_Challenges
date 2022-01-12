//! A software MMU with byte level permissions and uninitialize memory access
//! detection

use crate::primitive::Primitive;
use crate::emulator::VmExit;
use std::path::Path;

// Block size used for resetting and tracking memory which has been modified
/// THe larger this is, the fewer but more expensive memcpys() need to occur,
/// the small, the greater but less expensive memcpys() need to occur.
/// It seems the sweet spot is often 128-4096 bytes. 
const DIRTY_BLOCK_SIZE: usize = 4096;


pub const PERM_READ: u8  = 1 << 0;
pub const PERM_WRITE: u8 = 1 << 1;
pub const PERM_EXEC: u8 = 1 << 2;
/// to find uninitialzed memory
pub const PERM_RAW: u8   = 1 << 3;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct Perm(pub u8);

/// A guest virtual address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct VirtAddr(pub usize);

pub struct Sections {
    pub file_offset: usize,
    pub virt_addr: VirtAddr,
    pub file_size: usize,
    pub mem_size: usize,
    pub permissions: Perm,
}


pub struct Mmu{
    /// Block of memory for this address space
    /// offset 0 correspons to address 0 in the guest address space
    memory: Vec<u8>,

    /// Holding permission bytes for the correponding byte in the memory
    permissions: Vec<Perm>,
    
    /// Tracks blocks in `memory` which are dirty 
    dirty: Vec<usize>,

    /// Tracks which parts of memeory have been dirtied
    dirty_bitmap: Vec<u64>,
    
    /// current base of address of next allocations
    cur_alloc: VirtAddr,
}

impl Mmu{
    
    /// creating Mmu object
    pub fn new(size: usize) -> Self{
        Mmu{
            memory:       vec![0; size],
            permissions:  vec![Perm(0); size],
            dirty:        Vec::with_capacity(size / DIRTY_BLOCK_SIZE + 1),
            dirty_bitmap: vec![0u64; size / DIRTY_BLOCK_SIZE / 64 + 1],
            cur_alloc:     VirtAddr(0x1000),
        }
    }

    /// Fork from an existing MMU
    pub fn fork(&self) -> Self{
        let size = self.memory.len();

        Mmu{
            memory:       self.memory.clone(),
            permissions:  self.permissions.clone(),
            dirty:        Vec::with_capacity(size / DIRTY_BLOCK_SIZE + 1),
            dirty_bitmap: vec![0u64; size / DIRTY_BLOCK_SIZE / 64 + 1],
            cur_alloc:    self.cur_alloc.clone(),
        }
    }

    
    /// Restore memory back to the original state ( eg. restores all
    /// dirty block to the state of `other`
    pub fn reset(&mut self, other: &Mmu){

        for &block in &self.dirty {
            //Get the start and end addresses of the dirtied memory
            let start = block * DIRTY_BLOCK_SIZE;
            let end = (block + 1) * DIRTY_BLOCK_SIZE; 
            
            // Zero the bitmap. 
            self.dirty_bitmap[block / 64] = 0;

            // Restore memory state
            self.memory[start..end].copy_from_slice(&other.memory[start..end]);

            // Restor permissions
            self.permissions[start..end].copy_from_slice(
                &other.permissions[start..end]);
        }

        //cleart dirty lisrt
        self.dirty.clear();

        //Restore allocator state
        self.cur_alloc = other.cur_alloc;

    }
    
    /// function allocates the address
    pub fn allocate(&mut self, size: usize) -> Option<VirtAddr>{
        
        // Align the address size
        let align_size = size; // + 0xF & !0xF; 

        let base = self.cur_alloc;
        
        // Cannot allocate
        if base.0 >= self.memory.len() {
            return None;
        }

        self.cur_alloc = VirtAddr(self.cur_alloc.0.checked_add(align_size)?);

        if self.memory.len() < self.cur_alloc.0 {
             return None;
        }

        self.set_permissions(base, align_size, Perm(PERM_RAW|PERM_WRITE));

        Some(base)


    }
        /// Apply permissions to a region of memory
        pub fn set_permissions(&mut self, addr: VirtAddr, size: 
                               usize, perm: Perm) -> Option<()> {
            self.permissions.get_mut(addr.0..addr.0.checked_add(size)?)?
                .iter_mut().for_each(|x| *x = perm);

            Some(())
        }

        pub fn write_from(&mut self, addr: VirtAddr,  buf: &[u8]) 
            -> Result<(), VmExit> {

            let perms = self.permissions.get_mut(addr.0..addr.0.checked_add(buf.len())
                                                 .ok_or(VmExit::AddressIntegerOverflow)?)
                .ok_or(VmExit::AddressMiss(addr, buf.len()))?;
             
            let mut has_raw = false;
            
            for (idx, &perm) in perms.iter().enumerate() {
                has_raw |= (perm.0 & PERM_RAW)  != 0;
                if (perm.0 & PERM_WRITE) == 0 {
                    return Err(VmExit::WriteFault(VirtAddr(addr.0 + idx)));
                }
            }
           
            // Copy the buffer into memory
            self.memory[addr.0..addr.0 + buf.len()].copy_from_slice(buf);

            // Compute dirty bit blocks
            let block_start = addr.0 / DIRTY_BLOCK_SIZE;
            let block_end = (addr.0 + buf.len()) / DIRTY_BLOCK_SIZE;
             
            for block in block_start..=block_end{
                // Determine the bitmap postion of the dirty block
                let idx = block_start / 64;
                let bit = block_start  % 64;
                
                // Check if the block is not dirty
                if self.dirty_bitmap[idx] & (1 << bit) == 0 {
                    // Block is not dirty add it to the dirty list
                    self.dirty.push(block );

                    // Update the dirty bitmap
                    self.dirty_bitmap[idx]  |= 1 << bit; 
                }}

            if has_raw {
                perms.iter_mut().for_each(|x| {
                    if (x.0 & PERM_RAW) != 0 {
                        //mark the memory readable
                        *x = Perm(x.0 | PERM_READ);
                    }
                });
            }


            Ok(())
        }
       
        /// Return an immutable slice to emory at `addr` for `size` bytes that
        /// has been validated to amtch all `exp_perms`
        pub fn peek(&self, addr: VirtAddr,  size: usize, 
                               exp_perms: Perm) -> Result<&[u8], VmExit> {

            let perms = self.permissions.get(addr.0..addr.0.checked_add(size)
                                                 .ok_or(VmExit::AddressIntegerOverflow)?)
                .ok_or(VmExit::AddressMiss(addr, size))?;
            
            for (idx, &perm) in perms.iter().enumerate() {
                if (perm.0 & exp_perms.0) != exp_perms.0 {
                    return Err(VmExit::ReadFault(VirtAddr(addr.0 + idx)));
                }
            }
            
            Ok(&self.memory[addr.0..addr.0 + size] )

        }
        
        /// Read the memory at `addr` into `buf`
        ///  This function checks to see if all bits in `exp_perms` are set 
        ///  in the permission bytes
        pub fn read_into_perms(&self, addr: VirtAddr,  buf: &mut [u8], 
                               exp_perms: Perm) -> Result<(), VmExit> {

            let perms = self.permissions.get(addr.0..addr.0.checked_add(buf.len())
                                                 .ok_or(VmExit::AddressIntegerOverflow)?)
                .ok_or(VmExit::AddressMiss(addr, buf.len()))?;
            
            for (idx, &perm) in perms.iter().enumerate() {
                if (perm.0 & exp_perms.0) != exp_perms.0 {
                    return Err(VmExit::ReadFault(VirtAddr(addr.0 + idx)));
                }
            }
            
            buf.copy_from_slice(&self.memory[addr.0..addr.0 + buf.len()]);

            Ok(())
        }
        
        pub fn read_into(&self, addr: VirtAddr, buf: &mut [u8]) 
                    -> Result<(), VmExit>{
            self.read_into_perms(addr, buf, Perm(PERM_READ))
        }
        
        /// Read a type `T` at `addr` 
        pub fn read_perms<T: Primitive>(&mut self, addr: VirtAddr,
                                        exp_perms: Perm) -> Result<T, VmExit>{
            let mut tmp = [0u8; 16];
            self.read_into_perms(addr, &mut tmp[..core::mem::size_of::<T>()], exp_perms)?;
            Ok(unsafe { core::ptr::read_unaligned(tmp.as_ptr() as *const T)})

        }

        /// Read a type `T` at `addr` 
        pub fn read<T: Primitive>(&mut self, addr: VirtAddr) -> Result<T, VmExit>{
            self.read_perms(addr, Perm(PERM_READ))

        }
 
        
        /// Write a type `T` at `addr` 
        pub fn write<T: Primitive>(&mut self, addr: VirtAddr, 
                                   val: T) -> Result<(), VmExit>{
            let tmp  = unsafe {
                core::slice::from_raw_parts(&val as *const T as *const u8,
                                            core::mem::size_of::<T>())
            };

            self.write_from(addr, tmp)
        }


        

    /// Load a file into the emulator address space using the sections as
    /// described
    pub fn load<P: AsRef<Path>>(&mut self, filename: P,
                                sections: &[Sections]) -> Option<()>{

        // Read the input file
        let contents = std::fs::read(filename).ok()?;

        for section in sections {
            self.set_permissions(section.virt_addr, 
                                        section.mem_size,
                                        Perm(PERM_WRITE))?;

            // Write in the original file contents
            self.write_from(section.virt_addr,
                                   contents.get(
                                       section.file_offset..
                                       section.file_offset.checked_add(section.file_size)?)?
                                   ).ok()?;

            // Write in any padding with zeros
            if section.mem_size > section.file_size {
                let padding = vec![0u8; section.mem_size - section.file_size];
                self.write_from( 
                    VirtAddr(section.virt_addr.0.checked_add(section.file_size)?),
                    &padding).ok()?;
            }

            // Demote the permission
            for section in sections {
            self.set_permissions(section.virt_addr, 
                                        section.mem_size,
                                        section.permissions
                                        )?;
            }
        }
         Some(())

    }


        
}


