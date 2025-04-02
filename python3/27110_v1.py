# 특식 배부

n = int(input())

a, b, c = map(int, input().split())

sum = 0

for chick in [a, b, c]:
    if chick <= n:
        sum += chick
    else:
        sum += n

print(sum)
