use rasengan::prelude::*;

#[test]
fn writes_to_buf() {
    let mut ring = Rasengan::<u8, 3>::new();
    ring.write(1);
    ring.write(2);
    ring.write(3);
    let r1 = ring.read();
    let r2 = ring.read();
    let r3 = ring.read();

    assert_eq!(r1, Some(1));
    assert_eq!(r2, Some(2));
    assert_eq!(r3, Some(3));
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

    assert_eq!(r1, Some(3));
    assert_eq!(r2, Some(4));
    assert_eq!(r3, Some(5));
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

    assert_eq!(r1, Some(1));
    assert_eq!(r2, Some(2));
    assert_eq!(r3, Some(4));
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

    assert_eq!(r1, Some(1));
    assert_eq!(r2, Some(2));
    assert_eq!(r3, Some(4));
    assert_eq!(r4, Some(5));
    assert_eq!(r5, Some(6));
    assert_eq!(r6, Some(4));
}

#[test]
#[should_panic]
fn panics_when_reading_empty_buffer() {
    let mut circ_buf = Rasengan::<u8, 1>::new();
    let r1 = circ_buf.read();

    assert_eq!(r1, Some(2));
}
