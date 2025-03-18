# 벌집

n = int(input())

if n == 1:
    print(1)
else:
    idx = 1
    a = 2
    b = 7
    while not (a <= n <= b):
        a += 6 * idx
        b += 6 * (idx + 1)
        idx += 1
    print(idx + 1)
