use std::fs::File;
use std::io::{self, BufRead};
// use std::iter;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_input<P>(filename: P) -> Option<Vec<String>>
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        Option::from(
            lines
                .map(|line| match line {
                    Ok(l) => l,
                    _ => "".to_string(),
                })
                .filter(|l| *l != "".to_string())
                .collect::<Vec<String>>(),
        )
        // for line in lines {
        // if let Ok(l) = line {}
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
