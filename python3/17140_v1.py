# 이차원 배열과 연산

from sys import stdin
from collections import defaultdict


def read():
    return stdin.readline().rstrip()


def calc_r(matrix):
    new_matrix = list()
    temp_matrix = list()
    max_len = 0
    for idx in range(len(matrix)):
        cnt = defaultdict(int)
        new_row = list()
        temp_list = list()

        for num in matrix[idx]:
            if num != 0:
                cnt[num] += 1

        for key, value in cnt.items():
            temp_list.append([key, value])

        temp_list.sort(key=lambda x: [x[1], x[0]])
        new_row = sum(temp_list, [])

        if len(new_row) > 100:
            new_row = new_row[:100]
            max_len = 100
        else:
            if max_len < len(new_row):
                max_len = len(new_row)

        temp_matrix.append(new_row)

    for temp in temp_matrix:
        temp.extend([0 for i in range(max_len - len(temp))])
        new_matrix.append(temp)

    return new_matrix


def calc_c(matrix):
    transposed = list(map(list, zip(*matrix)))
    result = calc_r(transposed)
    return list(map(list, zip(*result)))


def check(matrix, r, c, k):
    # 배열 크기를 벗어나지 않는지 확인
    if r < len(matrix) and c < len(matrix[0]):
        return matrix[r][c] == k
    return False


r, c, k = map(int, read().split())
r -= 1
c -= 1

matrix = list()
for _ in range(3):
    matrix.append(list(map(int, read().split())))

# 초기 상태 확인
if check(matrix, r, c, k):
    print(0)
else:
    t = 0
    valid = False
    while t < 100:
        if len(matrix) >= len(matrix[0]):
            matrix = calc_r(matrix)
        else:
            matrix = calc_c(matrix)
        t += 1

        if check(matrix, r, c, k):
            valid = True
            break

    if valid:
        print(t)
    else:
        print(-1)
