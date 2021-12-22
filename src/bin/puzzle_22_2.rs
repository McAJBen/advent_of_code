use advent_of_code_2021::ZipWithNextExt;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
struct Cuboid {
    set_type: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Cuboid {
    fn new(input: &str) -> Self {
        // on x=-20..33,y=-21..23,z=-26..28
        let (set_type, rest) = input.split_once(' ').unwrap();
        let dimensions = rest.split(',').collect::<Vec<_>>();

        let x = dimensions[0][2..].split_once("..").unwrap();
        let x = (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap() + 1);

        let y = dimensions[1][2..].split_once("..").unwrap();
        let y = (y.0.parse::<i32>().unwrap(), y.1.parse::<i32>().unwrap() + 1);

        let z = dimensions[2][2..].split_once("..").unwrap();
        let z = (z.0.parse::<i32>().unwrap(), z.1.parse::<i32>().unwrap() + 1);

        Self {
            set_type: set_type == "on",
            x,
            y,
            z,
        }
    }
}

struct CuboidGrid {
    x_ranges: Vec<(i32, i32)>,
    y_ranges: Vec<(i32, i32)>,
    z_ranges: Vec<(i32, i32)>,
    cubes: Vec<Vec<Vec<bool>>>,
    num_lit: u64,
}

fn unique_ranges(mut values: Vec<i32>) -> Vec<(i32, i32)> {
    values.sort_unstable();
    values.dedup();
    values.into_iter().zip_with_next().collect()
}

fn get_ranges_in_range(ranges: &[(i32, i32)], range: (i32, i32)) -> Vec<usize> {
    ranges
        .iter()
        .enumerate()
        .skip_while(|(_, &r)| r.0 < range.0)
        .take_while(|(_, &r)| r.0 < range.1)
        .map(|(i, _)| i)
        .collect()
}

impl CuboidGrid {
    fn new(cuboids: &[Cuboid]) -> Self {
        let x_ranges = unique_ranges(cuboids.iter().flat_map(|c| [c.x.0, c.x.1]).collect());
        let y_ranges = unique_ranges(cuboids.iter().flat_map(|c| [c.y.0, c.y.1]).collect());
        let z_ranges = unique_ranges(cuboids.iter().flat_map(|c| [c.z.0, c.z.1]).collect());

        let cubes = vec![vec![vec![false; x_ranges.len()]; y_ranges.len()]; z_ranges.len()];

        Self {
            x_ranges,
            y_ranges,
            z_ranges,
            cubes,
            num_lit: 0,
        }
    }

    fn add_cuboid(&mut self, cuboid: &Cuboid) {
        let valid_x_ranges = get_ranges_in_range(&self.x_ranges, cuboid.x);
        let valid_y_ranges = get_ranges_in_range(&self.y_ranges, cuboid.y);
        let valid_z_ranges = get_ranges_in_range(&self.z_ranges, cuboid.z);

        for x_range in valid_x_ranges {
            for &y_range in valid_y_ranges.iter() {
                for &z_range in valid_z_ranges.iter() {
                    let cube = self.cubes[z_range][y_range][x_range];

                    if cuboid.set_type && !cube {
                        let x = self.x_ranges[x_range];
                        let y = self.y_ranges[y_range];
                        let z = self.z_ranges[z_range];

                        self.num_lit +=
                            (x.1 - x.0) as u64 * (y.1 - y.0) as u64 * (z.1 - z.0) as u64;
                    } else if !cuboid.set_type && cube {
                        let x = self.x_ranges[x_range];
                        let y = self.y_ranges[y_range];
                        let z = self.z_ranges[z_range];

                        self.num_lit -=
                            (x.1 - x.0) as u64 * (y.1 - y.0) as u64 * (z.1 - z.0) as u64;
                    }
                    self.cubes[z_range][y_range][x_range] = cuboid.set_type;
                }
            }
        }
    }
}

fn main() {
    let input = read_to_string("input/22").unwrap();

    let cuboids = input.lines().map(Cuboid::new).collect::<Vec<_>>();

    let mut cuboid_grid = CuboidGrid::new(&cuboids);

    for cuboid in cuboids {
        cuboid_grid.add_cuboid(&cuboid);
    }

    let num_lit = cuboid_grid.num_lit;

    assert_eq!(1235484513229032, num_lit);

    println!("{}", num_lit);
}
