# Solution for AdventOfCode 2022, Day 15
# https://adventofcode.com/2022/day/15




# in progress of understanding

# https://github.com/matthewmisiaszek/aoc_solves/blob/main/AoC_2022/AoC_2022_16.py

import re

def getInput(file : str):
    with open(file, 'r') as f:
        lines = [re.split('=|; |, | ', line) for line in f.read().splitlines()]
        tunnels = [line[10::] for line in lines]
        valves = [Valve(s[1], int(s[5]), tuple(tunnels[i])) for i,s in enumerate(lines)]

        # Connect valves that have tunnels to each other
        for valve in valves:
            for tunnel in valve.leadTo:
                i = next((i for i, x in enumerate(valves) if x.name == tunnel), None)
                valve.tunnels.append(valves[i])

    return valves # Return list of connected valves

class Valve():

    def __init__(self, name: str, flow: int, tunnels: tuple):
        self.name = name
        self.flow = flow
        self.leadTo = tunnels # String reference to Tunnels
        self.tunnels = []
    
    def __repr__(self) -> str:
        return self.name

START = 'AA'

def release_pressure(valves, time_limit, nworkers):
    queue = {(0, time_limit, tuple(), START)} # Pressure, time remaining, opened valves, starting point
    closed = {} # the highest pressure recorded for a given set of valves

    while queue:
        curr = max(queue)
        queue.discard(curr)
        pressure, time, opened, location = curr

        if opened in closed and closed[opened] >= pressure:
            continue # there is a better path already

        closed[opened] = pressure
        if time <= 0:
            continue # Time ran out



def main():

    print('---------------------------------------------')
    
    valves = getInput('example.txt')
    for valve in valves:
        print(valve, valve.tunnels)

    
    print('---------------------------------------------')


if __name__ == "__main__":
    main()
