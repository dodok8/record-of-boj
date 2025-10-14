# 팀 이름 정하기

from sys import stdin

read = lambda: stdin.readline().rstrip()

name = read()
n = int(read())
teams = [read() for i in range(n)]
teams.sort()
teams.reverse()
max_p = 0
max_team = ""

for team in teams:
    num_l = name.count("L") + team.count("L")
    num_o = name.count("O") + team.count("O")
    num_v = name.count("V") + team.count("V")
    num_e = name.count("E") + team.count("E")
    p = (
        (num_l + num_o)
        * (num_l + num_v)
        * (num_l + num_e)
        * (num_o + num_v)
        * (num_o + num_e)
        * (num_v + num_e)
    ) % 100
    if max_p <= p:
        max_p = p
        max_team = team
print(max_team)
