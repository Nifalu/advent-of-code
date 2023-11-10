# Solution for AdventOfCode 2022, Day 1
# https://adventofcode.com/2022/day/1

# Part 1: Find the block with the biggest sum and return that sum
# Part 2: Find the blocks with the three biggest sums and return the sum of those.

f = open('input.txt', 'r')

sums = []   # List for all the sums
curr = 0    # current sum

line = f.readline()
while line:
    if line == '\n':        # when block is finished
        sums.append(curr)   # add new sum to the list of sums
        curr = 0            # resets the curr sum for the next block
    else:
        curr += int(line)   # adds current line to the sum of the block
    line = f.readline()     # reads next line

sums.sort(reverse=True)   # sorts the list with biggest first

print('sum of biggest block: ' + str(sums[0]))              # Solution Part 1
print('sum of three biggest blocks: ' + str(sum(sums[:3]))) # Solution Part 2