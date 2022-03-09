use std::cmp::max;

const POINTS_TO_WIN_2: u32 = 21;

fn points(player_initial: [u32; 2], player_index: usize, round: u32) -> u32 {
    let mut s: u32 = 0;
    for i in 0..round {
        for j in 1..4 {
            s += (j + 3 * (player_index as u32) + 6 * i) % 100;
        }
    }
    ((player_initial[player_index] - 1 + s) % 10) + 1
}

fn part1(player_initial: [u32; 2]) -> u32 {
    let mut player_points = [0, 0];
    let mut round = 0;
    let mut player_index = 0;
    loop {
        player_points[player_index] += points(player_initial, player_index, round / 2 + 1);
        if player_points[player_index] >= 1000 {
            return (round + 1) * 3 * player_points[(player_index + 1) % 2];
        }
        round += 1;
        player_index = (player_index + 1) % 2;
    }
}
fn part2(player_initial: [u32; 2]) -> u128 {
    let player_points = [0, 0];
    let mut wins = [0, 0];
    let player_index = 1;
    let depth = 0;
    play_game(
        player_initial,
        player_points,
        &mut wins,
        player_index,
        1,
        depth,
    );
    max(wins[0], wins[1])
}

fn play_game(
    players_position: [u32; 2],
    player_points: [u32; 2],
    wins: &mut [u128; 2],
    player_index: usize,
    cumulative: u128,
    depth: u32,
) {
    if player_points[player_index] >= POINTS_TO_WIN_2 {
        wins[player_index] += cumulative;
        return;
    }
    let player_index = (player_index + 1) % 2;
    for (die_res, degen) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        if depth == 0 {
            println!("{}", die_res);
        }
        let mut new_players_position = players_position.clone();
        let mut new_player_points = player_points.clone();
        new_players_position[player_index] += die_res - 1;
        new_players_position[player_index] %= 10;
        new_players_position[player_index] += 1;
        new_player_points[player_index] += new_players_position[player_index];
        let cumulative = cumulative * degen as u128;
        play_game(
            new_players_position,
            new_player_points,
            wins,
            player_index,
            cumulative,
            depth + 1,
        );
    }
}

fn main() {
    println!("Part1 {}", part1([7, 1]));
    println!("Part2 {}", part2([7, 1]));
}
