use std::io::Write;

#[repr(usize)]
#[derive(Copy, Clone)]
enum State {
    Lock,
    Unlock,
}

const STATE_COUNT: usize = 2;

#[repr(usize)]
enum Event {
    Push,
    Coin,
}
const EVENT_COUNT: usize = 2;

const FSM: [[State; EVENT_COUNT]; STATE_COUNT] = [
    //PUSH COIN
    [State::Lock, State::Unlock], // LOCK
    [State::Lock, State::Unlock], // UNLOCK
];

fn next_state(state: State, event: Event) -> State {
    FSM[state as usize][event as usize]
}

#[allow(unreachable_patterns)]
fn state_as_str(state: State) -> &'static str {
    match state {
        State::Lock => "LOCK",
        State::Unlock => "UNLOCK",
        _ => unreachable!(),
    }
}

fn main() {
    let mut state = State::Lock;

    println!("State: {}", state_as_str(state));
    print!(">");

    let _ = std::io::stdout().flush();
    for line in std::io::stdin().lines() {
        match line.unwrap().to_lowercase().as_str() {
            "push" => state = next_state(state, Event::Push),
            "coin" => state = next_state(state, Event::Coin),
            "exit" => return,
            unknown => {
                eprintln!("ERROR: unknown event {unknown}");
            }
        }

        println!("State: {}", state_as_str(state));
        print!(">");
        let _ = std::io::stdout().flush();
    }
}
