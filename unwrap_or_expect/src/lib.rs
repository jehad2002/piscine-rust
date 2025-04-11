pub enum Security {
    Unknown, // no nessage 
    Message, // mess in er
    Warning, // warning in giniral
    NotFound, // mess wrong with original text
    UnexpectedUrl, // change the true ok, err
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        // using unwrap to take the res if ok return str if err return panic
        Security::Unknown => server.unwrap().to_string(),
        // if err dont print the original err and if no err return str 
        Security::Message => server.unwrap_or_else(|_| panic!("ERROR: program stops")).to_string(),
        // if ok return url and if err return warrning ...
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),

        Security::NotFound => match server {
                    // using match on server if ok retyrn url and if err return not found
            Ok(url) => url.to_string(),
            Err(msg) => format!("Not found: {}", msg),
        },

        Security::UnexpectedUrl => match server {
            // change if ok return panic and orin url

            Ok(url) => panic!("{}", url),
                        // if err return str
            Err(msg) => msg.to_string(),
        },
    }
}
