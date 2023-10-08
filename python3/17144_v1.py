from sys import stdin
from copy import deepcopy

Point = tuple[int, int]
Board = list[list[int]]
read = lambda: stdin.readline().rstrip()


def get_connected_points(point: Point, board: Board) -> list[Point]:
    r = len(board)
    c = len(board[0])
    x = point[0]
    y = point[1]

    connected_points = []
    for x0, y0 in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        if (0 <= x + x0 < r) and (0 <= y + y0 < c):
            if board[x + x0][y + y0] != -1:
                connected_points.append((x + x0, y + y0))
    return connected_points


def get_purifier_head(board: Board) -> Point:
    for idx, col in enumerate(board):
        for jdx, item in enumerate(col):
            if item == -1:
                return (idx, jdx)


def spread_dust(prev_board: Board) -> Board:
    r = len(prev_board)
    c = len(prev_board[0])

    curr_board = [[0 for __ in range(c)] for _ in range(r)]
    for idx in range(r):
        for jdx in range(c):
            if prev_board[idx][jdx] == -1:
                curr_board[idx][jdx] = -1
                continue
            connected_points = get_connected_points((idx, jdx), prev_board)
            curr_board[idx][jdx] = (
                curr_board[idx][jdx]
                + prev_board[idx][jdx]
                - (prev_board[idx][jdx] // 5) * len(connected_points)
            )
            for point in connected_points:
                curr_board[point[0]][point[1]] = curr_board[point[0]][
                    point[1]
                ] + (prev_board[idx][jdx] // 5)
    return curr_board


def purify_dust(prev_board: Board) -> Board:
    r = len(prev_board)
    c = len(prev_board[0])

    curr_board = deepcopy(prev_board)
    purifier_head = get_purifier_head(prev_board)
    curr_board[purifier_head[0]][purifier_head[1]] = -1
    curr_board[purifier_head[0]][purifier_head[1] + 1] = 0

    for idx in range(purifier_head[1] + 1, c - 1):
        curr_board[purifier_head[0]][idx + 1] = prev_board[purifier_head[0]][
            idx
        ]
    for idx in range(purifier_head[0], 0, -1):
        curr_board[idx - 1][c - 1] = prev_board[idx][c - 1]
    for idx in range(c - 1, 0, -1):
        curr_board[0][idx - 1] = prev_board[0][idx]
    for idx in range(0, purifier_head[0]):
        if curr_board[idx + 1][0] == -1:
            continue
        curr_board[idx + 1][0] = prev_board[idx][0]
    for idx in range(0, purifier_head[1]):
        if curr_board[purifier_head[0]][idx + 1] == -1:
            continue
        curr_board[purifier_head[0]][idx + 1] = prev_board[purifier_head[0]][
            idx
        ]

    purifier_tail = (purifier_head[0] + 1, purifier_head[1])
    curr_board[purifier_tail[0]][purifier_tail[1]] = -1
    curr_board[purifier_tail[0]][purifier_tail[1] + 1] = 0
    for idx in range(purifier_tail[1] + 1, c - 1):
        curr_board[purifier_tail[0]][idx + 1] = prev_board[purifier_tail[0]][
            idx
        ]
    for idx in range(purifier_tail[0], r - 1):
        curr_board[idx + 1][c - 1] = prev_board[idx][c - 1]
    for idx in range(c - 1, 0, -1):
        curr_board[r - 1][idx - 1] = prev_board[r - 1][idx]
    for idx in range(r - 1, purifier_tail[0], -1):
        if curr_board[idx - 1][0] == -1:
            continue
        curr_board[idx - 1][0] = prev_board[idx][0]
    for idx in range(0, purifier_tail[1]):
        if curr_board[purifier_tail[0]][idx + 1] == -1:
            continue
        curr_board[purifier_tail[0]][idx + 1] = prev_board[purifier_tail[0]][
            idx
        ]
    return curr_board


r, c, t = map(int, read().split())
init_board = [list(map(int, read().split())) for _ in range(r)]
# result_board = spread_dust(init_board)

result_board = purify_dust(spread_dust(init_board))
curr_t = 1
while curr_t < t:
    result_board = purify_dust(spread_dust(result_board))
    curr_t += 1
sum = 0

# print("--------------")
# for row in result_board:
#     for item in row:
#         print(item, end=" ")
#     print()

for row in result_board:
    for item in row:
        sum += item

print(sum + 2)
