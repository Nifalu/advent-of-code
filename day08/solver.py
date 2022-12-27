# Solution for AdventOfCode 2022, Day 8
# https://adventofcode.com/2022/day/8




def getInput(filename: str):                                    # Reads input file and returns two Integer-Grids
    with open(filename, 'r') as f:
        lines = f.readlines()
        h_grid = []
        v_grid = []
        for i in range(len(lines)):
            h_grid.append([])
            v_grid.append([])
            for j in range(len(lines[0])-1):
                h_grid[i].append(int(lines[i][j].strip('\n')))  # Grid as in the file
                v_grid[i].append(int(lines[j][i].strip('\n')))  # Grid rotated 90 degrees so height is now horizontal
        return h_grid, v_grid



def visible_from_outside(h_grid:list, v_grid:list, x:int, y:int, width:int, height:int):

    from_west = h_grid[x][0:y]                                  # Trees from west-border to current tree
    from_east = h_grid[x][y+1:width]                            # same for east, north and south
    from_north = v_grid[y][0:x]
    from_south = v_grid[y][x+1:height]


    # If current tree is higher than the biggest in any direction -> return true

    if h_grid[x][y] > max(from_west): # visible from west
        return True
    if h_grid[x][y] > max(from_east): # visible from east
        return True
    if h_grid[x][y] > max(from_north): # visible from north
        return True
    if h_grid[x][y] > max(from_south): # visible from south
        return True


def scenic_score_of_tree(h_grid:list, v_grid:list, x:int, y:int, width:int, height:int):

    to_north = list(v_grid[y][0:x])[::-1]                       # Trees from current tree to north border
    to_west = list(h_grid[x][0:y])[::-1]                        # same for west, east and south
    to_east = list(h_grid[x][y+1:width])
    to_south = list(v_grid[y][x+1:height])

    all_directions = [to_west, to_north, to_east, to_south]     # Lists all directions 


# Loops through directions and trees in current direction and checks how far the viewing distance is

    scenic_score = 1
    for direction in all_directions:
        viewing_distance = 0

        for tree in direction:
            if tree < h_grid[x][y]:
                viewing_distance += 1
            else:
                viewing_distance += 1
                break

        scenic_score *= viewing_distance            # calculates scencic_score for current tree
    
    return scenic_score




def p1():
    h_grid, v_grid = getInput('input.txt')
    width = len(h_grid)
    height = len(v_grid)
    sum_of_visible_trees = 2 * height + 2 * (width -2) # all border trees are visible

    # iterates through every tree except the border trees and sums up all visible ones
    for x in range(1,width-1):
        for y in range(1,height-1):
            if visible_from_outside(h_grid, v_grid, x, y, width, height):
                sum_of_visible_trees += 1
    print('There are', sum_of_visible_trees, 'visible Trees from the outside')
            

def p2():
    h_grid, v_grid = getInput('input.txt')
    width = len(h_grid)
    height = len(v_grid)


    # Iterates through every tree and finds the one with the biggest scenic score

    highest_scenic_score = 0
    winner_tree = 0
    for x in range(1,width-1):
        for y in range(1,height-1):
            score = scenic_score_of_tree(h_grid, v_grid, x, y, width, height)
            if highest_scenic_score < score:
                highest_scenic_score = score
                winner_tree = [x,y]
                

    print('Highest Scenic Score is:', highest_scenic_score, 'at coordinates:', winner_tree, 'and a tree height is:', h_grid[winner_tree[0]][winner_tree[1]])






def main():

    print('---------------------------------------------')
    p1()
    p2()
    print('---------------------------------------------')
    
    

if __name__ == "__main__":
    main()
        















