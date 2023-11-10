# Solution for AdventOfCode 2022, Day 13
# https://adventofcode.com/2022/day/13



# Returns list of integers or other lists
def getInput(file : str):
    with open(file, 'r') as f:
        lines = [line.strip() for line in f.read().splitlines()]
        lines = list(line for line in lines if line)
        lines = [parseline(line) for line in lines]
    return lines


# Parse input string into lists of ints and lists
def parseline(line: str):
    line = parse(line, 1)
    return(line[0])

def parse(line: str, n: int):
    packet_content = []
    while n < len(line):
        if len(line) <= n:
            print('went too deep')
            return
        if line[n] == '[':
            value, n = (parse(line, n + 1))
            packet_content.append(value)

        if line[n] == ']':
            return packet_content, n+1

        if line[n] != ',':
            a = line[n::].split(',', 1)
            a = a[0].split(']')
            packet_content.append(int(a[0]))
            n += len(a[0])
        else:
            n += 1


# Compares line a and b from one package. returns negative numbers when in correct order,
# 0 when equal and positive numbers when in incorrect order.
# Calls itself recusively to solve lists in lists
def compare(line_a, line_b) -> int:
    
    if isinstance(line_a, int) and isinstance(line_b, int):
        return line_a - line_b

    elif isinstance(line_a, list) and isinstance(line_b, list):
        for idx in range(min(len(line_a), len(line_b))):
            res = compare(line_a[idx], line_b[idx])
            if res != 0:
                return res
        return len(line_a) - len(line_b)

    elif isinstance(line_a, int):
        return compare([line_a], line_b)

    elif isinstance(line_b, int):
        return compare(line_a, [line_b])
    
    else: print('failed')


def main():

    print('---------------------------------------------')
    lines = getInput('input.txt')

    # iterate two lines at a time, compare the two and increase by index + 1 when in right order
    sum = 0
    for idx in range(1, len(lines), 2):
        if compare(lines[idx-1], lines[idx]) < 0:
            sum += idx//2 + 1

    print('part 1: ', sum)

    #part 2
    sorted_packets = [[[2]],[[6]]] # dividers are already in list
    for line in lines:
        for i in range(len(sorted_packets)):
            if compare(line, sorted_packets[i]) < 0:
                sorted_packets.insert(i,line)
                break
            elif i == len(sorted_packets)-1: # if packet is bigger that any list item.
                sorted_packets.append(line)
                break
    
    # find divider again because I lost it before...
    divider = []
    for idx, packet in enumerate(sorted_packets):
        if packet == [[2]] or packet == [[6]]:
            divider.append(idx+1)

    print('part 2: ', divider[0] * divider[1])
    
    print('---------------------------------------------')


if __name__ == "__main__":
    main()
