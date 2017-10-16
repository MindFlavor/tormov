#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Move,
    Link,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    pub regex: String,
    pub destination: String,
    pub action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub skipextension: String,
    pub matches: Vec<Match>,
}

impl Config {
    pub fn new(se: &str) -> Config {
        Config {
            skipextension: se.to_owned(),
            matches: Vec::new(),
        }
    }
}

impl Match {
    pub fn new(regex: &str, destination: &str, action: Action) -> Match {
        Match {
            regex: regex.to_owned(),
            destination: destination.to_owned(),
            action: action,
        }
    }
}

#[test]
fn test_serialize_deserialize() {
    use serde_json;

    let mut c = Config::new("tormed");

    c.matches.push(Match {
        regex: "*Arrow*".to_owned(),
        destination: "/mainpool/shared/shows/Arrow".to_owned(),
        action: Action::Move,
    });

    c.matches.push(Match {
        regex: "*Big.Bang*".to_owned(),
        destination: "/mainpool/shared/shows/The.big.bang.theory".to_owned(),
        action: Action::Link,
    });


    let s = serde_json::to_string(&c).unwrap();

    println!("{}", s);

    let c2: Config = serde_json::from_str(&s).unwrap();

    assert_eq!(c2.skipextension, c.skipextension);
    assert_eq!(c2.matches.len(), 2);
}
