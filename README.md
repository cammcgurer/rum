Cameron McGurer and Tyler Makhoul

Recieved helps from the TAs during their hours

We believe verything has been implemented correctly

We did not end up needing a manager file because and we moved more code into the data file with our memory/state struct

We used three modules, data.rs, funcs.rs and main.rs. Our main module gets the filename, instantiates our Um and it callsour execute funtion (from data) in a loop (it uses data.rs). Our data module has a struct that holds our reigsters, memory segments, program_counter, and queue; it holds our get_instructions, mask, get, and execute functions (it uses funcs.rs). Our funcs module has all of our instruction execution functions and uses data.rs

real    0m5.808s
user    0m6.012s
sys     0m0.084s
We ran cargo build then time cargo run --release -- midmark.um and created a counter that was incremented after an execution took place and when it reached 50,000,000 it broke out of the while loop that was calling the executions

We spent 5-10 hours analyzing and preparing the design for this assignment

We have spent around 20 hours onthe actual implementation