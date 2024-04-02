# Rasengan

Minimal circular buffer implementation.
Allows overwriting data once the buffer is full. Only allows reading data once.

## Write with wrap-around
```
           W       R                                                                
           │       │                                                                
       ╔═══▼═══╦═══▼═══╦═══════╦═══════╦───────┬───────┐                            
       ║       ║       ║       ║       ║       │       │▐▌                          
       ║       ║       ║       ║       ║       │       │▐▌                          
       ╚═══▲═══╩═══════╩═══════╩═══╤═══╩───────┴───────┘▐▌                          
        ▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘                          
           └─── Capacity = 4 ──────┘                                                
                                                                                    
       W < R // buffer has no unread values                                         
       read() -> panic("Nothing to read here")                                      
                                                                                    
           W       R                                                                
           │       │                                                                
       ╔═══▼═══╦═══▼═══╦═══════╦═══════╦───────┬───────┐                            
       ║       ║       ║       ║       ║       │       │▐▌                          
       ║       ║   5   ║       ║       ║       │       │▐▌                          
       ╚═══▲═══╩═══════╩═══════╩═══╤═══╩───────┴───────┘▐▌                          
        ▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘                          
           └───────────────────────┘                                                
                                                                                    
       inc(W); write(5); // increment W before writing                              
                                                                                    
           W       R                                                                
           │       │                                                                
       ╔═══▼═══╦═══▼═══╦═══════╦═══════╦───────┬───────┐                            
       ║       ║       ║       ║       ║       │       │▐▌                          
       ║       ║   5   ║       ║       ║       │       │▐▌                          
       ╚═══▲═══╩═══════╩═══════╩═══╤═══╩───────┴───────┘▐▌                          
        ▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘                          
           └───────────────────────┘                                                
                                                                                    
       read() -> 5; inc(R); // read before incrementing R                           
       //  5 has been read and cannot be re-read                                    
                                                                                    
           w       R                       W                                        
           │       │                       │                                        
       ╔═══▼═══╦═══▼═══╦═══════╦═══════╦───▼───┬───────┐                            
       ║       ║       ║       ║       ║       │       │▐▌                          
       ║   8   ║   5   ║   2   ║   4   ║       │       │▐▌                          
       ╚═══▲═══╩═══════╩═══════╩═══╤═══╩───────┴───────┘▐▌                          
        ▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘                          
           └───────────────────────┘                                                
                                                                                    
       inc(W); write(2); inc(W); write(4); inc(W); write(8);                        
       w = W % capacity // write with wrap-around                                   
                                                                                    

```

---
## Fork in the road
Depending on the sequence of operations there are two alternatives:

### Read with wrap-around
In this case the reader resumes, so no unread values were overwritten.

```
                                                                                    
           w       r                       W       R                                
           │       │                       │       │                                
       ╔═══▼═══╦═══▼═══╦═══════╦═══════╦───▼───┬───▼───┐                            
       ║       ║       ║       ║       ║       │       │▐▌                          
       ║   8   ║   5   ║   2   ║   4   ║       │       │▐▌                          
       ╚═══▲═══╩═══════╩═══════╩═══╤═══╩───────┴───────┘▐▌                          
        ▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀                           
           └───────────────────────┘                                                
                                                                                    
       read() -> 2; inc(R);                                                         
       read() -> 4; inc(R);                                                         
       read() -> 8; inc(R);                                                         
       r = R % capacity // read with wrap-around                                    
       w < R // buffer has no unread values                                         
       read() -> panic("Nothing to read here.")                                     
                                                                                    

```

### Write with overwrites
In this case the reader is still busy and the writer overwrites unread values. The read pointer is then moved to the least recent values in the buffer.

```
                                                           ╷                        
                             ├───────╴capacity - 1╶────────┤                        
                         w  R,r                            W                        
                         │   │                             │                        
       ╔═══════╦═══════╦═▼═══▼═╦═══════╦───────┬───────┬───▼───┐                    
       ║       ║       ║       ║       ║       │       │       │▐▌                  
       ║   8   ║   7   ║  10   ║   4   ║       │       │       │▐▌                  
       ╚═══▲═══╩═══════╩═══════╩═══╤═══╩───────┴───────┴───────┘▐▌                  
        ▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀                   
           └───────────────────────┘                                                
                                                                                    
       inc(W); write(7); inc(W); write(10);                                         
       W - R = capacity - 1                                                         
       // unread values at capacity, next write will overwrite unread values        
       inc(R) // move to least recent value in the buffer                           
                                                                                    
                                                                                    
                           w      R,r                      W                        
                           │       │                       │                        
       ╔═══════╦═══════╦═══▼═══╦═══▼═══╦───────┬───────┬───▼───┐                    
       ║       ║       ║       ║       ║       │       │       │▐▌                  
       ║   8   ║   7   ║  10   ║   4   ║       │       │       │▐▌                  
       ╚═══▲═══╩═══════╩═══════╩═══╤═══╩───────┴───────┴───────┘▐▌                  
        ▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀│▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀                   
           └───────────────────────┘                                                
       Final Configuration                                                          
```
