import heapq as pq
from sys import stdin, maxsize
from typing import TypeVar, Generic

read = lambda: stdin.readline().rstrip()

T = TypeVar("T")


class Heap(Generic[T]):
    def __init__(self, data: list[T] = []) -> None:
        self.data = data.copy()
        pq.heapify(self.data)

    def push(self, item: T):
        pq.heappush(self.data, item)
        return self

    def pop(self) -> T:
        return pq.heappop(self.data)

    def extend(self, items: list[T]):
        self.data.extend(items)
        pq.heapify(self.data)
        return self

    def __len__(self) -> int:
        return len(self.data)


num_v = int(read())
num_e = int(read())
edges = [list() for _ in range(num_v + 1)]

for _ in range(num_e):
    start, end, weight = map(int, read().split())
    edges[start].append((weight, end))

dist = [maxsize for _ in range(num_v + 1)]

start_v, end_v = map(int, read().split())

travel_pq = Heap()
travel_pq.push((0, start_v))
dist[start_v] = 0

while len(travel_pq) != 0:
    curr_dist, curr_v = travel_pq.pop()
    if curr_dist > dist[curr_v]:
        continue
    for weight, adj_v in edges[curr_v]:
        if dist[adj_v] > curr_dist + weight:
            dist[adj_v] = curr_dist + weight
            travel_pq.push((curr_dist + weight, adj_v))

print(dist[end_v])
