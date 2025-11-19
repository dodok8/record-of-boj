class CircularQueue:
    def __init__(self, max_size=8):
        self.front = 0
        self.rear = 0
        self.items = [None] * max_size
        self.max_size = max_size

    def isEmpty(
        self,
    ):
        return self.front == self.rear

    def isFull(self):
        return (self.rear + 1) % self.max_size == self.front

    def clear(self):
        self.front = self.rear

    def __len__(self):
        return (self.rear - self.front + self.max_size) % self.max_size

    def insert(self, item):
        if not self.isFull():
            self.rear = (self.rear + 1) % self.max_size
            self.items[self.rear] = item
        else:
            raise IndexError("용량 초과!")

    def pop(self):
        if self.isEmpty():
            raise IndexError("값이 없엉.")
        self.front = (self.front + 1) % self.max_size
        return self.items[self.front]

        pass

    def peek(self):
        if self.isEmpty():
            raise IndexError("값이 없엉.")
        return self.items[(self.front + 1) % self.max_size]

    def print(self):
        out = []
        if self.front < self.rear:
            out = self.items[self.front + 1 : self.rear + 1]
        else:
            out = self.items[self.front + 1 : self.max_size] + self.items[0 : self.rear + 1]

        print("[f=%s, r=%d] ==> " % (self.front, self.rear), out)


class CircularDeQueue:
    def __init__(self, max_size=8):
        self.front = 0
        self.rear = 0
        self.items = [None] * max_size
        self.max_size = max_size

    def isEmpty(
        self,
    ):
        return self.front == self.rear

    def isFull(self):
        return (self.rear + 1) % self.max_size == self.front

    def clear(self):
        self.front = self.rear
        pass

    def __len__(self):
        return (self.rear - self.front + self.max_size) % self.max_size

    def insert_back(self, item):
        if not self.isFull():
            self.rear = (self.rear + 1) % self.max_size
            self.items[self.rear] = item
        else:
            raise IndexError("용량 초과!")

    def insert_front(self, item):
        if not self.isFull():
            self.items[self.front] = item
            self.front = (self.front - 1 + self.max_size) % self.max_size
        else:
            raise IndexError("용량 초과!")

    def pop_front(self):
        if self.isEmpty():
            raise IndexError("값이 없엉.")
        self.front = (self.front + 1) % self.max_size
        return self.items[self.front]

    def pop_back(self):
        if self.isEmpty():
            raise IndexError("값이 없엉.")
        item = self.items[self.rear]
        self.rear = (self.rear - 1 + self.max_size) % self.max_size
        return item

    def peek_front(self):
        if self.isEmpty():
            raise IndexError("값이 없엉.")
        return self.items[(self.front + 1) % self.max_size]

    def peek_back(self):
        if self.isEmpty():
            raise IndexError("값이 없엉.")
        return self.items[self.rear % self.max_size]

    def print(self):
        out = []
        if self.front < self.rear:
            out = self.items[self.front + 1 : self.rear + 1]
        else:
            out = self.items[self.front + 1 : self.max_size] + self.items[0 : self.rear + 1]

        print("[f=%s, r=%d] ==> " % (self.front, self.rear), out)


if __name__ == "__main__":
    # CircularQueue Examples
    print("=" * 50)
    print("CircularQueue Examples")
    print("=" * 50)

    # Create a queue with max size 5
    queue = CircularQueue(5)

    # Insert elements
    queue.insert(10)
    queue.insert(20)
    queue.insert(30)
    print("After inserting 10, 20, 30:")
    queue.print()

    # Check length
    print(f"Queue length: {len(queue)}")

    # Peek at front element
    print(f"Front element (peek): {queue.peek()}")

    # Pop elements
    print(f"Popped: {queue.pop()}")
    print(f"Popped: {queue.pop()}")
    print("After popping twice:")
    queue.print()

    # Check if empty
    print(f"Is empty: {queue.isEmpty()}")

    # Insert more elements
    queue.insert(40)
    queue.insert(50)
    queue.insert(60)
    print("After inserting 40, 50, 60:")
    queue.print()

    # Check if full
    print(f"Is full: {queue.isFull()}")

    # Clear the queue
    queue.clear()
    print("After clearing:")
    queue.print()
    print(f"Is empty after clear: {queue.isEmpty()}")

    # CircularDeQueue Examples
    print("\n" + "=" * 50)
    print("CircularDeQueue Examples")
    print("=" * 50)

    # Create a deque with max size 6
    deque = CircularDeQueue(6)

    # Insert at back
    deque.insert_back(1)
    deque.insert_back(2)
    deque.insert_back(3)
    print("After insert_back 1, 2, 3:")
    deque.print()

    # Insert at front
    deque.insert_front(0)
    deque.insert_front(-1)
    print("After insert_front 0, -1:")
    deque.print()

    # Peek from both ends
    print(f"Front element (peek_front): {deque.peek_front()}")
    print(f"Back element (peek_back): {deque.peek_back()}")

    # Pop from front
    print(f"Pop from front: {deque.pop_front()}")
    print("After pop_front:")
    deque.print()

    # Pop from back
    print(f"Pop from back: {deque.pop_back()}")
    print("After pop_back:")
    deque.print()

    # Check length
    print(f"Deque length: {len(deque)}")

    # Clear and verify
    deque.clear()
    print("After clearing:")
    print(f"Is empty: {deque.isEmpty()}")
