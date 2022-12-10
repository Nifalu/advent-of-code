# Solution for AdventOfCode 2022, Day 6
# https://adventofcode.com/2022/day/6

# Part 1:
# Crates are moved between the stacks one by one
def part1():
    with open('input.txt', 'r') as f:
        chars = f.read()

        for i in range(3,len(chars),1):
            if len(set(chars[i-3:i+1])) == len(chars[i-3:i+1]):
                return i + 1

    return -1

# Part 2: 
# Crates are moved between the stacks all at once
def part2():
    with open('input.txt', 'r') as f:
        chars = f.read()
        
        for i in range(13,len(chars),1):
            if len(set(chars[i-13:i+1])) == len(chars[i-13:i+1]):
                return i + 1

    return -1


print('------------------------------')
print('Solution for part 1:', part1())  
print('Solution for part 2:', part2())
print('------------------------------')