// 이중 우선 순위 큐

const fs = require("fs");

type Comparator<T> = (a: T, b: T) => number;

export class Heap<T> {
  protected heap: T[] = [];
  private comparator: Comparator<T>;

  constructor(comparator: Comparator<T>) {
    this.comparator = comparator;
  }

  static heapify<T>(arr: Array<T>, comparator: Comparator<T>): Heap<T> {
    const heap = new Heap(comparator);
    heap.heap = [...arr];

    for (let i = Math.floor((arr.length - 1) / 2); i >= 0; i--) {
      heap.heapifyDownFrom(i);
    }

    return heap;
  }

  isEmpty() {
    return this.heap.length === 0;
  }

  clear() {
    this.heap = [];
  }

  peek(): T | null {
    return this.isEmpty() ? null : this.heap[0];
  }

  private swap(idx: number, jdx: number): void {
    [this.heap[idx], this.heap[jdx]] = [this.heap[jdx], this.heap[idx]];
  }

  push(val: T): void {
    this.heap.push(val);
    this.heapifyUp();
  }

  private heapifyUp(): void {
    let idx = this.heap.length - 1;
    while (
      Math.floor((idx - 1) / 2) >= 0 && this.comparator(
          this.heap[Math.floor((idx - 1) / 2)],
          this.heap[idx],
        ) > 0
    ) {
      this.swap(Math.floor((idx - 1) / 2), idx);
      idx = Math.floor((idx - 1) / 2);
    }
  }

  pop(): T | null {
    if (this.isEmpty()) {
      return null;
    } else {
      this.swap(0, this.heap.length - 1);
      const popped = this.heap.pop()!;
      this.heapifyDown();
      return popped;
    }
  }

  private heapifyDown(): void {
    this.heapifyDownFrom(0);
  }

  private heapifyDownFrom(idx: number): void {
    while (idx * 2 + 1 < this.heap.length) {
      let smallerChildIdx = idx * 2 + 1;

      if (
        idx * 2 + 2 < this.heap.length &&
        this.comparator(this.heap[idx * 2 + 2], this.heap[idx * 2 + 1]) < 0
      ) {
        smallerChildIdx = idx * 2 + 2;
      }

      if (this.comparator(this.heap[idx], this.heap[smallerChildIdx]) < 0) {
        break;
      }

      this.swap(idx, smallerChildIdx);
      idx = smallerChildIdx;
    }
  }
}

function* inputIterator<T>(arr: Array<T>) {
  for (const num of arr) {
    yield num;
  }
}

function main() {
  let input_path = "/dev/stdin";
  const input = inputIterator(
    fs.readFileSync(input_path).toString().split(/\s/) as string[],
  );
  const T = parseInt(input.next().value as string, 10);

  const cntMap: Map<number, number> = new Map();
  const maxHeap = new Heap<number>((a, b) => b - a);
  const minHeap = new Heap<number>((a, b) => a - b);

  const cleanHeaps = () => {
    while (!minHeap.isEmpty() && (cntMap.get(minHeap.peek()!) || 0) === 0) {
      minHeap.pop();
    }
    while (!maxHeap.isEmpty() && (cntMap.get(maxHeap.peek()!) || 0) === 0) {
      maxHeap.pop();
    }
  };

  for (let _tdx = 0; _tdx < T; _tdx += 1) {
    maxHeap.clear();
    minHeap.clear();
    cntMap.clear();
    let n = parseInt(input.next().value as string);

    for (let idx = 0; idx < n; idx += 1) {
      const cmd = input.next().value;
      const target = parseInt(input.next().value as string);

      if (cmd === "I") {
        // 값 삽입
        minHeap.push(target);
        maxHeap.push(target);
        cntMap.set(target, (cntMap.get(target) || 0) + 1);
      } else {
        // 값 삭제
        if (target === 1) {
          // 최댓값 삭제
          if (!maxHeap.isEmpty()) {
            const maxVal = maxHeap.peek()!;
            cntMap.set(maxVal, (cntMap.get(maxVal) || 0) - 1);
            maxHeap.pop();
          }
        } else {
          // 최솟값 삭제
          if (!minHeap.isEmpty()) {
            const minVal = minHeap.peek()!;
            cntMap.set(minVal, (cntMap.get(minVal) || 0) - 1);
            minHeap.pop();
          }
        }
        cleanHeaps();
      }
    }

    cleanHeaps();

    if (maxHeap.isEmpty() || minHeap.isEmpty()) {
      console.log("EMPTY");
    } else {
      console.log(`${maxHeap.peek()} ${minHeap.peek()}`);
    }
  }
}

main();
