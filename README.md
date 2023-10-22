# Rasengan

Minimal circular buffer implementation.
Allows overwriting data once the buffer is full. Only allows reading data once.

## Write with wrap-around
![circular_buffer_with_overflow](https://github.com/tauseefk/rasengan/assets/11029896/33d0cb17-74bd-4b1f-8683-981f894fef10)

---
## Fork in the road
Depending on the sequence of operations there are two alternatives:

### Read with wrap-around
In this case the reader resumes, so no unread values were overwritten.

![circular_buffer_with_no_overwrite](https://github.com/tauseefk/rasengan/assets/11029896/b643e0ff-e764-4462-ad3d-ff122f3a01e3)

### Write with overwrites
In this case the reader is still busy and the writer overwrites unread values. The read pointer is then moved to the least recent values in the buffer.

![circular_buffer_with_overwrite](https://github.com/tauseefk/rasengan/assets/11029896/f21b353d-cbe6-471e-b5d3-98e8f329edba)


