


#[derive(Debug, Copy, Clone)]
enum Entry {
    E,
    X,
    O,
}

struct Board {
    entries: [Entry; 9],
}

impl Board {
    fn new() -> Board {
        Board { entries: [Entry::E; 9] }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "{:?} {:?} {:?}", self.entries[0], self.entries[1], self.entries[2])?;
        writeln!(f, "{:?} {:?} {:?}", self.entries[3], self.entries[4], self.entries[5])?;
        writeln!(f, "{:?} {:?} {:?}", self.entries[6], self.entries[7], self.entries[8])
    }
}

fn main() {
    println!("{}", Board::new());
}
