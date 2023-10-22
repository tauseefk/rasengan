# Rasengan

Minimal circular buffer implementation.
Allows overwriting data once the buffer is full. Only allows reading data once.

## Write with wrap-around
<img src="https://github.com/tauseefk/rasengan/assets/11029896/68b3809b-7bae-436e-8b1c-f06b2b6ff67b" width="400" />


---
## Fork in the road
Depending on the sequence of operations there are two alternatives:

### Read with wrap-around
In this case the reader resumes, so no unread values were overwritten.

<img src="https://github.com/tauseefk/rasengan/assets/11029896/b013d8a7-0fc5-4ddb-8902-161be66b07d7" width="400" />

### Write with overwrites
In this case the reader is still busy and the writer overwrites unread values. The read pointer is then moved to the least recent values in the buffer.

<img src="https://github.com/tauseefk/rasengan/assets/11029896/52186f31-7e85-46a6-8b4b-cd8fd4b55497" width="400" />
