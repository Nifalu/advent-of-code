# Solution for AdventOfCode 2022, Day 11
# https://adventofcode.com/2022/day/11


class Monkey():
    def __init__(self, items, operation, test, is_true, is_false):
        self.items = items
        self.operation = [operation[0], operation[1]]
        self.test = int(test)
        self.is_true = is_true
        self.is_false = is_false


def make_monkeys(filename: str):
    with open(filename, 'r') as f:
        lines = f.read()

        lines = [line for line in lines.split('\n\n')] # divides input in paragraphs
        lines = [line.split('\n') for line in lines] # divides every paragraph in lines
        lines = [[elem.strip() for elem in line] for line in lines] # removes trailing spaces   

        zoo:list[Monkey] = [] # The Zoo has all the monkeys

        for line in lines:
            items = [int(i) for i in line[1][16:].split(', ')]
            tmp = line[2].split(' ')
            operation = [tmp[4], tmp[5]]
            test = int(line[3].rsplit(maxsplit=1)[-1])
            is_true = int(line[4].rsplit(maxsplit=1)[-1])
            is_false = int(line[5].rsplit(maxsplit=1)[-1])

            monkey = Monkey(items, operation, test, is_true, is_false)
            zoo.append(monkey)

        return zoo

def inspection(worry_level:int, operation):
    if operation[1] == 'old':
        return pow(worry_level,2)
    if operation[0] == '*':
        worry_level *= int(operation[1])
    elif operation[0] == '+':
        worry_level += int(operation[1])
    return worry_level


def p1():
    zoo = make_monkeys('input.txt')
    inspections = [0 for i in range(len(zoo))]

    for round in range(20):
        for idx, monkey in enumerate(zoo):
            while len(monkey.items) != 0:
                item = monkey.items.pop(0)  # Monkey takes an item
                item = inspection(item, monkey.operation)   # Calculates its new value
                inspections[idx] += 1

                item = item // 3 # We calm down

                # Monkey does the test and throws it to another monkey
                if item % int(monkey.test) == 0:    
                    zoo[int(monkey.is_true)].items.append(item)
                else:
                    zoo[int(monkey.is_false)].items.append(item)
    
    inspections.sort()
    return inspections[-1] * inspections[-2]


def p2():
    zoo = make_monkeys('input.txt')
    inspections = [0 for i in range(len(zoo))]

    # Calculate the least common multiplier to not overexagerate later
    primes = 1
    for monkey in zoo:
        primes *= monkey.test

    for round in range(10000):
        print('round : '+str(round+1), end='\r') # Cool printout that stays on the same line 
        for idx, monkey in enumerate(zoo):
            while len(monkey.items) != 0:
                item = monkey.items.pop(0)  # Monkey takes an item
                item = inspection(item, monkey.operation)   # Calculates its new value
                inspections[idx] += 1

                item = item % primes # We don't calm down but are good in math

                # Monkey does the test and throws it to another monkey
                if item % int(monkey.test) == 0:
                    zoo[int(monkey.is_true)].items.append(item)
                else:
                    zoo[int(monkey.is_false)].items.append(item)

    inspections.sort()
    print() # empty printout because our round-counter does not go to the next line.
    return inspections[-1] * inspections[-2]



def main():

    print('---------------------------------------------')
    print(p1())
    print(p2())
    print('---------------------------------------------')


if __name__ == "__main__":
    main()
