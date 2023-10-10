# 최소비용 구하기 2

import heapq as pq
from collections import deque
from sys import stdin, maxsize
from typing import TypeVar, Generic, TypedDict, TypeAlias

read = lambda: stdin.readline().rstrip()

T = TypeVar("T")
Weight: TypeAlias = int
Vertex: TypeAlias = int
Edge: TypeAlias = tuple[Weight, Vertex]
ResultDict = TypedDict("PathResult", {"path": list[Vertex], "weight": Weight})


class NotConnectedGraphException(Exception):
    pass


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


def get_shortest_path(start_v: Vertex, end_v: Vertex, edges: list[list[Edge]]):
    smallest_weights: list[tuple[Weight, Vertex]] = [
        (maxsize, -1) for _ in range(len(edges))
    ]
    smallest_weights[start_v] = (0, None)

    travel_pq = Heap[tuple[Weight, Vertex]]([(0, start_v)])

    while len(travel_pq) != 0:
        curr_weight, curr_v = travel_pq.pop()
        if curr_weight > smallest_weights[curr_v][0]:
            continue
        for weight, adj_v in edges[curr_v]:
            next_weight = curr_weight + weight
            if smallest_weights[adj_v][0] > next_weight:
                smallest_weights[adj_v] = (next_weight, curr_v)
                travel_pq.push((next_weight, adj_v))
    if smallest_weights[end_v][1] == -1:
        raise NotConnectedGraphException

    curr_v = end_v
    path_deque = deque()
    while curr_v is not None:
        path_deque.appendleft(curr_v)
        curr_v = smallest_weights[curr_v][1]

    result_dict: ResultDict = {
        "path": list(path_deque),
        "weight": smallest_weights[end_v][0],
    }

    return result_dict


num_v = int(read())
num_e = int(read())
edges = [list() for _ in range(num_v + 1)]

for _ in range(num_e):
    start, end, weight = map(int, read().split())
    edges[start].append((weight, end))

start_v, end_v = map(int, read().split())

result = get_shortest_path(start_v, end_v, edges)

print(result["weight"])
print(len(result["path"]))
for vertex in result["path"]:
    print(vertex, end=" ")
