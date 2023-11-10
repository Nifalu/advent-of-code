# Solution for AdventOfCode 2022, Day 14
# https://adventofcode.com/2022/day/14


def getInput(file : str):
    with open(file, 'r') as f:
        lines = [line.split('->') for line in f.read().splitlines()]
        lines = [[coords.strip() for coords in line] for line in lines]
        lines = [[coords.split(',') for coords in line] for line in lines]
        lines = [[[int(xy) for xy in coords] for coords in line] for line in lines]
        lines = [[tuple(coords) for coords in line] for line in lines]
    return lines


def drop(wall : list, part_1: bool):
    resting_sand = 0
    
    while True: # keep drippin
        x, y = (500, 0) # current drip

        while True: # try to fall
            if (x, y+1) not in wall[1]: # move down
                y += 1
            
            elif (x-1, y+1) not in wall[1]: # move downleft
                y += 1
                x -= 1

            elif (x+1, y+1) not in wall[1]: # move downright
                y += 1
                x += 1

            else: #rest
                wall[1].add((x, y))
                resting_sand += 1
                #print('resting now',resting_sand, 'latest:', x,y)
                if y == 0:
                    return # filled the hole
                break

            # Part 1
            if part_1 and y > wall[0]: # drop to infinity
                return
            
            # Part 2
            if not part_1 and y == wall[0] + 2:
                wall[1].add((x, y)) # can't go deeper
                break


def draw_wall(start : tuple, end : tuple, wall : list):

    x1, y1 = start
    x2, y2 = end
    
    for x in range(min(x1, x2), max(x1,x2)+1):
        for y in range(min(y1, y2), max(y1, y2)+1):
            wall[1].add((x,y))
            wall[0] = y if y > wall[0] else wall[0]
    
            
                

def main():

    print('---------------------------------------------')
    
    lines = getInput('input.txt')
    wall_1 = [0, set()]
    wall_2 = [0, set()]
    
    for line in lines:
        for idx in range(1, len(line)):
            draw_wall(line[idx-1], line[idx], wall_1)
            draw_wall(line[idx-1], line[idx], wall_2)

    drop(wall_1, True) # part 1
    drop(wall_2, False) # part 2

    print('---------------------------------------------')


if __name__ == "__main__":
    main()
