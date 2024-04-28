pub struct Rasengan<T: Copy, const N: usize> {
    buf: [Option<T>; N],
    read_ptr: usize,
    write_ptr: usize,
}

#[allow(dead_code)]
impl<T: Copy, const N: usize> Rasengan<T, N> {
    pub fn new() -> Self {
        Self {
            buf: [None; N],
            read_ptr: 1,
            write_ptr: 0,
        }
    }

    fn wrapping_increment(&self, idx: usize) -> usize {
        (idx + 1) % self.buf.len()
    }

    fn will_overwrite_unread_data(&self) -> bool {
        self.write_ptr + 1 == self.read_ptr + self.buf.len()
    }

    // Overwrites when buffer is full
    pub fn write(&mut self, data: T) {
        if self.will_overwrite_unread_data() {
            self.read_ptr += 1;
        }

        self.write_ptr += 1;
        self.buf[self.write_ptr % self.buf.len()] = Some(data);
    }

    pub fn read(&mut self) -> Option<T> {
        // this relies on the fact that read will always lead write
        if self.write_ptr < self.read_ptr {
            return None;
        }

        let data = self.buf[self.read_ptr % self.buf.len()];
        self.read_ptr += 1;

        data
    }
}
