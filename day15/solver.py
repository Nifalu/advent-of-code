# Solution for AdventOfCode 2022, Day 15
# https://adventofcode.com/2022/day/15

import re
from z3 import *


def getInput(file : str):
    with open(file, 'r') as f:
        sensors = [re.split('=|:|,', line) for line in f.read().splitlines()]
        sensors = [[tuple([int(s[1]), int(s[3])]), tuple([int(s[5]), int(s[7])])] for s in sensors]
    return sensors

def manhattan(x, y):
    return sum(abs(val1-val2) for val1, val2 in zip(x,y))

def main():

    print('---------------------------------------------')
    
    sensors = getInput('example.txt')
    blocked = set()
    beacons = set()
    LINE = 10
    for sensor in sensors:
        beacons.add(sensor[1])
        beacons.add(sensor[0])

    for sensor in sensors:
        dist = manhattan(sensor[0], sensor[1])
        sensor.append(dist)
        if manhattan(sensor[0], (sensor[0][0],LINE)) <= sensor[2]: # sensor crosses the line
            
            i = 0
            while manhattan((sensor[0]), (sensor[0][0]+i,LINE)) <= sensor[2]:
                if (sensor[0][0]+i,LINE) not in beacons:
                    blocked.add(sensor[0][0]+i)
                i += 1

            i = -1
            while manhattan((sensor[0]), (sensor[0][0]+i,LINE)) <= sensor[2]:
                if (sensor[0][0]+i,LINE) not in beacons:
                    blocked.add(sensor[0][0]+i)

                i -= 1


    i = Solver()

    for x in range(20):
        for y in range(20):
            if (x,y) not in beacons:
                print(x,y)
        
    print(len(blocked))

    print('---------------------------------------------')


if __name__ == "__main__":
    main()
