use std::fmt;
use std::io::{self, BufRead};
use std::ops;
use std::str::FromStr;

#[derive(Clone)]
struct IntCode {
    pub memory: Vec<usize>,
}
impl FromStr for IntCode {
    type Err = <usize as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the line into integers
        let memory = s.split(',').map(|i| i.parse()).collect::<Result<_, _>>()?;

        Ok(IntCode { memory })
    }
}

impl IntCode {
    fn run(mut self) -> usize {
        let mut pos = 0;
        loop {
            match self.memory[pos] {
                1 => {
                    let a = self.memory[pos + 1];
                    let b = self.memory[pos + 2];
                    let c = self.memory[pos + 3];
                    self.memory[c] = self.memory[a] + self.memory[b];
                    pos += 4;
                }
                2 => {
                    let a = self.memory[pos + 1];
                    let b = self.memory[pos + 2];
                    let c = self.memory[pos + 3];
                    self.memory[c] = self.memory[a] * self.memory[b];
                    pos += 4;
                }
                99 => return self.memory[0],
                _ => {}
            }
        }
    }
    fn run_sym(&self) -> Expr {
        let mut pos = 0;
        let mut memory: Vec<Expr> = self
            .memory
            .clone()
            .into_iter()
            .map(|n| Expr::Term(Term::Factor(Factor::Number(n))))
            .collect();
        loop {
            match memory[pos].eval() {
                1 => {
                    let a = memory[pos + 1].clone();
                    let b = memory[pos + 2].clone();
                    let c = memory[pos + 3].clone();
                    memory[c.eval()] = memory[a.eval()].clone() + memory[b.eval()].clone();
                    pos += 4;
                }
                2 => {
                    let a = memory[pos + 1].clone();
                    let b = memory[pos + 2].clone();
                    let c = memory[pos + 3].clone();
                    memory[c.eval()] = memory[a.eval()].clone() * memory[b.eval()].clone();
                    pos += 4;
                }
                99 => return memory[0].clone(),
                _ => {}
            }
        }
    }
}
#[derive(Clone, Debug)]
enum Expr {
    Term(Term),
    Add(Term, Term),
}
impl Expr {
    fn eval(&self) -> usize {
        match self {
            Expr::Term(term) => term.eval(),
            Expr::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
        }
    }
    fn simplify(&self) -> Self {
        match self {
            Expr::Term(term) => Expr::Term(term.simplify()),
            Expr::Add(lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();
                match (lhs, rhs) {
                    (Term::Factor(Factor::Number(lhs)), Term::Factor(Factor::Number(rhs))) => {
                        Expr::Term(Term::Factor(Factor::Number(lhs + rhs)))
                    }
                    _ => self.clone(),
                }
            }
        }
    }
}
impl ops::Add for Expr {
    type Output = Expr;
    fn add(self, rhs: Self) -> Self {
        Expr::Add(
            Term::Factor(Factor::Expr(Box::new(self))),
            Term::Factor(Factor::Expr(Box::new(rhs))),
        )
    }
}
impl ops::Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Self) -> Self {
        Expr::Term(Term::Multiply(
            Factor::Expr(Box::new(self)),
            Factor::Expr(Box::new(rhs)),
        ))
    }
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Term(term) => term.fmt(f),
            Expr::Add(lhs, rhs) => write!(f, "({}+{})", lhs, rhs),
        }
    }
}

#[derive(Clone, Debug)]
enum Term {
    Factor(Factor),
    Multiply(Factor, Factor),
}
impl Term {
    fn eval(&self) -> usize {
        match self {
            Term::Factor(factor) => factor.eval(),
            Term::Multiply(lhs, rhs) => lhs.eval() * rhs.eval(),
        }
    }
    fn simplify(&self) -> Self {
        match self {
            Term::Factor(factor) => Term::Factor(factor.simplify()),
            Term::Multiply(lhs, rhs) => {
                match (lhs, rhs) {
                    (Factor::Number(lhs), Factor::Number(rhs)) => {
                        Term::Factor(Factor::Number(lhs * rhs))
                    }
                    _ => self.clone(),
                }
                //Term::Multiply(lhs.simplify(), rhs.simplify())
            }
        }
    }
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Factor(factor) => factor.fmt(f),
            Term::Multiply(lhs, rhs) => write!(f, "({}*{})", lhs, rhs),
        }
    }
}

#[derive(Clone, Debug)]
enum Factor {
    Number(usize),
    Expr(Box<Expr>),
}
impl Factor {
    fn eval(&self) -> usize {
        match self {
            Factor::Number(n) => *n,
            Factor::Expr(e) => e.eval(),
        }
    }
    fn simplify(&self) -> Self {
        match self {
            Factor::Number(n) => Factor::Number(*n),
            Factor::Expr(e) => Factor::Expr(Box::new(e.simplify())),
        }
    }
}
impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Factor::Number(n) => n.fmt(f),
            Factor::Expr(expr) => expr.fmt(f),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let line = line.trim_end();

    let program: IntCode = line.parse()?;

    const PART_1_NOUN: usize = 12;
    const PART_1_VERB: usize = 2;
    const PART_2_TARGET: usize = 19690720;

    // Copy the program
    let mut p1_program = program.clone();
    // Set the variables
    p1_program.memory[1] = PART_1_NOUN;
    p1_program.memory[2] = PART_1_VERB;
    // Run the program symbolically and get an AST of its runtime
    let ast = p1_program.run_sym();

    println!("Part 1: {}", p1_program.run());
    println!("{}", ast);

    // Brute force solution to part 2
    for noun in 0..program.memory.len() {
        for verb in 0..program.memory.len() {
            // Clone the program
            let mut p2_program = program.clone();
            p2_program.memory[1] = noun;
            p2_program.memory[2] = verb;
            if p2_program.run() == PART_2_TARGET {
                let mut p2_program = program.clone();
                p2_program.memory[1] = noun;
                p2_program.memory[2] = verb;
                let ast = p2_program.run_sym();
                println!("{}", ast);
                println!("Part 2: {}", noun * 100 + verb);
            }
        }
    }
    Ok(())
}
