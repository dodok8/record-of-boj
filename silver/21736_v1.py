from sys import stdin
from collections import deque


def get_connected_points(point: tuple[int, int]) -> list[tuple[int, int]]:
    global board, m, n, visited
    connected_points = []
    x = point[0]
    y = point[1]
    for x0, y0 in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        if (0 <= x + x0 < n) and (0 <= y + y0 < m):
            if not visited[x + x0][y + y0] and board[x + x0][y + y0] != "X":
                connected_points.append((x + x0, y + y0))
                visited[x + x0][y + y0] = True
    return connected_points


def get_start_point() -> list[tuple[int, int]]:
    global board
    result = []
    for idx, col in enumerate(board):
        for jdx, item in enumerate(col):
            if item == "I":
                result.append((idx, jdx))
    return result


read = lambda: stdin.readline().rstrip()

n, m = map(int, read().split())
board = [list(read()) for _ in range(n)]
visited = [[False for __ in range(m)] for _ in range(n)]
start_points = get_start_point()
travel_queue = deque()


for start_point in start_points:
    travel_queue.append(start_point)
    visited[start_point[0]][start_point[1]] = True
number_of_person = 0

while len(travel_queue) != 0:
    curr_point = travel_queue.popleft()
    if board[curr_point[0]][curr_point[1]] == "P":
        number_of_person += 1
    connected_points = get_connected_points(curr_point)
    travel_queue.extend(connected_points)


if number_of_person == 0:
    print("TT")
else:
    print(number_of_person)
