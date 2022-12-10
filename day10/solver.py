# Solution for AdventOfCode 2022, Day 10
# https://adventofcode.com/2022/day/10



# Parse input
def getInput(filename: str):
    with open(filename, 'r') as f:
        input = f.read().splitlines()
    return [(line[0:4], line[5::]) for line in input]  # [(addx, 15), (noop, ''), ...]

def solve():
    lines = getInput('input.txt')
    cycles = 1                  # we start with the 'first' circle
    register = 1
    signal_strengh = 0
    for line in lines:

        if line[0] == 'noop':
            # Cycle 1:
            renderPixel(cycles-1, register)  # do -1 because our current cycle has not been completed yet.
            if (cycles-20) % 40 == 0:
                signal_strengh += cycles * register
            cycles += 1

        else:
            # Cycle 1:
            renderPixel(cycles-1, register)
            if (cycles-20) % 40 == 0:
                signal_strengh += cycles * register
            cycles += 1

            # Cycle 2:
            renderPixel(cycles-1, register)
            if (cycles-20) % 40 == 0:
                signal_strengh += cycles * register
            register += int(line[1])
            cycles += 1

    print('\nSignal strengh is:', signal_strengh)


def renderPixel(cycle:int, register:int):
    if (cycle % 40 == 0) and cycle != 0:
        print('')
    if (cycle % 40) in [register-1, register, register+1]:
        print('#', end='')
    else:
        print(' ', end='')



def main():

    print('---------------------------------------------')
    solve()
    print('---------------------------------------------')


if __name__ == "__main__":
    main()
