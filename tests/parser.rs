use textfsm_rs::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod end_to_end;

fn print_pair(indent: usize, pair: &Pair<'_, Rule>) {
    // println!("Debug: {:#?}", &pair);
    let spaces = " ".repeat(indent);
    println!("{}Rule:    {:?}", spaces, pair.as_rule());
    println!("{}Span:    {:?}", spaces, pair.as_span());
    println!("{}Text:    {}", spaces, pair.as_str());
    for p in pair.clone().into_inner() {
        print_pair(indent + 2, &p);
    }
}

fn main() {
    for arg in std::env::args().skip(1) {
        // println!("Reading file {}", &arg);
        let template = std::fs::read_to_string(&arg).expect("File read failed");
        let template = format!("{}\n", template);

        match TextFSMParser::parse(Rule::file, &template) {
            Ok(pairs) => {
                for pair in pairs {
                    print_pair(0, &pair);
                }
            }
            Err(e) => panic!("file {} Error: {}", &arg, e),
        }
    }
}
