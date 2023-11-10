# Solution for AdventOfCode 2022, Day 7
# https://adventofcode.com/2022/day/7


class Node():

    def __init__(self, name, parent):
        self.dirs = {}
        self.files = {}
        self.name = name
        self.parent = parent


    def root(self):                                     # make me root
        self.parent = self


class Tree():

    def __init__(self, root, curry):
        self.root: Node = root
        self.curry: Node = curry

    def add_file(self, file_name: str, file_size: int): # add directory
        self.curry.files[file_name] = file_size

    def add_dir(self, dir_name: str, dir_node):         # add file
        self.curry.dirs[dir_name] = dir_node

    def move_up(self):                                  # move up
        self.curry = self.curry.parent

    def cd(self, name):                                 # move down
        self.curry = self.curry.dirs.get(name)

    # Returns total size below a certain node
    def getSize(self, node: Node):
        size = 0
        for dir in node.dirs:
            size += self.getSize(node.dirs.get(dir))
        for file in node.files:
            size += node.files.get(file)
        global dir_sizes
        dir_sizes.append(size)
        return int(size)

    # Sums the Sizes of directorys smaller than the upperlimit
    def sumTheSizes(self, upperlimit: int, node: Node):
        sum = 0
        for dir in node.dirs:
            sum += self.sumTheSizes(upperlimit, node.dirs.get(dir))
            size = self.getSize(node.dirs.get(dir))
            
            if size <= upperlimit:
                sum += size
                
        return int(sum)

    # You need space? I do space! -> returns the size of the directory that should be deleted, but not the directory. Because MacCleaner is trash.
    def macCleaner(self, total_space: int, usedspace: int, needed_space: int):
        res = usedspace
        limit = total_space - needed_space
        for size in dir_sizes:
            new_space = usedspace - size
            if (new_space <= limit) and (size < res):
                res = size
        return res

    # print entire tree
    def printTree(self):
        print('------------------- TREE --------------------')
        self.print1(self.root, 0)
        print('---------------------------------------------')

    # print tree from node downwards
    def print1(self, node: Node, level):
        print('\t' * level + node.name)
        for dir in node.dirs:
            self.print1(node.dirs.get(dir), level + 1)
        for file in node.files:
            print('\t' * (level + 1) + str(file))


# building da tree
def tree_builder(tree: Tree, file): 
    line = file.readline().strip()

    while line:
        if line[0:6] == '$ cd /':
            tree.curry = tree.root              # goto Root
        elif line[0:7] == '$ cd ..':
            tree.move_up()                      # goto Parent
        elif line[0:4] == '$ cd':
            tree.cd(line[5::])                  # goto directory
        elif line[0:3] == 'dir':
            dir = Node(line[4::],tree.curry)    # add directory
            tree.add_dir(line[4::], dir)
        
        elif not line[0:4] == '$ ls':
            size, name = line.split(' ')        # add file
            tree.add_file(name, int(size))

        line = file.readline().strip()




#-----------------------------------------------------------------------

dir_sizes = [] # stores sizes of every directory

def main():
    root = Node('root', None)
    root.root()
    t = Tree(root, root)
    with open('input.txt') as file:
        tree_builder(t, file)
    
    print('---------------------------------------------')
    print('\n')
    print('Sum of Sizes of Dirs below threshhold (Part 1):  ', t.sumTheSizes(100000, t.root))
    print('total size:  ', t.getSize(t.root))
    print('delete (Part 2):  ', t.macCleaner(70000000, t.getSize(t.root),30000000))
    t.printTree()
    print('\n')
    print('---------------------------------------------')
    

if __name__ == "__main__":
    main()
        















