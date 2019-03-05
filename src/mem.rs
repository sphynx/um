use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::u32;

pub struct Mem {
    data: Vec<Option<Vec<u32>>>,
    free_pq: BinaryHeap<Reverse<u32>>,
    len: u32, // FIXME: this is not actually needed for now, we can just use data.len()
}

impl Mem {
    pub fn new() -> Self {
        Mem {
            data: Vec::new(),
            free_pq: BinaryHeap::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn alloc(&mut self, size: u32) -> u32 {
        match self.free_pq.pop() {
            Some(Reverse(addr)) => {
                let v = vec![0; size as usize];
                self.data[addr as usize] = Some(v);
                addr
            }

            None => {
                let v = vec![0; size as usize];
                self.data.push(Some(v));
                self.len += 1;
                self.len - 1
            }
        }
    }

    pub fn free(&mut self, addr: u32) {
        match self.data.get_mut(addr as usize) {
            Some(Some(v)) => {
                v.clear(); // FIXME: is that needed?
                self.data[addr as usize] = None;
                self.free_pq.push(Reverse(addr));
            }
            Some(None) => panic!(
                "free: attempt to free address {} which is already free",
                addr
            ),
            None => panic!("free: attempt to free unallocated address {}", addr),
        }
    }

    pub fn read(&self, addr: u32, offset: u32) -> &u32 {
        match &self.data.get(addr as usize) {
            Some(Some(v)) => match v.get(offset as usize) {
                Some(val) => val,
                None => panic!(
                    "read: offset {} is out of bounds for address {} (len: {})",
                    offset,
                    addr,
                    v.len()
                ),
            },
            Some(None) => panic!("read: address {} has been deallocated", addr),
            None => panic!("read: address {} has not been allocated", addr),
        }
    }

    pub fn write(&mut self, addr: u32, offset: u32, val: u32) {
        match self.data.get_mut(addr as usize) {
            Some(Some(v)) => {
                if (offset as usize) < v.len() {
                    v[offset as usize] = val;
                } else {
                    panic!(
                        "write: offset {} is out of bounds for address {} (len: {})",
                        offset,
                        addr,
                        v.len()
                    );
                }
            }
            Some(None) => panic!("write: address {} has been deallocated", addr),
            None => panic!("write: address {} has not been allocated", addr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc() {
        let mut mem = Mem::new();
        let m0 = mem.alloc(10);
        let m1 = mem.alloc(20);
        assert_eq!(mem.len(), 2);
        assert_eq!(m0, 0);
        assert_eq!(m1, 1);
    }

    #[test]
    #[should_panic(expected = "0 has been deallocated")]
    fn free_err() {
        let mut mem = Mem::new();
        let m0 = mem.alloc(10);
        mem.free(m0);
        mem.read(m0, 1);
    }

    #[test]
    #[should_panic(expected = "attempt to free unallocated address 0")]
    fn free_err2() {
        let mut mem = Mem::new();
        mem.free(0);
    }

    #[test]
    #[should_panic(expected = "attempt to free address 0 which is already free")]
    fn double_free_err() {
        let mut mem = Mem::new();
        let m0 = mem.alloc(10);
        mem.free(m0);
        mem.free(m0);
    }

    #[test]
    fn alloc_lowest() {
        let mut mem = Mem::new();

        let m0 = mem.alloc(10);
        let m1 = mem.alloc(20);
        let _m2 = mem.alloc(30);

        mem.free(m0);
        mem.free(m1);

        let m3 = mem.alloc(40);
        assert_eq!(m3, m0);
    }

    #[test]
    fn len3() {
        let mut mem = Mem::new();

        let m0 = mem.alloc(10);
        let m1 = mem.alloc(20);
        let m2 = mem.alloc(30);

        mem.free(m0);
        mem.free(m1);
        mem.free(m2);

        assert_eq!(mem.len(), 3);
    }

    #[test]
    fn len1() {
        let mut mem = Mem::new();

        let m0 = mem.alloc(10);
        mem.free(m0);

        let m1 = mem.alloc(20);
        mem.free(m1);

        mem.alloc(30);
        assert_eq!(mem.len(), 1);
    }

    #[test]
    fn init_with_zero() {
        let mut mem = Mem::new();

        let m0 = mem.alloc(10);
        for i in 0..10 {
            assert_eq!(mem.read(m0, i), &0);
        }
    }

    #[test]
    #[should_panic]
    fn read_err_offset() {
        let mut mem = Mem::new();
        let m0 = mem.alloc(10);
        mem.read(m0, 10);
    }

    #[test]
    #[should_panic]
    fn read_err_zero() {
        let mut mem = Mem::new();
        let m0 = mem.alloc(0);
        mem.read(m0, 0);
    }

    #[test]
    #[should_panic]
    fn read_err_addr() {
        let mem = Mem::new();
        mem.read(1, 0);
    }

    #[test]
    fn write_and_read() {
        let mut mem = Mem::new();
        let block0 = mem.alloc(10);
        mem.write(block0, 0, 384);
        assert_eq!(mem.read(block0, 0), &384);
    }

    #[test]
    #[ignore]
    fn fill_all_memory() {
        let mut mem = Mem::new();
        for _ in 0..=u32::MAX {
            mem.alloc(1);
        }
        assert_eq!(mem.len(), u32::MAX);
    }
}
