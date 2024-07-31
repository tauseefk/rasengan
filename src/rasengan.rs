pub struct Rasengan<T, const N: usize> {
    buf: [Option<T>; N],
    read_ptr: usize,
    write_ptr: usize,
}

impl<T: Copy, const N: usize> Rasengan<T, N> {
    pub fn new() -> Self {
        Rasengan::new_const_default()
    }

    pub const fn new_const_default() -> Self {
        Self {
            buf: [None; N],
            read_ptr: 1,
            write_ptr: 0,
        }
    }

    fn will_overwrite_unread_data(&self) -> bool {
        self.write_ptr + 1 == self.read_ptr + self.buf.len()
    }

    /// Overwrites when buffer is full
    ///
    /// ```
    /// use rasengan::Rasengan;
    /// let mut circ_buf = Rasengan::<u8, 3>::new();
    /// circ_buf.write(1);
    /// circ_buf.write(2);
    /// circ_buf.write(3);
    /// circ_buf.write(4);
    /// circ_buf.write(5); // read ptr gets moved to 3 as that's the oldest value left
    /// let r1 = circ_buf.read();
    /// let r2 = circ_buf.read();
    /// let r3 = circ_buf.read();
    ///
    /// assert_eq!(r1, Some(3));
    /// assert_eq!(r2, Some(4));
    /// assert_eq!(r3, Some(5));
    /// ```
    ///
    /// ```
    /// use rasengan::Rasengan;
    /// let mut circ_buf = Rasengan::<u8, 3>::new();
    /// circ_buf.write(1);
    /// let r1 = circ_buf.read();
    /// circ_buf.write(2);
    /// circ_buf.write(3);
    /// let r2 = circ_buf.read();
    /// circ_buf.write(4);
    /// circ_buf.write(5);
    /// circ_buf.write(6); // read ptr gets moved to 4 as that's the oldest value left
    /// let r3 = circ_buf.read();
    ///
    /// assert_eq!(r1, Some(1));
    /// assert_eq!(r2, Some(2));
    /// assert_eq!(r3, Some(4));
    pub fn write(&mut self, data: T) {
        if self.will_overwrite_unread_data() {
            self.read_ptr += 1;
        }

        self.write_ptr += 1;
        self.buf[self.write_ptr % self.buf.len()] = Some(data);
    }

    /// Read previously unread data.
    ///
    /// ```
    /// use rasengan::Rasengan;
    ///
    /// let mut circ_buf = Rasengan::<u8, 3>::new();
    /// circ_buf.write(1);
    /// let r1 = circ_buf.read();
    /// circ_buf.write(2);
    /// circ_buf.write(3);
    /// let r2 = circ_buf.read();
    /// circ_buf.write(4);
    /// circ_buf.write(5);
    /// circ_buf.write(6); // read ptr gets moved to 4 as that's the oldest value left
    /// let r3 = circ_buf.read();
    ///
    /// assert_eq!(r1, Some(1));
    /// assert_eq!(r2, Some(2));
    /// assert_eq!(r3, Some(4));
    /// ```
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

impl<T: Copy + PartialEq, const N: usize> Rasengan<T, N> {
    /// Writes to the buffer if the last element is not the same.
    /// Overwrites when buffer is full.
    ///
    /// ```
    /// use rasengan::Rasengan;
    ///
    /// let mut circ_buf = Rasengan::<u8, 3>::new();
    /// circ_buf.write_unique(1);
    /// circ_buf.write_unique(2);
    /// circ_buf.write_unique(2);
    /// let r1 = circ_buf.read();
    /// let r2 = circ_buf.read();
    /// let r3 = circ_buf.read();

    /// assert_eq!(r1, Some(1));
    /// assert_eq!(r2, Some(2));
    /// assert_eq!(r3, None);
    /// ```
    pub fn write_unique(&mut self, data: T) {
        if self.write_ptr == 0 {
            self.write(data);
            return;
        }

        let idx = self.write_ptr % self.buf.len();
        let last_written = self.buf[idx];

        if let Some(last_written) = last_written {
            if data != last_written {
                self.write(data);
            }
        }
    }
}
