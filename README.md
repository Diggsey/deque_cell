deque_cell
==========

Provides an efficient wrapper around VecDeque, enabling safe internal mutability without the overhead of RefCell's runtime checks.


API
---

Method names match those of VecDeque, but mutating methods take `&self` rather than `&mut self`.
Safety is preserved by disallowing internal references to the VecDeque.


Example use
-----------

`DequeCell<T>` can be used to store a queue of actions to perform when mutable access to a resource is possible.
When mutable access is gained, each action can be popped from the queue and executed, with exclusive access to the
resource. Due to the internal mutability of the queue, these actions can themselves push additional actions onto the queue.
