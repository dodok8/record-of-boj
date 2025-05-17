# 장난을 잘 치는 토카 양

from sys import stdin
import math

read = lambda: stdin.readline().rstrip()


a, b = map(int, read().split())
c, d = map(int, read().split())
k = int(read())


def get_toka_d(n: int):
    return b * n + n * (n - 1) * (-k) // 2


def get_doll_d(n: int):
    return d * n


def get_move_cnt():
    cnt = -1

    start = 0
    end = 0

    if k == 0:
        end = math.ceil(a / b)
    else:
        end = math.floor(b / k) + 1

    while start <= end:
        mid = (start + end) // 2

        curr_toka = a - get_doll_d(mid)

        if curr_toka <= 0:
            start = start
            end = mid - 1

            cnt = mid
        else:
            start = mid + 1
            end = end

    return cnt


t = get_move_cnt()

if t == -1:
    print(-1)
else:
    curr_doll = c - get_doll_d(t)

    if curr_doll > 0:
        print(t)
    else:
        print(-1)
