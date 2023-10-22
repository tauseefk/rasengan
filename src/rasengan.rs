#[allow(dead_code)]
struct Rasengan<T: Copy, const N: usize> {
    buf: [Option<T>; N],
    read_ptr: usize,
    write_ptr: usize,
}

#[allow(dead_code)]
impl<T: Copy, const N: usize> Rasengan<T, N> {
    fn new() -> Self {
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

    pub fn read(&mut self) -> T {
        // this relies on the fact that read will always lead write
        if self.write_ptr < self.read_ptr {
            panic!("No unread data available.");
        }

        let data = self.buf[self.read_ptr % self.buf.len()];
        self.read_ptr += 1;

        match data {
            Some(val) => val,
            None => unimplemented!(),
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
fn panics_on_multiple_reads_of_same_data() {
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

    let r4 = circ_buf.read();
    let r5 = circ_buf.read();
    let r6 = circ_buf.read();

    assert_eq!(r1, 1);
    assert_eq!(r2, 2);
    assert_eq!(r3, 4);
    assert_eq!(r4, 5);
    assert_eq!(r5, 6);
    assert_eq!(r6, 4);
}

#[test]
#[should_panic]
fn panics_when_reading_empty_buffer() {
    let mut circ_buf = Rasengan::<u8, 1>::new();
    let r1 = circ_buf.read();

    assert_eq!(r1, 2);
}
