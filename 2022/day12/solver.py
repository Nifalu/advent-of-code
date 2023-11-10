# Solution for AdventOfCode 2022, Day 12
# https://adventofcode.com/2022/day/12



# Node that contains its level, distance to start and all its neighbors
class Node():

    def __init__(self, level):
        self.neighbors = []
        self.distance = None
        self.level = level
    
    def add_neighbor(self, node):
        self.neighbors.append(node)

    def __repr__(self) -> str: # when printing the node, self.level is printed
        return self.level



# Read input from file
def getInput(input: str) -> str:
    with open(input) as f:
        input = f.readlines()
        input = [elem.strip('\n') for elem in input]
    return input


# Step 1: Transform the input grid to a map of nodes.
# Step 2: Connect the nodes with each other and save starting node.
# Step 3: Write all starting nodes in the queue.
# Step 4: bfs search and return distance to searched node.
def solve(input: str, startingpoint: str):

    visited = set()
    queue = []
    map = [[] for elem in input]  
    neighbors4 = ((1, 0), (0, 1), (-1, 0), (0, -1))  # offset to neighbors

    # step 1
    for idx, line in (enumerate(input)):
        for idx2, elem in (enumerate(input[idx])):
            node = Node(input[idx][idx2])
            map[idx].append(node)

    # step 2
    for x, line in (enumerate(map)):
        for y, elem in (enumerate(map[x])):
            for off_x, off_y in neighbors4:
                if 0 <= x + off_x < len(map) and 0 <= y + off_y < len(map[x]):
                    elem.add_neighbor(map[x+off_x][y+off_y])

            # step 3
            if elem.level in startingpoint:
                queue.append((0, map[x][y]))

    # step 4
    for dist, node in queue:
        if node.level == 'E':
            return dist
        for neighbor in node.neighbors:
            if neighbor != None:
                if good(node, neighbor) and neighbor not in visited:
                    visited.add(neighbor)
                    queue.append((dist + 1, neighbor))



def good(node_a: Node, node_b: Node):
    return ord(node_b.level.replace("E", "z")) - ord(node_a.level.replace("S", "a")) <= 1


def main():

    print('---------------------------------------------')
    lines = getInput('input.txt')
    distance1 = solve(lines, 'S')
    distance2 = solve(lines, 'Sa')
    print(distance1, distance2)
    print('---------------------------------------------')


if __name__ == "__main__":
    main()
