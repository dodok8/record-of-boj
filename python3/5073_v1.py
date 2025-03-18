# 삼각형과 세 변

from sys import stdin


def read():
    return stdin.readline().rstrip()


while True:
    given = list(map(int, read().split()))
    given.sort()

    a, b, c = given

    if a == b and b == c and c == a and a == 0:
        break
    elif a + b <= c:
        print("Invalid")
    elif a == b and b == c and c == a:
        print("Equilateral")
    elif a == b or b == c:
        print("Isosceles")
    else:
        print("Scalene")
