# Solution for AdventOfCode 2022, Day 9
# https://adventofcode.com/2022/day/9

class Node():                       # A Node represents a Knot in the Rope

    def __init__(self, name: str, x: int, y: int):
        self.x = x
        self.y = y
        self.name = name
        self.next = None

    def left(self):                 # Moves node 1 left
        self.x -= 1

    def right(self):                # Moves node 1 right
        self.x += 1

    def up(self):                   # Moves node 1 up
        self.y += 1

    def down(self):                 # Moves node 1 down
        self.y -= 1

    def getPos(self):               # returns current position
        return self.x, self.y


class Rope():

    # Creates a Rope with given number of Knots. Each knot is connected to the following knot
    def __init__(self, number_of_knots: int, boundary: int, ):
        self.boundary = boundary            # boundary to how far away any two connected Knots can be
        self.visited = set()                # list of positions that the tail visited
        self.head = Node('head', 0, 0)
        for i in range(1,number_of_knots):
            self.append(Node(str(i),0,0))


    # Adds a knot at the end of the Rope
    def append(self, node: Node):
        curr = self.head
        while curr.next != None:
            curr = curr.next
        curr.next = node

    # Moves the Head of the Rope 'distance' steps in this 'direction'
    def move_head(self, direction: str, distance: int):
        for i in range(distance):
            if direction == 'L':
                self.head.left()
            elif direction == 'R':
                self.head.right()
            elif direction == 'U':
                self.head.up()
            elif direction == 'D':
                self.head.down()

            # After the head moved, the tail needs to update()
            self.update()

    # If any knot moved too far away from his next knot, it pulls the next knot in his direction.
    # This iterates through all the knots of the Rope starting at the head and updates any if needed.
    def update(self):
        curr = self.head
        while curr.next != None:
            if self.is_out_of_bounds(curr, curr.next, self.boundary):
                self.follow_my_head(curr, curr.next)
            curr = curr.next

        # After all knots have been updated, we add the (new) tail position to the set of visited positions
        self.visited.add(curr.getPos())


    # checks if any two knots are too far away from each other
    def is_out_of_bounds(self, head: Node, tail: Node, bound: int):
        if (abs(head.x - tail.x) > bound) or (abs(head.y - tail.y) > bound):
            return True


    # Moves the tail closer to the head. 
    # This is not the head or tail of the rope but just any two Nodes
    def follow_my_head(self, head: Node, tail: Node):
        if (head.x > tail.x) and (head.y == tail.y): # head is right of tail
            tail.right()
        elif (head.x < tail.x) and (head.y == tail.y): # head is left of tail
            tail.left()
        elif (head.x == tail.x) and (head.y > tail.y): # head is over tail
            tail.up()
        elif (head.x == tail.x) and (head.y < tail.y): # head is under tail
            tail.down()
        elif (head.x > tail.x) and (head.y > tail.y): # head is up-right of tail
            tail.up()
            tail.right()
        elif (head.x > tail.x) and (head.y < tail.y): # head is down-right of tail
            tail.down()
            tail.right()
        elif (head.x < tail.x) and (head.y > tail.y): # head is up-left of tail
            tail.up()
            tail.left()
        elif (head.x < tail.x) and (head.y < tail.y): # head is down-left of tail    
            tail.down()
            tail.left()

# --------------------------------------------------------------


# Parse input to a list of lines
def getInput(filename: str):
    with open(filename, 'r') as f:
        line = f.readline().strip('\n')
        lines = []
        while line:
            lines.append(line)
            line = f.readline().strip('\n')

        return lines

# Solve Part 1
def p1():
    lines = getInput('input.txt')
    rope = Rope(2,1) # Create Rope with two knots
    rope.visited.add((0,0))

    for line in lines:
        rope.move_head(line[0], int(line[2::]))

    print('Part 1: Tail visited', len(rope.visited), 'positions.')


# Solve Part 2
def p2():
    lines = getInput('input.txt')
    rope = Rope(10,1)   # Create Rope with ten knots
    rope.visited.add((0,0))

    for line in lines:
        rope.move_head(line[0], int(line[2::]))

    print('Part 2: Tail visited', len(rope.visited), 'positions.')



def main():

    print('---------------------------------------------')
    p1()
    p2()
    print('---------------------------------------------')
    
    

if __name__ == "__main__":
    main()
        















