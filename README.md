# not-linear-time-sorting
My algorithm for linear time sorting (not fully fleshed out, use at risk of universe collapsing in on itself).

For a sufficiently small sleep time delta, this may not be a joke of an algorithm. The sleep time delta cannot be so small (or the input size so large) that it interrupts the first loop that spawns the threads.

#### Bookmarks
[mpsc channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html)
[mpsc receiver](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html)
[message passing between threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
[future memory exploration](https://news.ycombinator.com/item?id=42270378)