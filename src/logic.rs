// LKの体系を書く
#[derive(PartialEq)]
pub struct Sequent(Vec<Formula>, Vec<Formula>);

// 論理式
#[derive(Clone, PartialEq)]
pub enum Formula {
    Atom(&'static str), //原始式
    And(Box<Formula>, Box<Formula>),
    Not(Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
}

enum Rule {
    WeakeningLeft,
    NotRight,
    OrRightOne,
    OrRightTwo,
    ExchangeRight,
    ContractionRight,
}

enum ProofTree {
    Path(Sequent, Rule, Box<ProofTree>),
    Goal(Sequent),
}

impl Sequent {
    fn is_reachable(&self, rule: &Rule, another: &Sequent) -> bool {
        use Rule::*;
        match rule {
            WeakeningLeft => match (self, another) {
                (Sequent(left, right), Sequent(next_left, next_right)) => {
                    match next_left.as_slice() {
                        [_, next_left_rest @ ..] => {
                            if left.as_slice() == next_left_rest && right == next_right {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                }
            },
            _ => unimplemented!(),
        }
    }
}

impl ProofTree {
    fn is_valid(&self) -> bool {
        use ProofTree::*;
        match self {
            Path(sequent, rule, rest) => match &**rest {
                Goal(next_sequent) => sequent.is_reachable(rule, next_sequent),
                Path(next_sequent, _, _) => {
                    sequent.is_reachable(rule, next_sequent) && rest.is_valid()
                }
            },
            Goal(_) => true,
        }
    }
}

#[test]
fn sample_proof() {
    use Formula::*;
    use ProofTree::*;
    use Rule::*;
    let proof = Goal(Sequent(vec![Atom("A")], vec![Atom("A")]));
    assert!(proof.is_valid());

    let proof2 = Path(
        Sequent(vec![Atom("A")], vec![Atom("A")]),
        WeakeningLeft,
        Box::new(Goal(Sequent(vec![Atom("A"), Atom("A")], vec![Atom("A")]))),
    );
    assert!(proof2.is_valid());

    let proof3 = Path(
        Sequent(vec![Atom("A")], vec![Atom("A")]),
        NotRight,
        Box::new(Path(
            Sequent(vec![], vec![Atom("A"), Not(Box::new(Atom("A")))]),
            OrRightTwo,
            Box::new(Path(
                Sequent(
                    vec![],
                    vec![
                        Atom("A"),
                        Or(Box::new(Atom("A")), Box::new(Not(Box::new(Atom("A"))))),
                    ],
                ),
                ExchangeRight,
                Box::new(Path(
                    Sequent(
                        vec![],
                        vec![
                            Or(Box::new(Atom("A")), Box::new(Not(Box::new(Atom("A"))))),
                            Atom("A"),
                        ],
                    ),
                    OrRightOne,
                    Box::new(Path(
                        Sequent(
                            vec![],
                            vec![
                                Or(Box::new(Atom("A")), Box::new(Not(Box::new(Atom("A"))))),
                                Or(Box::new(Atom("A")), Box::new(Not(Box::new(Atom("A"))))),
                            ],
                        ),
                        ContractionRight,
                        Box::new(Goal(Sequent(
                            vec![],
                            vec![Or(Box::new(Atom("A")), Box::new(Not(Box::new(Atom("A")))))],
                        ))),
                    )),
                )),
            )),
        )),
    );
    assert!(proof3.is_valid());
}
