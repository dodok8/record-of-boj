# 열 개씩 끊어 출력하기

s = input()

for idx in range(0, len(s), 10):
    print(s[idx : idx + 10])
