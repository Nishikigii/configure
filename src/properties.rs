#[derive(Debug, Clone)]
pub struct Properties
{
    indexes : Vec<String>,          // keys
    entities: Vec<Option<String>>,  // values
    access  : bool                  // parsing completion status
}

impl Properties 
{
    pub fn new()-> Self 
    {
        Self { indexes: vec![], entities: vec![], access: true }
    }

    pub fn parse( content: &String )-> Self
    {
        type Keys = Vec<String>;
        type Vals = Vec<Option<String>>;

        let mut k: Vec<char> = vec![];
        let mut v: Vec<char> = vec![];
        let mut keys: Keys = vec![];
        let mut vals: Vals = vec![];
        let mut symbol  = false;
        let mut scope   = false;
        for char in content.chars()
        {
            match char 
            {
                ' ' | '\n' | '\t' => 
                { 
                    if scope { v.push(char); continue } 
                }
                '=' | ':' =>
                {
                    if scope { v.push(char); continue }
                    keys.push( String::from_iter(&k) );
                    symbol = true;
                    k.clear();
                }
                '\"' =>
                {
                    if symbol { scope = true; symbol = false; continue }
                    scope = false;
                    vals.push( Some(String::from_iter(&v)) );
                    v.clear();
                }
                _ =>
                { 
                    if scope { v.push(char); continue };
                    k.push(char);
                }
            }
        }
        Self { indexes: keys, entities: vals, access: true }
    }
}

#[test]
fn test()
{
    use std::fs::read_to_string;

    let content = read_to_string("./cache/properties").unwrap();
    let properties = Properties::parse(&content);

    println!("{properties:?}")
}
