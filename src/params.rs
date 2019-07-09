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

    pub fn with_extra_fields(self, fields: Vec<String>) -> Params {
        let mut params = self;
        params.extra_fields = fields;
        params
    }

    pub fn to_string(self) -> String {
        let mut res = "?".to_string();

        if !self.query.is_empty() {
            res.push_str("&q=");
            for (key, value) in  self.query.into_iter(){
                res.push_str(&key);
                if value.is_empty() {
                    res.push_str(":");
                    res.push_str(&value);
                }
                res.push_str("+");
            };
        }

        if !self.extra_fields.is_empty() {
            res.push_str("&extra_fields=");
            res.push_str(&self.extra_fields.join(","));
        }

        self.lang.map(|l|{
                res.push_str("&lang=");
                res.push_str(&l.to_string())
        });

        self.limit.map(|l|{
                res.push_str("&limit=");
                res.push_str(&l.to_string())
        });
        self.offset.map(|o|{
                res.push_str("&offset=");
                res.push_str(&o.to_string())
        });

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_string() {
        use super::Params;
        let p = Params::new()
            .with_extra_fields(vec!["user".to_string(), "user.avatar".to_string()])
            .limit(30)
            .offset(10);
        assert_eq!(p.to_string(), "?&extra_fields=user,user.avatar&limit=30&offset=10");
    }
}
