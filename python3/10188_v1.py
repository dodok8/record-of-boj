from sys import stdin


def read():
    return stdin.readline().rstrip()


for _ in range(int(read())):
    w, h = map(int, read().split())

    for hdx in range(h):
        print("X" * w)
    print()
