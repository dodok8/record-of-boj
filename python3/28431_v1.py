# 양말 짝 맞추기

socks = [0 for _ in range(10)]

for idx in range(5):
    socks[int(input())] += 1

for idx in range(10):
    if socks[idx] % 2 == 1:
        print(idx)
