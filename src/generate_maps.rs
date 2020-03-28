use rand::Rng;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    // returns the euclidean distance between two points
    fn dist(&self, p2: &Point) -> f32 {
      return (((self.x - p2.x)^2 + (self.y - p2.y)^2).abs() as f32).sqrt()
    }
}

fn generate_player_starts(board_size: i32, num_players: i32) -> Vec<Point> {
    let mut player_starts: Vec<Point> = Vec::new();

    let mut num_player_starts = 0;

    while num_player_starts < num_players {
        let x = rand::thread_rng().gen_range(0, board_size);
        let y = rand::thread_rng().gen_range(0, board_size);
        let player_start = Point{x: x, y: y};

        // will place players more than 3 squares away from each other
        for other_player_start in &player_starts {
            if (&player_start).dist(other_player_start) <= 3.0 {
                continue;
            }
        }

        player_starts.push(player_start);
        num_player_starts += 1;
    }

    player_starts
}

fn generate_obstacle_pos(board_size: i32, player_starts: &Vec<Point>, num_obstacles: i32) -> std::vec::Vec<Point> {
    let mut obstacle_positions: Vec<Point> = Vec::new();

    let mut num_obstacle_positions = 0;

    while num_obstacle_positions < num_obstacles {
        let x = rand::thread_rng().gen_range(0, board_size);
        let y = rand::thread_rng().gen_range(0, board_size);
        let obstacle_pos = Point{x: x, y: y};

        // will place obstacle if all players can reach each other
        obstacle_positions.push(obstacle_pos);
        num_obstacle_positions += 1;

        if !all_player_reachable(board_size, &player_starts, &obstacle_positions) {
            obstacle_positions.pop();
            num_obstacle_positions -= 1;
        }
    }

    obstacle_positions
}

fn all_player_reachable(board_size: i32, player_starts: &Vec<Point>, obstacle_positions: &Vec<Point>) -> bool {
    for player_start in player_starts {
        for other_player_start in player_starts {
            let mut seen_positions = vec![vec![0; board_size as usize]; board_size as usize];

            if path_possible(board_size, player_start, other_player_start, obstacle_positions, &mut seen_positions) == false {
                return false;
            }
        }
    }

    true
}

// make obstacle_positions a reference
fn path_possible(board_size: i32, start: &Point, end: &Point, obstacle_positions: &Vec<Point>, seen_positions: &mut Vec<Vec<i32>>) -> bool {
    if start.x == end.x {
        return true;
    }

    let x = start.x;
    let y = start.y;
    let next_positions = [Point{x: x + 1, y: y}, Point{x: x - 1, y: y}, Point{x: x, y: y + 1}, Point{x: x, y: y - 1}];

    for next_pos in &next_positions {
        if !is_obstacle(next_pos, obstacle_positions) && is_valid_pos(next_pos, board_size) && seen_positions[next_pos.x as usize][next_pos.y as usize] == 0 {
            //add to seen_positions
            seen_positions[next_pos.x as usize][next_pos.y as usize] = 1;

            path_possible(board_size, next_pos, end, obstacle_positions, seen_positions);
        }
    }

    if seen_positions[end.x as usize][end.y as usize] == 1 {
        return true;
    }
    false
}

fn is_obstacle(pos: &Point, obstacle_positions: &Vec<Point>) -> bool {
    for obstacle_pos in obstacle_positions {
        if pos == obstacle_pos {
            return true;
        }
    }
    false
}

fn is_valid_pos(pos: &Point, board_size: i32) -> bool {
    if pos.x >= 0 && pos.x < board_size && pos.y >= 0 && pos.y < board_size {
        return true;
    }
    else {
        return false;
    }
}

// tune number of players
// tune % of board to be obstacles
// tune board size
// tune number of minions

pub fn generate() {
    let num_players = 2;
    let _num_minions = 3; // per player
    let obstacle_percentage = 0.08;
    let min_board_size = 7;
    let max_board_size = 16;

    let board_size = rand::thread_rng().gen_range(min_board_size, max_board_size); //square. can make rectangle

    let player_starts = generate_player_starts(board_size, num_players);

    let num_obstacles = (((board_size * board_size) as f32) * obstacle_percentage) as i32;
    let obstacles_pos = generate_obstacle_pos(board_size, &player_starts, num_obstacles);
    println!("board size: {} num_obstacles: {} player_starts: {:?} obstacles_pos {:?}", board_size, num_obstacles, player_starts, obstacles_pos);
}

// Initial map state:
// (x,y) of each player
// (x,y) of each minion
// Board size (x, y)
// Obstacles: [(x, y)]
