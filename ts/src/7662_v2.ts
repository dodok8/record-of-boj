const fs = require("fs");

class MinMaxHeap<T> {
  protected heap: Array<T | undefined> = [undefined];

  private swap(idx: number, jdx: number): void {
    [this.heap[idx], this.heap[jdx]] = [this.heap[jdx], this.heap[idx]];
  }

  clear() {
    this.heap = [undefined];
  }

  isEmpty() {
    return this.heap.length == 1;
  }

  size() {
    return this.heap.length - 1;
  }

  constructor(v?: Array<T>) {
    if (v == null) {
      this.heap = [undefined];
      return;
    }
    this.heap = [undefined, ...v];

    for (let idx = Math.floor((this.heap.length - 1) / 2); idx >= 1; idx--) {
      this.heapifyDown(idx);
    }
  }

  push(val: T) {
    this.heap.push(val);
    this.bubbleUp(this.heap.length - 1);
  }

  private bubbleUp(idx: number) {
    if (idx <= 1) return;

    if (idx == 2) {
      if (this.heap[1]! > this.heap[2]!) {
        this.swap(1, 2);
      }
      return;
    }

    const parent = Math.floor(idx / 2);

    if (idx % 2 == 1) {
      if (this.heap[idx]! > this.heap[parent]!) {
        this.swap(idx, parent);
        this.bubbleUpInMaxTree(parent);
      } else {
        this.bubbleUpInMinTree(idx);
      }
    } else {
      if (this.heap[idx]! < this.heap[parent]!) {
        this.swap(idx, parent);
        this.bubbleUpInMinTree(parent);
      } else {
        this.bubbleUpInMaxTree(idx);
      }
    }
  }

  private bubbleUpInMinTree(idx: number) {
    while (idx > 1) {
      if (this.heap[idx]! < this.heap[1]!) {
        this.swap(idx, 1);
        idx = 1;
      }

      const parent = Math.floor(idx / 2);
      const grandparent = Math.floor(parent / 2);

      if (grandparent < 1) break;

      if (this.heap[idx]! < this.heap[grandparent]!) {
        this.swap(idx, grandparent);
        idx = grandparent;
      } else {
        break;
      }
    }
  }

  private bubbleUpInMaxTree(idx: number) {
    while (idx > 2) {
      if (this.heap.length > 2 && this.heap[idx]! > this.heap[2]!) {
        this.swap(idx, 2);
        idx = 2;
      }

      const parent = Math.floor(idx / 2);
      const grandparent = Math.floor(parent / 2);

      if (grandparent < 2) break;

      if (this.heap[idx]! > this.heap[grandparent]!) {
        this.swap(idx, grandparent);
        idx = grandparent;
      } else {
        break;
      }
    }
  }

  peekMin(): T | undefined {
    return this.heap[1];
  }

  peekMax(): T | undefined {
    return this.heap.length > 2 ? this.heap[2] : this.heap[1];
  }

  popMin(): T | undefined {
    if (this.isEmpty()) return undefined;

    const val = this.heap[1];
    const last = this.heap.pop()!;

    if (!this.isEmpty()) {
      this.heap[1] = last;
      this.heapifyDown(1);
    }

    return val;
  }

  popMax(): T | undefined {
    if (this.isEmpty()) return undefined;

    if (this.heap.length == 2) {
      return this.heap.pop();
    }

    const val = this.heap[2];
    const last = this.heap.pop()!;

    if (this.heap.length > 2) {
      this.heap[2] = last;
      this.heapifyDown(2);
    }

    return val;
  }

  private heapifyDown(idx: number) {
    if (idx <= 0 || idx >= this.heap.length) return;

    if (idx % 2 == 1) {
      this.heapifyDownMin(idx);
    } else {
      this.heapifyDownMax(idx);
    }
  }

  private heapifyDownMin(idx: number) {
    while (true) {
      let smallest = idx;
      const leftChild = 2 * idx;
      const rightChild = 2 * idx + 1;

      if (
        leftChild < this.heap.length &&
        this.heap[leftChild]! < this.heap[smallest]!
      ) {
        smallest = leftChild;
      }
      if (
        rightChild < this.heap.length &&
        this.heap[rightChild]! < this.heap[smallest]!
      ) {
        smallest = rightChild;
      }

      if (smallest == idx) break;

      this.swap(idx, smallest);

      if (smallest % 2 == 0 && smallest > 2) {
        const sibling = smallest - 1;
        if (this.heap[smallest]! > this.heap[sibling]!) {
          this.swap(smallest, sibling);
          idx = sibling;
          continue;
        }
      }

      idx = smallest;
    }
  }

  private heapifyDownMax(idx: number) {
    while (true) {
      let largest = idx;
      const leftChild = 2 * idx;
      const rightChild = 2 * idx + 1;

      if (
        leftChild < this.heap.length &&
        this.heap[leftChild]! > this.heap[largest]!
      ) {
        largest = leftChild;
      }
      if (
        rightChild < this.heap.length &&
        this.heap[rightChild]! > this.heap[largest]!
      ) {
        largest = rightChild;
      }

      if (largest == idx) break;

      this.swap(idx, largest);

      if (largest % 2 == 1 && largest > 1) {
        const sibling = largest + 1;
        if (
          sibling < this.heap.length &&
          this.heap[largest]! < this.heap[sibling]!
        ) {
          this.swap(largest, sibling);
          idx = sibling;
          continue;
        }
      }

      idx = largest;
    }
  }
}

function main() {
  let input_path = "/dev/stdin";
  try {
    if (process.env.LOCAL) {
      input_path = "/workspaces/record-of-boj/input/7662_v2";
    }
  } catch {}

  const input = fs.readFileSync(input_path).toString().trim().split("\n");
  let idx = 0;

  const T = parseInt(input[idx++]);
  const results: string[] = [];

  for (let t = 0; t < T; t++) {
    const k = parseInt(input[idx++]);
    const heap = new MinMaxHeap<number>();

    for (let i = 0; i < k; i++) {
      const line = input[idx++].split(" ");
      const op = line[0];
      const value = parseInt(line[1]);

      if (op === "I") {
        heap.push(value);
      } else if (op === "D") {
        if (!heap.isEmpty()) {
          if (value === 1) {
            heap.popMax();
          } else {
            heap.popMin();
          }
        }
      }
    }

    if (heap.isEmpty()) {
      results.push("EMPTY");
    } else {
      results.push(`${heap.peekMax()} ${heap.peekMin()}`);
    }
  }

  console.log(results.join("\n"));
}

main();
