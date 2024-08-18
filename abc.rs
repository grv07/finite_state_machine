const COLUMN_STATE_SIZE: usize = 130;
const LINE_END: usize = 128;

#[derive(Debug)]
struct FsmColumn {
    column: [usize; COLUMN_STATE_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            column: [0; COLUMN_STATE_SIZE],
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
                    column.column[LINE_END] = fsm.row.len() + 1;
                    fsm.row.push(column);
                }

                '.' => {
                    let mut column = FsmColumn::new();
                    for i in 0..127 {
                        column.column[i] = fsm.row.len() + 1;
                    }

                    fsm.row.push(column);
                }

                _ => {
                    let mut column = FsmColumn::new();
                    column.column[c as usize] = fsm.row.len() + 1;
                    fsm.row.push(column);
                }
            }
        }

        fsm
    }

    fn find(&self, query: &str) -> bool {
        let mut state: usize = 1;

        for c in query.chars() {
            if state == 0 && self.row[state].column[c as usize] == 0 {
                return false;
            }

            state = self.row[state].column[c as usize];
        }

        if self.row[state].column[LINE_END] == 0 {
            return false;
        }

        true
    }

    fn dump(&self) {
        for i in 0..COLUMN_STATE_SIZE {
            print!("{i:03?} => ");
            for r in &self.row {
                print!(" {}", r.column[i as usize]);
            }
            println!();
        }
    }
}

fn main() {
    let regex_q = "ab.$";
    let regex = Regex::compile(regex_q);
    regex.dump();

    let queries = vec!["ab", "abc", "abd", "abb", "hello"];

    println!("Regex: {}", regex_q);
    for query in queries {
        println!("{query} => {}", regex.find(query));
    }
}
