from sys import stdin
from collections import deque

read = lambda: stdin.readline().rstrip()

given_string = read()
bomb_string = read()
last_of_bomb = bomb_string[-1]
judge_stack = deque()

for letter in given_string:
    if letter == last_of_bomb and len(judge_stack) + 1 >= len(bomb_string):
        popped_string = ""
        for _ in range(len(bomb_string) - 1):
            popped_string = judge_stack.pop() + popped_string
        popped_string = popped_string + letter
        if popped_string == bomb_string:
            pass
        else:
            judge_stack.extend(popped_string)
    else:
        judge_stack.append(letter)

if len(judge_stack) == 0:
    print("FRULA")
else:
    for letter in judge_stack:
        print(letter, end="")
    print()
