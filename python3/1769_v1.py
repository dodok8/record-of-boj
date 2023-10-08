from sys import stdin

read = lambda: stdin.readline().rstrip()
get_sum = lambda x: sum(map(int, str(x)))

init_num = int(read())
curr_num = init_num

step = 0
while int(curr_num) >= 10:
    curr_num = get_sum(curr_num)
    step += 1
print(step)
print("YES" if curr_num % 3 == 0 else "NO")
