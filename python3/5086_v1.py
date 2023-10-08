from sys import stdin

read = lambda: stdin.readline().rstrip()

while True:
    first, second = map(int, input().split())

    if (first, second) == (0, 0):
        break
    if second % first == 0:
        print("factor")
    elif first % second == 0:
        print("multiple")
    else:
        print("neither")
