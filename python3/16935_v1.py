# 배열 돌리기 3

from sys import stdin


def read():
    return stdin.readline().rstrip()


def convert_1():
    global matrix
    for idx in range(n // 2):
        matrix[idx], matrix[n - 1 - idx] = matrix[n - 1 - idx], matrix[idx]


def convert_2():
    global matrix
    for idx in range(n):
        for jdx in range(m // 2):
            (
                matrix[idx][jdx],
                matrix[idx][m - 1 - jdx],
            ) = (
                matrix[idx][m - 1 - jdx],
                matrix[idx][jdx],
            )


def convert_3():
    global m, n, matrix
    temp = [[0] * n for _ in range(m)]
    for idx in range(n):
        for jdx in range(m):
            temp[jdx][n - 1 - idx] = matrix[idx][jdx]

    n, m = m, n
    matrix = temp


def convert_4():
    global m, n, matrix
    temp = [[0] * n for _ in range(m)]
    for idx in range(n):
        for jdx in range(m):
            temp[m - 1 - jdx][idx] = matrix[idx][jdx]

    n, m = m, n
    matrix = temp


def convert_5():
    global m, n, matrix
    temp = [[0] * m for _ in range(n)]

    # 1 to 2

    for idx in range(n // 2):
        for jdx in range(m // 2):
            temp[idx][jdx + m // 2] = matrix[idx][jdx]

    # 2 to 3

    for idx in range(n // 2):
        for jdx in range(m // 2, m):
            temp[idx + n // 2][jdx] = matrix[idx][jdx]

    # 3 to 4:
    for idx in range(n // 2, n):
        for jdx in range(m // 2, m):
            temp[idx][jdx - m // 2] = matrix[idx][jdx]

    # 4 to 1
    for idx in range(n // 2, n):
        for jdx in range(m // 2):
            temp[idx - n // 2][jdx] = matrix[idx][jdx]

    matrix = temp


def convert_6():
    global m, n, matrix
    temp = [[0] * m for _ in range(n)]

    # 1 to 4

    for idx in range(n // 2):
        for jdx in range(m // 2):
            temp[idx + n // 2][jdx] = matrix[idx][jdx]

    # 2 to 1

    for idx in range(n // 2):
        for jdx in range(m // 2, m):
            temp[idx][jdx - m // 2] = matrix[idx][jdx]

    # 3 to 2:
    for idx in range(n // 2, n):
        for jdx in range(m // 2, m):
            temp[idx - n // 2][jdx] = matrix[idx][jdx]

    # 4 to 3
    for idx in range(n // 2, n):
        for jdx in range(m // 2):
            temp[idx][jdx + m // 2] = matrix[idx][jdx]

    matrix = temp


n, m, r = map(int, read().split())

matrix = [list(map(int, read().split())) for _ in range(n)]
converts = list(map(int, read().split()))

for convert in converts:
    if convert == 1:
        convert_1()
    elif convert == 2:
        convert_2()
    elif convert == 3:
        convert_3()
    elif convert == 4:
        convert_4()
    elif convert == 5:
        convert_5()
    else:
        convert_6()


for mat in matrix:
    print(*mat)
