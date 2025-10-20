# 감마선을 맞은 컴퓨터

from sys import stdin

read = lambda: stdin.readline().rstrip()

for _ in range(15):
    image = read().split()
    if "w" in image:
        print("chunbae")
        break
    elif "b" in image:
        print("nabi")
        break
    elif "g" in image:
        print("yeongcheol")
        break
