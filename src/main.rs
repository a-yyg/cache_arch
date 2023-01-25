use std::fmt;

// Generic Memory Implementation, with ability to act as cache
// for other memory implementations.
#[derive(Clone)]
pub struct MemoryField {
    valid: bool,
    data: u8,
}

impl Default for MemoryField {
    fn default() -> Self {
        Self {
            valid: false,
            data: 0,
        }
    }
}

pub struct Memory {
    data: Vec<MemoryField>,
    cache: Option<Box<Memory>>,
}

impl Memory {
    pub fn new(bsize: usize) -> Self {
        Self {
            data: vec![MemoryField::default(); bsize],
            cache: None,
        }
    }

    pub fn set(&mut self, index: usize, val: u8) {
        self.data[index].valid = true;
        self.data[index].data = val;
    }

    pub fn get(&mut self, index: usize) -> u8 {
        if !self.data[index].valid {
            let lookup = 
                self.cache.as_mut()
                          .expect("No backup memory specified for cache.")
                          .get(index);
            self.data[index].valid = true;
            self.data[index].data = lookup
        }
            
        self.data[index].data
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, MemoryField{valid: _, data}) in self.data.iter().enumerate() {
            write!(f, "{:02X} ", data)?;
            if i > 0 && i % 8 == 7 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let mut mem = Memory::new(1 << 6); // 64 bits
    mem.set(0x00, 0x0F);
    mem.set(0x10, 0xDE);
    mem.set(0x20, 0xAD);

    println!("{}", mem);
}

#[test]
fn test_mem() {
    let mut mem = Memory::new(1 << 6); // 64 bits
    mem.set(0x00, 0x0F);
    assert_eq!(mem.get(0x00), 0x0F);

    mem.set(0x10, 0xDE);
    assert_eq!(mem.get(0x10), 0xDE);

    mem.set(0x20, 0xAD);
    assert_eq!(mem.get(0x20), 0xAD);
}
