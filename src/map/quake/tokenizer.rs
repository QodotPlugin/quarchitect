#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Token {
    Comment(String),
    OpenBrace,
    CloseBrace,
    Property(Property),
    BrushPlane(String),
    Unrecognized(String),
}

impl Token {
    fn comment(string: &str) -> Token {
        Token::Comment(String::from(string))
    }

    fn property(key: &str, value: &str) -> Token {
        Token::Property(Property::new(&key, &value))
    }

    fn brush_plane(string: &str) -> Token {
        Token::BrushPlane(String::from(string))
    }

    fn unrecognized(string: &str) -> Token {
        Token::Unrecognized(String::from(string))
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Property {
    pub key: String,
    pub value: String,
}

impl Property {
    pub fn new(key: &str, value: &str) -> Property {
        let key = String::from(key);
        let value = String::from(value);
        Property { key, value }
    }
}

pub fn run(file_string: String) -> Vec<Token> {
    println!("TODO-3: Rewrite with nom");

    file_string
        .lines()
        .map(|line| match line.chars().next() {
            Some('/') => Token::comment(line),
            Some('"') => {
                let mut comps = line.split('"');
                comps.next();
                let key = comps.next().unwrap();
                comps.next();
                let value = comps.next().unwrap();
                Token::property(key, value)
            }
            Some('{') => Token::OpenBrace,
            Some('}') => Token::CloseBrace,
            Some('(') => Token::brush_plane(&line),
            _ => Token::unrecognized(&line)
        })
        .collect()
}


#[derive(Debug)]
pub struct Diff<'a> {
    pub added: Vec<&'a Token>,
    pub removed: Vec<&'a Token>
}

impl<'a> Diff<'a> {
    fn new(added: Vec<&'a Token>, removed: Vec<&'a Token>) -> Diff<'a> {
        Diff { added, removed }
    }
}

pub fn diff_tokens<'a>(tokens_a: &'a[Token], tokens_b: &'a[Token]) -> Diff<'a> {
    let size_x = tokens_a.len();
    let size_y = tokens_b.len();
    let mut l: Vec<Vec<usize>> = Vec::new();
    l.resize(size_x + 1, Vec::<usize>::new());
    for v in &mut l {
        v.resize(size_y + 1, 0);
    }

    for index_x in 0..size_x {
        for index_y in 0..size_y {
            if index_x == 0 || index_y == 0 {
                l[index_x][index_y] = 0;
            }
            else if tokens_a[index_x-1] == tokens_b[index_y-1] {
                l[index_x][index_y] = l[index_x-1][index_y-1] + 1;
            }
            else {
                l[index_x][index_y] = std::cmp::max(l[index_x-1][index_y], l[index_x][index_y-1]);
            }
        }
    }

    let mut index = l[size_x-1][size_y-1];
    let mut lcs: Vec<Option<&Token>> = Vec::new();
    lcs.resize(index, None);


    let mut i = size_x - 1;
    let mut j = size_y - 1;
    while i > 0 && j > 0 {
        if tokens_a[i-1] == tokens_b[j-1] {
            lcs[index-1] = Some(&tokens_a[i-1]);
            i -= 1;
            j -= 1;
            index -= 1;
        }
        else if l[i-1][j] > l[i][j-1] {
            i -= 1;
        }
        else {
            j -= 1;
        }
    }

    let lcs: Vec<&Token> = lcs.iter().map(|token| {
        token.unwrap()
    }).collect();

    let mut removed: Vec<&Token> = Vec::new();
    for token in tokens_a {
        if lcs.iter().find(|comp| **comp == token).is_none() {
            removed.push(token);
        }
    }

    let mut added: Vec<&Token> = Vec::new();
    for token in tokens_b {
        if lcs.iter().find(|comp| **comp == token).is_none() {
            added.push(token);
        }
    }

    Diff::new(added, removed)
}