# Solution for AdventOfCode 2022, Day 3
# https://adventofcode.com/2022/day/3

# index of char represents its prio/points
prio = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'

def getPrio(char):
    return prio.index(char) + 1

def part1():
    with open('input.txt', 'r') as file:
        line = file.readline()
        sum = 0
        while line:
            # intersects two strings and gets Prio of the outcome
            sum = sum + getPrio(''.join(set(line[:(len(line)//2)]).intersection(line[(len(line)//2):])))
            line = file.readline()
    return sum


def part2():
    with open('input.txt', 'r') as file:
        lines = file.readlines()
        sum = 0
        for i in range(0,len(lines),3):
            # intersects three strings and gets Prio of the outcome
            sum = sum + getPrio(''.join(set(lines[i].strip('\n')).intersection((''.join(set(lines[i+1].strip('\n')).intersection(lines[i+2].strip('\n')))))))
    return sum


print('------------------------------')
print('Solution for part 1:', part1())  
print('Solution for part 2:', part2())
print('------------------------------')