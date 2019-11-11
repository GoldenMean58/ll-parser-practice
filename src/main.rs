use std::io;

#[derive(PartialEq, Debug, Clone, Copy)]
enum TerminalSymbol{
    LeftParens,
    RightParens,
    A,
    Plus,
    End,
    Invalid,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum NonTerminalSymbol {
    S,
    F,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Symbol {
    Terminal(TerminalSymbol),
    NonTerminal(NonTerminalSymbol),
}

impl From<usize> for Symbol {
    fn from(num: usize) -> Self {
        match num {
            0 => Symbol::Terminal(TerminalSymbol::LeftParens),
            1 => Symbol::Terminal(TerminalSymbol::RightParens),
            2 => Symbol::Terminal(TerminalSymbol::A),
            3 => Symbol::Terminal(TerminalSymbol::Plus),
            4 => Symbol::Terminal(TerminalSymbol::End),
            _ => Symbol::Terminal(TerminalSymbol::Invalid),
        }
    }
}

impl From<Symbol> for usize {
    fn from(sym: Symbol) -> Self {
        match sym {
            Symbol::Terminal(TerminalSymbol::LeftParens) => 0,
            Symbol::Terminal(TerminalSymbol::RightParens) => 1,
            Symbol::Terminal(TerminalSymbol::A) => 2,
            Symbol::Terminal(TerminalSymbol::Plus) => 3,
            Symbol::Terminal(TerminalSymbol::End) => 4,
            _ => 5,
        }
    }
}

fn lexical_analysis(input_text: String) -> Vec<Symbol> {
    println!("Lexical analysis!");
    let mut ret: Vec<Symbol> = vec![];
    for c in input_text.chars() {
        ret.push(match c {
            '(' => Symbol::Terminal(TerminalSymbol::LeftParens),
            ')' => Symbol::Terminal(TerminalSymbol::RightParens),
            'a' => Symbol::Terminal(TerminalSymbol::A),
            '+' => Symbol::Terminal(TerminalSymbol::Plus),
            _ => Symbol::Terminal(TerminalSymbol::Invalid),
        });
    }
    ret.push(Symbol::Terminal(TerminalSymbol::End));
    println!("tokens: {:?}", ret);
    ret
}

fn syntatics_analysis(tokens: Vec<Symbol>) {
    println!("Syntatics analysis!");
    // let table = [[1, 255, 0, 255, 255, 255], [255, 255, 2, 255, 255, 255]];
    let table = [[1, 255, 0, 255, 255, 255], [255, 255, 2, 255, 255, 255]];
    // let rules = [
    //     vec![Symbol::NonTerminal(NonTerminalSymbol::F)], 
    //     vec![Symbol::Terminal(TerminalSymbol::LeftParens), Symbol::NonTerminal(NonTerminalSymbol::S), Symbol::Terminal(TerminalSymbol::Plus), Symbol::NonTerminal(NonTerminalSymbol::F), Symbol::Terminal(TerminalSymbol::RightParens)], 
    //     vec![Symbol::Terminal(TerminalSymbol::A)]
    // ];
    let rules = [
        vec![Symbol::NonTerminal(NonTerminalSymbol::F)], 
        vec![Symbol::Terminal(TerminalSymbol::LeftParens), Symbol::NonTerminal(NonTerminalSymbol::S), Symbol::Terminal(TerminalSymbol::Plus), Symbol::NonTerminal(NonTerminalSymbol::S), Symbol::Terminal(TerminalSymbol::RightParens)], 
        vec![Symbol::Terminal(TerminalSymbol::A)]
    ];
    let mut stack = vec![Symbol::Terminal(TerminalSymbol::End), Symbol::NonTerminal(NonTerminalSymbol::S)];
    let mut token_position = 0;
    while stack.len() > 0 {
        let token = tokens[token_position];
        let symbol = stack.pop().unwrap();
        match symbol {
            Symbol::NonTerminal(non_terminal) => {
                let non_terminal_index = non_terminal as usize;
                let terminal_index = usize::from(token);
                let mut rule = rules[table[non_terminal_index][terminal_index]].clone();
                println!("rule: {:?}", rule);
                rule.reverse();
                stack = [stack, rule].concat();
                println!("stack: {:?}", stack);
            },
            Symbol::Terminal(terminal) => {
                match token {
                    Symbol::Terminal(terminal_symbol) if terminal == terminal_symbol => {
                        token_position += 1;
                        if terminal == TerminalSymbol::End {
                            println!("input accepted!");
                        }
                    },
                    _ => {
                        println!("bad input: {:?}", token);
                        break;
                    }
                }
            },
        };
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    let input = input.trim().to_string();
    // syntatics_analysis(lexical_analysis("(a+a)".to_string()));
    syntatics_analysis(lexical_analysis(input));
}
