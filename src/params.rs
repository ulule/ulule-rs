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

    pub fn to_string(self) -> String {
        let mut res = Vec::new();

        if !self.query.is_empty() {
            let mut q = Vec::new();
            for (key, value) in  self.query.into_iter(){
                let mut tuple = key;
                if !value.is_empty() {
                    tuple.push_str(":");
                    tuple.push_str(&value);
                }
                q.push(tuple);
            };
            res.push(format!("q={}",q.join("+")));
        }

        if !self.extra_fields.is_empty() {
            res.push(format!("extra_fields={}", &self.extra_fields.join(",")));
        }

        self.lang.map(|l|{res.push(format!("lang={}", &l.to_string()));});
        self.limit.map(|l|{res.push(format!("limit={}", &l.to_string()));});
        self.offset.map(|o|{res.push(format!("offset={}", &o.to_string()));});

        if res.is_empty() {
            return "".to_string()
        }

        format!("?{}", res.join("&"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_string() {
        use super::Params;
        let p = Params::new()
            .with_extra_fields(vec!["user".to_string(), "user.avatar".to_string()])
            .add_query("tag_id", "1337")
            .add_query("beer", "")
            .limit(30)
            .offset(10);
        assert_eq!(p.to_string(), "?q=tag_id:1337+beer&extra_fields=user,user.avatar&limit=30&offset=10");
    }
}
