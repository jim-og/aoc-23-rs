# Advent of Code 2023
[![codecov](https://codecov.io/github/jim-og/aoc-23-rs/graph/badge.svg?token=ETMH794N4Z)](https://codecov.io/github/jim-og/aoc-23-rs)

My solutions to 2023's [Advent of Code](https://adventofcode.com/2023).

## Notes
1. Day 5 part 2 is interesting. Working out the answer traditionally is not feasabile as there are an extreme number of possible inputs. It's significantly quicker to consider all possible results and perform the process in reverse until a matching seed is found.
1. Day 7 has a good example of custom ordering.
1. Day 8 makes use of lowest common multiple.
1. Day 10 has some interesting techniques. [Shoelace Formula](https://en.wikipedia.org/wiki/Shoelace_formula) determines the area of a simple polygon whose vertices are described by their Cartesian coordinates in the plane. [Pick's Theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem) can then be used to calculate the area in terms of the number of integer points within the boundary.
1. Day 11 uses [Taxicab geometry](https://en.wikipedia.org/wiki/Taxicab_geometry) to calculate the distance. This is the distance a taxicab would take between two points in a city instead of the Euclidean distance (how the crow flies).
1. Day 12 uses top-down [Dynamic Programming](https://en.wikipedia.org/wiki/Overlapping_subproblems).
1. Day 13 is a good example of traits and how to transpose a matrix.
1. Day 14 uses [Brent's algorithm](https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm) for cycle detection. Part 2 is too slow and needs improvement; I suspect this is due to the method of rotating the platform and then tilting it which results in a lot of copying. I like the elegance of having a single tilt function but clearly it's not very efficient.
1. Day 16 part 2 feels like it would be a candidate for dynamic programming as there a lots of overlapping sub-problems. The result of light travelling over a tile in a given direction has likely been seen before but correctly handling beams which get into loops makes it tricky.
1. Day 17 uses the [A* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm) to find the shortest distance. The Taxicab distance is used to calculate a heuristic to guide its search; it would be interesting to try others and see which is quickest for this class of problem. 