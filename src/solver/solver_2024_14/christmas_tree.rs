// ###############################
// #.............................#
// #.............................#
// #.............................#
// #.............................#
// #..............#..............#
// #.............###.............#
// #............#####............#
// #...........#######...........#
// #..........#########..........#
// #............#####............#
// #...........#######...........#
// #..........#########..........#
// #.........###########.........#
// #........#############........#
// #..........#########..........#
// #.........###########.........#
// #........#############........#
// #.......###############.......#
// #......#################......#
// #........#############........#
// #.......###############.......#
// #......#################......#
// #.....###################.....#
// #....#####################....#
// #.............###.............#
// #.............###.............#
// #.............###.............#
// #.............................#
// #.............................#
// #.............................#
// #.............................#
// ###############################
pub static CHRISTMAS_TREE: [[bool; 31]; 33] = [
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        true, true, true, true, true, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, true,
        true, true, true, true, true, true, false, false, false, false, false, false, false, false,
        false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, true, true,
        true, true, true, true, true, true, true, false, false, false, false, false, false, false,
        false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        true, true, true, true, true, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, true,
        true, true, true, true, true, true, false, false, false, false, false, false, false, false,
        false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, true, true,
        true, true, true, true, true, true, true, false, false, false, false, false, false, false,
        false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, true, true, true,
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, true, true, true, true, true,
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, true, true,
        true, true, true, true, true, true, true, false, false, false, false, false, false, false,
        false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, true, true, true,
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, true, true, true, true, true,
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, false, false, false, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, false, false, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, false, false, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, true, true, true, true, true,
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, false, false, false, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, false, false, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, false, false, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, false, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ],
];
