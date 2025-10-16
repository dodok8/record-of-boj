# 그룹 단어 체커

from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())
cnt = n

for _ in range(n):
    word = read()
    for idx in range(len(word) - 1):
        if word[idx] == word[idx + 1]:
            continue
        elif word[idx] in word[idx + 1 :]:
            cnt -= 1
            break

print(cnt)
