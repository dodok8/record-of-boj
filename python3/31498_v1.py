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
    # 언제 집에 도착하는지 판단 하는 함수
    cnt = -1

    start = 0
    end = 0

    if k == 0:
        end = math.ceil(a / b)
    else:
        end = math.floor(b / k)

    while start <= end:
        mid = (start + end) // 2

        # print(f"start: {start} end: {end} mid: {mid}")

        curr_toka = a - get_toka_d(mid)

        # print(f"curr_toka: {curr_toka}")

        if curr_toka <= 0:
            # 토카가 집에 도착한 경우이니 더 작은 범위에서 탐색
            cnt = mid

            start = start
            end = mid - 1
        else:
            start = mid + 1
            end = end
    return cnt


t = get_move_cnt()

if t == -1:
    print(-1)
else:
    curr_doll = a + c - get_doll_d(t)
    if curr_doll > 0:
        print(t)
    else:
        print(-1)
