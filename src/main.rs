use std::env;

type State = [u8; 9];
type Position = (State, Vec<u8>);

const MOVES: [State; 9] = [
    [1, 1, 1, 1, 0, 0, 1, 0, 0],
    [1, 1, 1, 0, 1, 0, 0, 1, 0],
    [1, 1, 1, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 1, 1, 1, 0, 0],
    [0, 1, 0, 1, 1, 1, 0, 1, 0],
    [0, 0, 1, 1, 1, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 1, 1, 1],
    [0, 1, 0, 0, 1, 0, 1, 1, 1],
    [0, 0, 1, 0, 0, 1, 1, 1, 1],
];

fn make_move(state: State, m: State) -> State {
    let mut ret: State = state.clone();

    for position in 0..9 {
        ret[position] += m[position];
        ret[position] %= 3;
    }

    return ret;
}

#[allow(dead_code)]
fn possible_states_from_state(initial_state: State) -> Vec<State> {
    let mut result: Vec<State> = vec![];

    for m in MOVES {
        result.push(make_move(initial_state, m));
    }

    return result;
}

#[allow(dead_code)]
fn possible_positions_from_position(initial_position: Position) -> Vec<Position> {
    let mut result: Vec<Position> = vec![];

    for (i, m) in MOVES.iter().enumerate() {
        let mut new_path = initial_position.1.clone();
        new_path.push(i as u8);
        result.push((make_move(initial_position.0, *m), new_path));
    }

    return result;
}

fn positions_from_state(state: State) -> Vec<Position> {
    let mut positions: Vec<Position> = vec![];

    for i in 0..3u16.pow(9) {
        let mut m: State = [0; 9];

        let mut path: Vec<u8> = vec![];
        for pos in 0..9 {
            let count = ((i / 3u16.pow(pos as u32)) % 3) as u8;
            for _ in 0..count {
                path.push(pos as u8);
                m = make_move(m, MOVES[pos]);
            }
        }

        positions.push((make_move(state, m), path));
    }

    return positions;
}

fn solve(initial_state: State, solved_state: State) -> Result<Position, String> {
    let positions = positions_from_state(initial_state);

    for position in positions.clone() {
        if position.0 == solved_state {
            return Ok(position);
        }
    }

    return Err("Could not find solution to given positions".to_string());
}

fn state_from_string(string: String) -> Result<State, String> {
    if string.len() != 9 || !string.chars().all(|c| c >= '1' && c <= '3') {
        return Err("String provided is not a valid state description.".to_string());
    }

    let mut ret: State = [0; 9];

    for (i, c) in string.chars().enumerate() {
        ret[i] = (c.to_digit(10).unwrap() as u8) - 1;
    }

    return Ok(ret);
}

fn print_state(state: State) {
    println!(
        "{} | {} | {}\n----------\n{} | {} | {}\n----------\n{} | {} | {}\n",
        state[0] + 1,
        state[1] + 1,
        state[2] + 1,
        state[3] + 1,
        state[4] + 1,
        state[5] + 1,
        state[6] + 1,
        state[7] + 1,
        state[8] + 1
    );
}

fn print_solution(initial: State, solution: Position) {
    println!("Found solution between given states: {:?}", solution.1);
    println!("Initial Position:");
    print_state(initial);

    let mut state = initial.clone();

    for (i, c) in solution.1.iter().enumerate() {
        println!("{}: Making move {}", i + 1, c);
        state = make_move(state, MOVES[(*c) as usize]);
        print_state(state);
    }

    println!("This is the final position.");
}

fn main() {
    //let initial_state: State = [0, 1, 1, 1, 1, 0, 2, 0, 2];
    //let solved_state: State = [2, 2, 2, 2, 2, 2, 2, 2, 2];

    let args: Vec<String> = env::args().collect();

    // check that the args are what we expect
    if args.len() != 3 {
        println!("Incorrect number of arguments provided.");
        return;
    }

    let initial_state = state_from_string(args[1].clone()).unwrap();
    let final_state = state_from_string(args[2].clone()).unwrap();

    let position = solve(initial_state, final_state).unwrap();

    print_solution(initial_state, position);
}
