from sys import stdin

read = lambda: stdin.readline().rstrip()

num_b, num_f, power = map(int, read().split())

for _ in range(num_b):
    enemies = list(map(int, read().split()))
    enemies.sort()
    num_i = 0
    for enemy in enemies:
        if enemy == -1:
            num_i += 1
        elif enemy <= power:
            power += enemy
        else:
            while enemy > power:
                num_i -= 1
                power *= 2

                if num_i == -1:
                    break
            power += enemy
    if num_i < 0:
        break
    else:
        for i in range(num_i):
            power *= 2
if num_i < 0:
    print(0)
else:
    print(1)
