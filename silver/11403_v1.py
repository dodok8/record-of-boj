from sys import stdin
from collections import deque

read = lambda: stdin.readline().rstrip()

n = int(read())
board = [list(map(int, read().split())) for _ in range(n)]
result = [[0 for _ in range(n)] for __ in range(n)]

for start in range(n):
    visited = [False for _ in range(n)]
    travel_stack = deque()
    for point, is_neighbor in enumerate(board[start]):
        if is_neighbor == 1:
            travel_stack.append(point)
    while len(travel_stack) != 0:
        curr_point = travel_stack.pop()
        if visited[curr_point]:
            continue
        for point, is_neighbor in enumerate(board[curr_point]):
            if is_neighbor == 1 and not visited[point]:
                travel_stack.append(point)
        visited[curr_point] = True
    for idx, is_visited in enumerate(visited):
        result[start][idx] = 1 if is_visited else 0

for row in result:
    for item in row:
        print(item, end=" ")
    print()
