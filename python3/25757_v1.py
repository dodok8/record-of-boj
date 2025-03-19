# 임스와 함께하는 미니게임

from sys import stdin


def read():
    return stdin.readline().rstrip()


n, game = read().split()

n = int(n)

people = set()

for _ in range(n):
    people.add(read())

match game:
    case "Y":
        print(len(people))
    case "F":
        print(len(people) // 2)
    case _:
        print(len(people) // 3)
