# Solution for AdventOfCode 2022, Day 2
# https://adventofcode.com/2022/day/2

# A defeats C, C defeats B, B defeats A // used to calculate winner
rules = ['A', 'B', 'C']

# Dictionary to calculate the index of "rules"
translate = {'A':0, 'B':1, 'C':2, 'X':0, 'Y':1, 'Z':2}


def calculate(one, two):    # calculates the points for player two
    if rules[two] == rules[one]:        #draw
        return (3 + (two + 1))
    if rules[two] == rules[one - 2]:    #second wins
        return (6 + (two + 1))
    if rules[two] == rules[one - 1]:    #first wins
        return (0 + (two + 1))

def draw(first):    # returns the points for the move needed to draw
    return (3 + 1 + translate[first])

def win(first):     # returns the points for the move needed to win
    return (6 + 1 +(translate[rules[translate[first] - 2]]))

def loss(first):    # returns the points for the move needed to lose
    return (0 + 1 +(translate[rules[translate[first] - 1]]))


def part1():
    with open('input.txt', 'r') as f:
        line = f.readline()
        sum = 0
        while line:
            sum = sum + calculate(translate[line[0]], translate[line[2]])
            line = f.readline()
        return sum

def part2():
    with open('input.txt', 'r') as f:
        line = f.readline()
        sum = 0
        while line:
            if line[2] == 'X':      # force a loss
                sum = sum + loss(line[0])
            elif line[2] == 'Y':    # force a draw
                sum = sum + draw(line[0])
            elif line[2] == 'Z':    # force a win
                sum = sum + win(line[0])
            line = f.readline() 
        return sum


print('------------------------------')
print('Solution for part 1:', part1())
print('Solution for part 2:', part2())
print('------------------------------')