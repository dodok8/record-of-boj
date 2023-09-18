from sys import stdin

read = lambda: stdin.readline().rstrip()


def get_color_sum(n: int, color: int) -> int:
    global sum, cost
    if sum[n][color] != -1:
        return sum[n][color]
    compare_list: list[int] = []
    colors = {0, 1, 2} - {color}
    for item in colors:
        compare_list.append(get_color_sum(n - 1, item) + cost[n][color])
    sum[n][color] = min(compare_list)
    return sum[n][color]


n = int(read())
cost = [list(map(int, read().split())) for _ in range(n)]
sum = [[-1, -1, -1] for _ in range(n)]
for idx in range(3):
    sum[0][idx] = cost[0][idx]

print(min([get_color_sum(n - 1, color) for color in range(3)]))
