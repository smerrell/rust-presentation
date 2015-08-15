use regex::Regex;

pub fn errors() {
    // enum Option<T> {
    //     Some(T),
    //     None
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let address = to_sip_address("sip:username@test.com:5600");
    match address {
        Ok(a) => { println!("Parsed! {:?}", a); },
        Err(e) => { println!("Error! {:?}", e); },
    }
}

#[derive(Debug)]
struct Sip {
    protocol: String,
    user: String,
    domain: String,
    port: Option<i32>,
}

#[derive(Debug)]
enum ParseError {
    EmptyString,
    InvalidFormat
}

fn to_sip_address(value: &str) -> Result<Sip, ParseError> {
    if value.is_empty() {
        return Err(ParseError::EmptyString)
    }

    let re = Regex::new(r"^(?P<protocol>sip):(?P<user>[^@]+)@(?P<domain>[^:]+):?(?P<port>\d+)?$").unwrap();
    if !re.is_match(&value) {
        return Err(ParseError::InvalidFormat);
    }

    let captures = re.captures(&value).unwrap();

    let mut port = None;
    if let Some(p) = captures.name("port") {
        port = Some(p.parse::<i32>().unwrap());
    }

    let s = Sip {
        protocol: captures.name("protocol").unwrap().to_string(),
        user: captures.name("user").unwrap().to_string(),
        domain: captures.name("domain").unwrap().to_string(),
        port: port,
    };

    Ok(s)
}
