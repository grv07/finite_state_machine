const COLUMN_STATE_SIZE: usize = 130;
const LINE_END: usize = 128;

#[derive(Debug, Default, Clone, Copy)]
struct FsmAction {
    next: usize,
    offset: i32,
}

#[derive(Debug)]
struct FsmColumn {
    column: [FsmAction; COLUMN_STATE_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            column: [FsmAction::default(); COLUMN_STATE_SIZE],
        }
    }
}

struct Regex {
    row: Vec<FsmColumn>,
}

impl Regex {
    fn compile(input: &str) -> Self {
        let mut fsm = Regex {
            row: vec![FsmColumn::new()],
        };

        for c in input.chars() {
            match c {
                '$' => {
                    let mut column = FsmColumn::new();
                    column.column[LINE_END] = FsmAction {
                        next: fsm.row.len() + 1,
                        offset: 1,
                    };
                    fsm.row.push(column);
                }

                '.' => {
                    let mut column = FsmColumn::new();
                    for i in 0..127 {
                        column.column[i] = FsmAction {
                            next: fsm.row.len() + 1,
                            offset: 1,
                        };
                    }

                    fsm.row.push(column);
                }

                '*' => {
                    let n = fsm.row.len();
                    for v in fsm
                        .row
                        .last_mut()
                        .unwrap()
                        .column
                        .first_chunk_mut::<129>()
                        .unwrap()
                        .iter_mut()
                    {
                        // I --> "ba*b$"
                        // I --> b --> a } ---> b ---> $
                        //              ^
                        if v.next == n {
                            v.next = n - 1;
                        } else if v.next == 0 {
                            v.next = n;
                            v.offset = 0;
                        }
                    }
                }

                _ => {
                    let mut column = FsmColumn::new();
                    column.column[c as usize] = FsmAction {
                        next: fsm.row.len() + 1,
                        offset: 1,
                    };

                    fsm.row.push(column);
                }
            }
        }

        fsm
    }

    fn find(&self, query: &str) -> bool {
        let mut state: usize = 1;
        let mut head: usize = 0;

        let query = query.chars().collect::<Vec<_>>();

        while 0 < state && state < self.row.len() && head < query.len() {
            let c = query[head];

            let fsm_action = self.row[state].column[c as usize];

            state = fsm_action.next;
            head = head + fsm_action.offset as usize;
        }

        if state == 0 {
            return false;
        }

        while state > 0 && state < self.row.len() {
            state = self.row[state].column[LINE_END].next;
        }

        state >= self.row.len()
    }

    fn dump(&self) {
        for i in 0..COLUMN_STATE_SIZE {
            print!("{i:03?} => ");
            for r in &self.row {
                print!(
                    " ({} {})",
                    r.column[i as usize].next, r.column[i as usize].offset
                );
            }
            println!();
        }
    }
}

fn main() {
    let regex_q = ".c$";
    let regex = Regex::compile(regex_q);
    regex.dump();

    let queries = vec!["bbbbbbbc", "abc", "bc", "cbc", "abd", "abb", "hello"];

    println!("Regex: {}", regex_q);
    for query in queries {
        println!("{query} => {}", regex.find(query));
    }
}
