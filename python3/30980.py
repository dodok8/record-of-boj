import re
from sys import stdin

read = lambda: stdin.readline().rstrip()
n, m = map(int, read().split())

is_correct = [[False] * m for _ in range(n)]
paper: list[str] = list()

for _ in range(n * 3):
    paper.append(read())

q = re.compile("([0-9])(\+)([0-9])=([0-9]*)")


def is_valid(given: str):
    first, _, second, result = q.findall(given)[0]
    first = int(first)
    second = int(second)
    return int(result) == first + second


for idx in range(n):
    curr_idx = 3 * idx + 1
    for match in q.finditer(paper[curr_idx]):
        start, end, group = match.start(), match.end(), match.group()
        if is_valid(group):
            replacement = "*" * (end - start)
            paper[curr_idx - 1] = (
                paper[curr_idx - 1][:start]
                + replacement
                + paper[curr_idx - 1][end:]
            )
            paper[curr_idx + 1] = (
                paper[curr_idx + 1][:start]
                + replacement
                + paper[curr_idx + 1][end:]
            )
            paper[curr_idx] = (
                paper[curr_idx][: start - 1]
                + "*"
                + paper[curr_idx][start:end]
                + "*"
                + paper[curr_idx][end + 1 :]
            )
        else:
            paper[3 * idx] = (
                paper[3 * idx][: start + 2] + "/" + paper[3 * idx][start + 3 :]
            )
            paper[3 * idx + 1] = (
                paper[3 * idx + 1][:start]
                + group.replace("+", "/")
                + paper[3 * idx + 1][end:]
            )
            paper[3 * idx + 2] = (
                paper[3 * idx + 2][:start]
                + "/"
                + paper[3 * idx + 2][start + 1 :]
            )

for rows in paper:
    for letter in rows:
        print(letter, end="")
    print()
