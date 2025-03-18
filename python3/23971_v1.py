# ZOAC 4

from math import ceil

h, w, n, m = map(int, input().split())

x = ceil(h / (n + 1))
y = ceil(w / (m + 1))

print(int(x * y))
