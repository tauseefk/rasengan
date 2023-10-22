#[allow(dead_code)]
struct Rasengan<T: Sized + Copy, const N: usize> {
    buf: [Option<T>; N],
    read_ptr: usize,
    write_ptr: usize,
}

#[allow(dead_code)]
impl<T: Sized + Copy, const N: usize> Rasengan<T, N> {
    fn new() -> Self {
        Self {
            buf: [None; N],
            read_ptr: 0,
            write_ptr: 0,
        }
    }

    fn wrapping_increment(&self, idx: usize) -> usize {
        (idx + 1) % self.buf.len()
    }

    // Overwrites when buffer is full
    pub fn write(&mut self, data: T) {
        let should_update_read_ptr =
            self.write_ptr == self.read_ptr && self.buf[self.write_ptr].is_some();

        self.buf[self.write_ptr] = Some(data);
        self.write_ptr = self.wrapping_increment(self.write_ptr);

        if should_update_read_ptr {
            self.read_ptr = self.wrapping_increment(self.read_ptr);
        }
    }

    pub fn read(&mut self) -> T {
        let data = self.buf[self.read_ptr];

        match data {
            Some(val) => {
                self.read_ptr = self.wrapping_increment(self.read_ptr);
                val
            }
            None => panic!("Buffer is empty."),
        }
    }
}

#[test]
fn writes_to_buf() {
    let mut ring = Rasengan::<u8, 3>::new();
    ring.write(1);
    ring.write(2);
    ring.write(3);
    let r1 = ring.read();
    let r2 = ring.read();
    let r3 = ring.read();

    assert_eq!(r1, 1);
    assert_eq!(r2, 2);
    assert_eq!(r3, 3);
}

#[test]
fn writes_to_buf_with_overlap() {
    let mut circ_buf = Rasengan::<u8, 3>::new();
    circ_buf.write(1);
    circ_buf.write(2);
    circ_buf.write(3);
    circ_buf.write(4);
    circ_buf.write(5); // read ptr gets moved to 3 as that's the oldest value left
    let r1 = circ_buf.read();
    let r2 = circ_buf.read();
    let r3 = circ_buf.read();

    assert_eq!(r1, 3);
    assert_eq!(r2, 4);
    assert_eq!(r3, 5);
}

#[test]
fn interleaved_write_reads() {
    let mut circ_buf = Rasengan::<u8, 3>::new();
    circ_buf.write(1);
    let r1 = circ_buf.read();
    circ_buf.write(2);
    circ_buf.write(3);
    let r2 = circ_buf.read();
    circ_buf.write(4);
    circ_buf.write(5);
    circ_buf.write(6); // read ptr gets moved to 4 as that's the oldest value left
    let r3 = circ_buf.read();

    assert_eq!(r1, 1);
    assert_eq!(r2, 2);
    assert_eq!(r3, 4);
}

#[test]
#[should_panic]
fn panics_when_reading_empty_buffer() {
    let mut circ_buf = Rasengan::<u8, 1>::new();
    let r1 = circ_buf.read();

    assert_eq!(r1, 2);
}
