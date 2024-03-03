from sys import stdin
import re

read = lambda: stdin.readline().rstrip()


def check_item(name, item):
    pattern = re.compile(f"_{item}$")
    return bool(pattern.match(name))


_n, item = read().split()
n = int(_n)
cnt = 0
for _ in range(0, n):
    name, num_s = read().split()
    num = int(num_s)

    if name == item:
        cnt += num
    else:
        if bool(re.compile(f".*_{item}_.*").match(name)):
            cnt += num
        elif bool(re.compile(f".*_{item}$").match(name)):
            cnt += num
        elif bool(re.compile(f"^{item}_.*").match(name)):
            cnt += num
print(cnt)
