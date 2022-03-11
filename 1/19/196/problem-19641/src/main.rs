fn main() {
    use std::io::Read;

    let mut io = std::io::stdin();
    let mut input = String::new();
    match io.read_to_string(&mut input) {
        Ok(_) => match solve(&input) {
            Ok(output) => print!("{}", output),
            Err(error) => print!("{:#?}", error),
        }
        Err(error) => print!("{:#?}", error),
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines();
    let count = lines.next().unwrap().parse()?;
    let mut edges = vec![vec![]; count + 1];
    for line in lines.clone().take(count) {
        let mut points = line.split_whitespace();
        let index: usize = points.next().unwrap().parse()?;
        let edge = &mut edges[index];
        for point in points.take_while(|&point| point != "-1") {
            edge.push(point.parse()?);
        }
        edge.sort_by_key(|&key: &Key| key.0);
    }

    let root = lines.last().unwrap().parse()?;
    for (key, (left, right)) in traverse(&edges, root).iter().enumerate().skip(1) {
        writeln!(output, "{} {} {}", key, left, right)?;
    }

    Ok(output)
}

fn traverse(edges: &[Vec<Key>], root: Key) -> Vec<(Index, Index)> {
    fn traverse(
        edges: &[Vec<Key>],
        nodes: &mut Vec<(Index, Index)>,
        visiting: &mut Vec<bool>,
        parent: Key,
        left: Index,
    ) -> Index {
        visiting[parent.0] = true;

        let mut right = left + 1;
        for &child in edges[parent.0].iter() {
            if !visiting[child.0] {
                right = traverse(edges, nodes, visiting, child, right) + 1;
            }
        }
        nodes[parent.0] = (left, right);

        right
    }

    let mut nodes = vec![(Index(0), Index(0)); edges.len()];
    let mut visiting = vec![false; edges.len()];
    traverse(edges, &mut nodes, &mut visiting, root, Index(1));

    nodes
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Key(usize);

#[derive(Clone, Copy)]
struct Index(u32);

impl std::fmt::Display for Key {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Key {
    type Err = std::num::ParseIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Key(s.parse()?))
    }
}

impl std::fmt::Display for Index {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add<u32> for Index {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u32) -> Self {
        Index(self.0 + rhs)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::Int(error)
    }
}
