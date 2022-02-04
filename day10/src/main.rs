use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn read<P>(path: P) -> std::io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// scoring for part 1
fn score(state: State) -> usize {
    match state {
        State::Corrupted('>') => 25137,
        State::Corrupted('}') => 1197,
        State::Corrupted(']') => 57,
        State::Corrupted(')') => 3,
        _ => 0,
    }
}

// map character to its counterpart
fn reverse(c: char) -> char {
    match c {
        '>' => '<',
        '}' => '{',
        ']' => '[',
        ')' => '(',
        '<' => '>',
        '{' => '}',
        '[' => ']',
        '(' => ')',
        _ => panic!("Invalid character!"),
    }
}

// calculate points for a completion (pt2)
fn points(chars: Vec<char>) -> usize {
    chars.iter().fold(0, |score, c| {
        score * 5
            + match c {
                '>' => 4,
                '}' => 3,
                ']' => 2,
                ')' => 1,
                _ => panic!("Invalid character!"),
            }
    })
}

#[derive(Debug, std::cmp::PartialEq)]
enum State {
    Valid,
    Incomplete,
    Corrupted(char),
}

fn evaluate(line: &str) -> State {
    let chars: Vec<char> = line.chars().collect();
    let mut stack: Vec<char> = Vec::new();
    for char in chars {
        let s = match char {
            '(' | '[' | '{' | '<' => {
                stack.push(char);
                (char, None)
            }
            '>' | '}' | ']' | ')' => (char, stack.pop()),
            _ => panic!("Invalid character!"),
        };
        match s {
            (cur, Some(popped)) => match (popped, cur) {
                ('<', '>') | ('{', '}') | ('[', ']') | ('(', ')') => {}
                (_, _) => {
                    return State::Corrupted(cur);
                }
            },
            (_, None) => {}
        }
    }
    if stack.is_empty() {
        return State::Valid;
    }
    State::Incomplete
}

fn complete(line: &str) -> Vec<char> {
    let chars: Vec<char> = line.chars().collect();
    let mut stack: Vec<char> = Vec::new();
    for char in chars {
        let s = match char {
            '(' | '[' | '{' | '<' => {
                stack.push(char);
                (char, None)
            }
            '>' | '}' | ']' | ')' => (char, stack.pop()),
            _ => panic!("Invalid character!"),
        };
        match s {
            (cur, Some(popped)) => match (popped, cur) {
                ('<', '>') | ('{', '}') | ('[', ']') | ('(', ')') => {}
                (_, _) => {}
            },
            (_, None) => {}
        }
    }
    stack.into_iter().rev().map(reverse).collect()
}

fn main() -> io::Result<()> {
    let input: String = read("10.in").unwrap();
    let total_error_score: usize = input.lines().into_iter().map(evaluate).map(score).sum();
    println!("Total syntax error score (pt1): {:?}", total_error_score);

    let input: String = read("10.in").unwrap();
    let mut scores = input
        .lines()
        .into_iter()
        .map(|line| (line, evaluate(line)))
        .filter(|(_, state)| state == &State::Incomplete)
        .map(|(line, _)| complete(line))
        .map(points)
        .collect::<Vec<usize>>();

    scores.sort_unstable();
    // will always be an odd number
    let middle_score: usize = scores[scores.len() / 2];

    println!("Middle score (pt2): {:?}", middle_score);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_lines() {
        let lines = [
            "([])",
            "{()()()}",
            "<([{}])>",
            "[<>({}){}[([])<>]]",
            "(((((((((())))))))))",
        ];

        for line in lines {
            assert_eq!(evaluate(line), State::Valid);
        }
    }
    #[test]
    fn incomplete_lines() {
        let lines = [
            "([]",
            "{()()()",
            "<([{}])",
            "[<>({}){}[([])<>]",
            "(((((((((()))))))))",
        ];

        for line in lines {
            assert_eq!(evaluate(line), State::Incomplete);
        }
    }

    #[test]
    fn corrupted_lines() {
        let lines = [
            ("(]", State::Corrupted(']')),
            ("{()()()>", State::Corrupted('>')),
            ("(((()))}", State::Corrupted('}')),
            ("<([]){()}[{}])", State::Corrupted(')')),
        ];

        for (line, expected) in lines {
            assert_eq!(evaluate(line), expected);
        }
    }

    #[test]
    fn test_complete() {
        let lines_and_completions = [
            ("[({(<(())[]>[[{[]{<()<>>", "}}]])})]"),
            ("[(()[<>])]({[<{<<[]>>(", ")}>]})"),
            ("(((({<>}<{<{<>}{[]{[]{}", "}}>}>))))"),
            ("{<[[]]>}<{[{[{[]{()[[[]", "]]}}]}]}>"),
            ("<{([{{}}[<[[[<>{}]]]>[]]", "])}>"),
        ];

        for (line, completion) in lines_and_completions {
            let completed: String = complete(line).iter().collect();
            assert_eq!(completed, completion);
        }
    }

    #[test]
    fn test_example() {
        let lines_and_expected = [
            ("[({(<(())[]>[[{[]{<()<>>", State::Incomplete),
            ("[(()[<>])]({[<{<<[]>>(", State::Incomplete),
            ("{([(<{}[<>[]}>{[]{[(<()>", State::Corrupted('}')),
            ("(((({<>}<{<{<>}{[]{[]{}", State::Incomplete),
            ("[[<[([]))<([[{}[[()]]]", State::Corrupted(')')),
            ("[{[{({}]{}}([{[{{{}}([]", State::Corrupted(']')),
            ("{<[[]]>}<{[{[{[]{()[[[]", State::Incomplete),
            ("[<(<(<(<{}))><([]([]()", State::Corrupted(')')),
            ("<{([([[(<>()){}]>(<<{{", State::Corrupted('>')),
            ("<{([{{}}[<[[[<>{}]]]>[]]", State::Incomplete),
        ];

        for (line, expected) in &lines_and_expected {
            assert_eq!(evaluate(line), *expected);
        }

        let lines = lines_and_expected.iter().map(|(l, _)| *l);
        let total_error_score: usize = lines.into_iter().map(evaluate).map(score).sum();
        assert_eq!(total_error_score, 26397);
    }
}
