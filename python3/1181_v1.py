from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())
words = []

for _ in range(n):
    words.append(read())

words = list(set(words))
words.sort()
words.sort(key=lambda word: len(word))

for word in words:
    print(word)
