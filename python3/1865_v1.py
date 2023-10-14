# 웜홀

import heapq as pq
from sys import stdin, maxsize
from typing import TypeVar, Generic, TypeAlias

read = lambda: stdin.readline().rstrip()

T = TypeVar("T")
Weight: TypeAlias = int
Vertex: TypeAlias = int
Edge: TypeAlias = tuple[Weight, Vertex]


class NotConnectedGraphException(Exception):
    pass


class MinusCycleException(Exception):
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


def get_smallest_weight_bellman_ford(
    start_v: Vertex, end_v: Vertex, edges: list[list[Edge]]
):
    num_v = len(edges) - 1
    upper_bound: list[Weight] = [maxsize for _ in range(num_v + 1)]
    upper_bound[start_v] = 0
    is_updated = False
    for count in range(num_v):
        is_updated = False
        for curr_v in range(1, num_v + 1):
            for weight, adj_v in edges[curr_v]:
                if upper_bound[adj_v] > upper_bound[curr_v] + weight:
                    if (
                        upper_bound[adj_v] == maxsize
                        and upper_bound[curr_v] == maxsize
                    ):
                        continue
                    upper_bound[adj_v] = upper_bound[curr_v] + weight
                    is_updated = True
        if not is_updated:
            break
        if is_updated and count == num_v - 1:
            raise MinusCycleException
    if upper_bound[end_v] == maxsize:
        raise NotConnectedGraphException
    return upper_bound[end_v]


for _ in range(int(read())):
    num_v, num_road, num_wormhole = map(int, read().split())
    edges: list[list[Edge]] = [list() for _ in range(num_v + 1)]
    for _ in range(num_road):
        start, end, weight = map(int, read().split())
        edges[start].append((weight, end))
        edges[end].append((weight, start))
    for _ in range(num_wormhole):
        start, end, abs_weight = map(int, read().split())
        edges[start].append((-abs_weight, end))

    is_possible = False

    for start_v in range(1, num_v + 1):
        is_possible = False
        end_v = start_v
        try:
            smallest_weight = get_smallest_weight_bellman_ford(
                start_v, end_v, edges
            )
            if smallest_weight < 0:
                is_possible = True
                break
        except MinusCycleException:
            is_possible = True
            break
            # 가능하면 탐사할 더 탐사할 필요가 없음.
    if is_possible:
        print("YES")
    else:
        print("NO")