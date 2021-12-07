# part1
def part1():
    distances = []
    for pos in range(0, max(input)):
        distance = [abs(input[i] - pos) for i in range(len(input))]
        distances.append(sum(distance))
    print("Part1", min(distances))


def part2():
    fuel_requirements = []
    for pos in range(0, max(input)):
        diff = [abs(input[i] - pos) for i in range(len(input))]
        fuel = [sum(range(diff[i]+1)) for i in range(len(input))]
        fuel_requirements.append(sum(fuel))
    print("Part2", min(fuel_requirements))


#input = open("example.in").read().strip().split(",")
input = open("7.in").read().strip().split(",")
input = list(map(int, input))
part1()
part2()
