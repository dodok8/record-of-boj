# 디지털 티비비

from sys import stdin


def read():
    return stdin.readline().rstrip()


channels = list()

for _ in range(int(read())):
    channels.append(read())


operations = list()

cursor = 0

while True:
    if channels[cursor] != "KBS1":
        cursor += 1
        operations.append(1)
    else:
        channels[cursor], channels[cursor - 1] = channels[cursor - 1], channels[cursor]
        cursor -= 1
        operations.append(4)

    if channels[0] == "KBS1":
        break

while True:
    if channels[cursor] != "KBS2":
        cursor += 1
        operations.append(1)
    else:
        channels[cursor], channels[cursor - 1] = channels[cursor - 1], channels[cursor]
        cursor -= 1
        operations.append(4)

    if channels[1] == "KBS2":
        break

print("".join(operations))
