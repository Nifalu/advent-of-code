# Solution for AdventOfCode 2022, Day 4
# https://adventofcode.com/2022/day/4

# Pairs of ranges: from a-b and from c-d

# Part 1:
# Count all Ranges that completely include the other one
def part1():
    with open('input.txt', 'r') as f:
        line = f.readline()
        counter = 0
        while line:
            x, y = line.split(',')
            a, b = x.split('-')
            c, d = y.split('-')
            if (int(a) >= int(c) and int(b) <= int(d)) or (int(a) <= int(c) and int(b) >= int(d)):
                counter += 1
            line = f.readline()
    return counter

# Part 2:
# Count all Ranges that overlap (or include) the other one
def part2():
    with open('input.txt', 'r') as f:
        line = f.readline()
        counter = 0
        while line:
            x, y = line.split(',')
            a, b = x.split('-')
            c, d = y.split('-')
            a, b, c, d = int(a), int(b), int(c), int(d)
            if (a <= c and b >= c) or (c <= a and d >= a):
                print(a,b,c,d)
                counter += 1

            line = f.readline()
    return counter



print('------------------------------')
print('Solution for part 1:', part1())  
print('Solution for part 2:', part2())
print('------------------------------')