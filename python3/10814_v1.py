from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())
people = []

for _ in range(n):
    people.append(read().split())

people.sort(key=lambda person: int(person[0]))

for age, name in people:
    print(f"{age} {name}")
