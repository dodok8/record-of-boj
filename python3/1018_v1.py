# 체스판 다시 칠하기

from sys import stdin

read = lambda: stdin.readline().rstrip()

black_board = [["B", "W"], ["W", "B"]]
white_board = [["W", "B"], ["B", "W"]]

n, m = map(int, read().split())

chess_board = [list(read()) for _ in range(n)]

min_ans = 65  # 64칸을 다 칠한 것 보다 더 큰 케이스 존재 X

for x in range(n):
    for y in range(m):
        if x + 7 >= n or y + 7 >= m:
            continue
        black_ans = 0
        white_ans = 0
        for idx in range(8):
            for jdx in range(8):
                if chess_board[x + idx][y + jdx] != black_board[idx % 2][jdx % 2]:
                    black_ans += 1
                if chess_board[x + idx][y + jdx] != white_board[idx % 2][jdx % 2]:
                    white_ans += 1
        if min_ans > black_ans:
            min_ans = black_ans
        if min_ans > white_ans:
            min_ans = white_ans

print(min_ans)
