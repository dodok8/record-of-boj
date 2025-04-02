# Balancing Weights

from sys import stdin

read = lambda: stdin.readline().rstrip()

t = int(read())

for _t in range(t):
    n = int(read())
    torque = sum(map(int, read().split()))

    if torque == 0:
        print("Equilibrium")
    elif torque > 0:
        print("Right")
    else:
        print("Left")
