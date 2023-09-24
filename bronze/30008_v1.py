from sys import stdin

read = lambda: stdin.readline().rstrip()


def get_percent(rank: int, n: int):
    return rank * 100 // n


def get_grade(p):
    if 0 <= p <= 4:
        return 1
    elif p <= 11:
        return 2
    elif p <= 23:
        return 3
    elif p <= 40:
        return 4
    elif p <= 60:
        return 5
    elif p <= 77:
        return 6
    elif p <= 89:
        return 7
    elif p <= 96:
        return 8
    else:
        return 9


n, k = map(int, read().split())
for rank in map(int, read().split()):
    print(get_grade(get_percent(rank, n)), end=" ")
print()
