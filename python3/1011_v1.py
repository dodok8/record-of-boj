# Fly me to the Alpha Centauri

from sys import stdin

read = lambda: stdin.readline().rstrip()

for _ in range(int(read())):
    x, y = map(int, read().split())
    distance = y - x

    n = 0
    while True:
        if distance <= n * (n + 1):
            break
        n += 1

    if distance <= n**2:
        print(n * 2 - 1)
    else:
        print(n * 2)
