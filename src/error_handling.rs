use regex::Regex;

pub fn errors() {
    // enum Option<T> {
    //     Some(T),
    //     None
    // }

    let foobar = foo_to_bar("foo");
    println!("foobar: {:?}", foobar);

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let good_address = "sip:username@test.com:5600";
    let no_port = "sip:username@test.com";
    let bad_address = "thisisnotvalid";

    let address = to_sip_address(good_address);
    match address {
        Ok(a) => { println!("Parsed! {:?}", a); },
        Err(e) => { println!("Error! {:?}", e); },
    }

    // panicking is also ok as Rust cleans up itself afterwards
    // segfaults are never ok by Rust's design

    //assert!("foo" == "bar");
    let is_it_42 = is_42(4);
    println!("Is it 42? {}", is_it_42);
}

fn foo_to_bar(foo: &str) -> Option<&str> {
    if foo == "foo" {
        Some("bar")
    } else {
        None
    }
}

fn is_42(x: i32) -> bool {
    if x == 42 {
        true
    } else {
        err("Not 42, panic!")
    }
}

// diverging functions tell Rust this function will end in a panic
fn err(message: &str) -> ! {
    println!("{}", message);
    panic!();
}

fn to_sip_address(value: &str) -> Result<Sip, ParseError> {
    if value.is_empty() {
        return Err(ParseError::EmptyString)
    }

    let re = Regex::new(r"^(?P<protocol>sip):(?P<user>[^@]+)@(?P<domain>[^:]+):?(?P<port>\d+)?$").unwrap();
    if !re.is_match(&value) {
        let err_message = format!("The string `{}` is not a valid SIP address", &value);
        return Err(ParseError::InvalidFormat(err_message));
    }

    let captures = re.captures(&value).unwrap();

    let mut port = None;
    if let Some(p) = captures.name("port") {
        port = Some(p.parse::<i32>().unwrap());
    }

    let sip = Sip {
        protocol: captures.name("protocol").unwrap().to_string(),
        user: captures.name("user").unwrap().to_string(),
        domain: captures.name("domain").unwrap().to_string(),
        port: port,
    };

    Ok(sip)
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
    InvalidFormat(String)
}
