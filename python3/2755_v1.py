# 이번학기 평점은 몇점?

from sys import stdin

read = lambda: stdin.readline().rstrip()

grades = {
    "A+": 4.3,
    "A0": 4.0,
    "A-": 3.7,
    "B+": 3.3,
    "B0": 3.0,
    "B-": 2.7,
    "C+": 2.3,
    "C0": 2.0,
    "C-": 1.7,
    "D+": 1.3,
    "D0": 1.0,
    "D-": 0.7,
    "F": 0.0,
}


credits = 0.0
tot = 0.0

for _ in range(int(read())):
    subject, credit, grade = read().split()
    credit = int(credit)
    credits += credit
    tot += credit * grades[grade]

print(f"{tot / credits + 10**-10:.2f}")
