from sys import stdin
import heapq as pq


class Heap:
    def __init__(self, data) -> None:
        self.data = data.copy()
        pq.heapify(self.data)

    def push(self, item) -> None:
        pq.heappush(self.data, item)
        return self

    def pop(self):
        return pq.heappop(self.data)

    def top(self):
        return self.data[0]

    def extend(self, items):
        self.data.extend(items)
        pq.heapify(self.data)
        return self

    def __len__(self) -> int:
        return len(self.data)


read = lambda: stdin.readline().rstrip()

num_v, min_cost = map(int, read().split())

points = list()

for _idx in range(num_v):
    points.append(list(map(int, read().split())))


def calc_cost(i, j):
    return (points[i][0] - points[j][0]) ** 2 + (
        points[i][1] - points[j][1]
    ) ** 2


contained = [False for _ in range(num_v)]
edges_pq = Heap([(0, 0)])

dist = 0

while len(edges_pq) > 0:
    curr_v, curr_w = edges_pq.pop()
    if contained[curr_v]:
        continue
    contained[curr_v] = True
    dist += curr_w
    for next_v in range(num_v):
        if not contained[next_v]:
            next_w = calc_cost(next_v, curr_v)
            if next_w > min_cost:
                edges_pq.push((next_v, next_w))

print(dist)
