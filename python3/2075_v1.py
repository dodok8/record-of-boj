# N번째 큰 수

import heapq
from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())

heap = list(map(int, read().split()))
heapq.heapify(heap)  # n개가 저장된 힙

for _idx in range(n - 1):
    for num in map(int, read().split()):
        if heap[0] < num:
            heapq.heappush(heap, num)
            heapq.heappop(heap)

print(heap[0])
