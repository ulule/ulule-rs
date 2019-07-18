use std::str::FromStr;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Params {
    query: Vec<(String, String)>,
    extra_fields: Vec<String>,
    lang: Option<String>,
    limit: Option<u64>,
    offset: Option<u64>,
}

impl Params {
    pub fn new() -> Params {
        Params{
            query: Default::default(),
            lang: None,
            limit: None,
            offset: None,
            extra_fields: Default::default(),
        }
    }

    pub fn lang(self, l: impl Into<String>) -> Params {
        let mut params = self;
        params.lang = Some(l.into());
        params
    }

    pub fn limit(self, l: u64) -> Params {
        let mut params = self;
        params.limit = Some(l);
        params
    }

    pub fn offset(self, o: u64) -> Params {
        let mut params = self;
        params.offset = Some(o);
        params
    }

    pub fn add_query<K, V>(self, key: K, value: V) -> Params
        where K: Into<String>, V: Into<String> {
            let mut params = self;
            params.query.push((key.into(), value.into()));
            params
        }

    pub fn with_extra_fields(self, fields: Vec<String>) -> Params {
        let mut params = self;
        params.extra_fields = fields;
        params
    }
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = Vec::new();

        if !self.query.is_empty() {
            let q: Vec<String> = self.query.iter().map(|(key, value)| {
                let mut tuple = key.clone();
                if !value.is_empty() {
                    tuple.push_str(":");
                    tuple.push_str(&value);
                }; tuple
            }).collect();
            res.push(format!("q={}", q.join("+")));
        }

        if !self.extra_fields.is_empty() {
            res.push(format!("extra_fields={}", &self.extra_fields.join(",")));
        }

        self.lang.as_ref().map(|l|{res.push(format!("lang={}", l));});
        self.limit.map(|l|{res.push(format!("limit={}", l));});
        self.offset.map(|o|{res.push(format!("offset={}", o));});

        write!(f, "?{}", res.join("&"))
    }
}

impl FromStr for Params {
    type Err = ParseIntError;
    fn from_str(input: &str) -> Result<Params, Self::Err> {
        let mut params = Params::new();
        let p: Vec<&str> = input.trim_start_matches('?').split('&').collect();
        for v in &p {
            let tuple: Vec<&str> = v.split('=').collect();
            if tuple.len() > 1 {
                match tuple[0] {
                    "limit" => params.limit = Some(tuple[1].parse::<u64>()?),
                    "offset" => params.offset = Some(tuple[1].parse::<u64>()?),
                    "lang" => params.lang = Some(tuple[1].to_string()),
                    "extra_fields" => params.extra_fields = tuple[1]
                        .split(',').map(|i| i.to_string()).collect(),
                    "q" => {
                        let queries: Vec<&str> = tuple[1].split(|c| c == '+' || c == ' ').collect();
                        for query in queries {
                            let tuple:Vec<&str> = query.split(":").collect();
                            match tuple.len() {
                                1 => params.query.push((tuple[0].to_string(), "".to_string())),
                                2 => params.query.push((tuple[0].to_string(), tuple[1].to_string())),
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                }
            }
        }
        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use super::Params;
    use std::str::FromStr;
    #[test]
    fn from_string() {
        assert_eq!(
            Params::from_str("?q=tag_id:1337+beer&extra_fields=user,user.avatar&limit=30&offset=10&lang=fr").unwrap(),
            Params::new()
            .with_extra_fields(vec!["user".to_string(), "user.avatar".to_string()])
            .add_query("tag_id", "1337")
            .add_query("beer", "")
            .limit(30)
            .offset(10)
            .lang("fr"));
    }
    #[test]
    fn to_string() {
        let p = Params::new()
            .with_extra_fields(vec!["user".to_string(), "user.avatar".to_string()])
            .add_query("tag_id", "1337")
            .add_query("beer", "")
            .limit(30)
            .offset(10)
            .lang("fr");
        assert_eq!(p.to_string(), "?q=tag_id:1337+beer&extra_fields=user,user.avatar&lang=fr&limit=30&offset=10");
    }
}
