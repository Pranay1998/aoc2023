input = """Time:        61677571
Distance:   430103613071150"""

x = input.split("\n")
times = list(map(lambda x: int(x), x[0][x[0].index(":") + 1:].strip().split()))
distances = list(map(lambda x: int(x), x[1][x[1].index(":") + 1:].strip().split()))

assert(len(times) == len(distances))

result = 1
for i in range(len(times)):
    num_ways = 0
    time = times[i]
    winning_distance = distances[i]
    for j in range(1, time):
        speed = j
        distance = speed * (time - j)
        if distance > winning_distance:
            num_ways += 1
    if num_ways > 0:
        result *= num_ways
print(result)
