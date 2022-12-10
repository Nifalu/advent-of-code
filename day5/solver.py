# Solution for AdventOfCode 2022, Day 5
# https://adventofcode.com/2022/day/5


# Two stacks because I'm to lazy to read it from se file
stacks1 = [[],['H','T','Z','D'],
        ['Q','R','W','T','G','C','S'],
        ['P','B','F','Q','N','R','C','H'],
        ['L','C','N','F','H','Z'],
        ['G','L','F','Q','S'],
        ['V','P','W','Z','B','R','C','S'],
        ['Z','F','J'],
        ['D','L','V','Z','R','H','Q'],
        ['B','H','G','N','F','Z','L','D']]

# The same stack again because the first one gets destroyed by part 1
stacks2 = [[],['H','T','Z','D'],
        ['Q','R','W','T','G','C','S'],
        ['P','B','F','Q','N','R','C','H'],
        ['L','C','N','F','H','Z'],
        ['G','L','F','Q','S'],
        ['V','P','W','Z','B','R','C','S'],
        ['Z','F','J'],
        ['D','L','V','Z','R','H','Q'],
        ['B','H','G','N','F','Z','L','D']]


# Part 1:
# Crates are moved between the stacks one by one
def part1():
    with open('input.txt', 'r') as f:
        line = f.readline()

        while line:
            line = line.split(' ')
            for i in range(int(line[1])):
                stacks1[int(line[5])].append(stacks1[int(line[3])].pop())
            line = f.readline()

        res = ''
        for i in range(1,10):
            if len(stacks1[i]) != 0:
                res = res + stacks1[i].pop()

    return res

# Part 2: 
# Crates are moved between the stacks all at once
def part2():
    with open('input.txt', 'r') as f:
        line = f.readline()
        helperStack = []

        while line:
            line = line.split(' ')
            for i in range(int(line[1])):
                helperStack.append(stacks2[int(line[3])].pop())

            for i in range(int(line[1])):
                stacks2[int(line[5])].append(helperStack.pop())

            line = f.readline()

        res = ''
        for i in range(1,10):
            if len(stacks2[i]) != 0:
                res = res + stacks2[i].pop()

    return res


print('------------------------------')
print('Solution for part 1:', part1())  
print('Solution for part 2:', part2())
print('------------------------------')