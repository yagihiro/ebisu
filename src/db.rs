use url::Url;

#[derive(Debug)]
pub struct DB {
    database_type: String,
    host: String,
    port: i32,
    socket: String,
    user: String,
    pass: String,
    database: String,
}

pub fn connect_with_url(url: &str) -> DB {
    let parsed_url: Url = Url::parse(url).unwrap();
    let mut host = "".to_string();
    let mut port: i32 = 3306;
    let mut socket = "".to_string();
    let mut password = "".to_string();
    let mut database = "".to_string();

    if let Some(x) = parsed_url.host_str() {
        host = x.to_string();
    }
    if let Some(x) = parsed_url.port() {
        port = x as i32;
    }
    if let Some(x) = parsed_url.query_pairs().find(|x| x.0 == "socket") {
        socket = x.1.into_owned();
    }
    if let Some(x) = parsed_url.password() {
        password = x.to_string();
    }
    if let Some(x) = parsed_url.query_pairs().find(|x| x.0 == "database") {
        database = x.1.into_owned();
    }

    DB {
        database_type: parsed_url.scheme().to_string(),
        host: host,
        port: port,
        socket: socket,
        user: parsed_url.username().to_string(),
        pass: password,
        database: database,
    }
}

impl DB {
    pub fn database_type(&self) -> &str {
        &self.database_type
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn socket(&self) -> &str {
        &self.socket
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn password(&self) -> &str {
        &self.pass
    }

    pub fn database(&self) -> &str {
        &self.database
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_connect_with_url() {
        let db: DB = connect_with_url("mysql://127.0.0.1");
        assert_eq!(db.database_type(), "mysql");
        assert_eq!(db.host(), "127.0.0.1");
        assert_eq!(db.port(), 3306);
        assert_eq!(db.socket(), "");
        assert_eq!(db.user(), "");
        assert_eq!(db.password(), "");
        assert_eq!(db.database(), "");

        let db: DB = connect_with_url("mysql://?socket=/tmp/mysql.sock");
        assert_eq!(db.database_type(), "mysql");
        assert_eq!(db.host(), "");
        assert_eq!(db.port(), 3306);
        assert_eq!(db.socket(), "/tmp/mysql.sock");
        assert_eq!(db.user(), "");
        assert_eq!(db.password(), "");
        assert_eq!(db.database(), "");

        let db: DB = connect_with_url("mysql://user123:password123@127.0.0.1?database=db123");
        assert_eq!(db.database_type(), "mysql");
        assert_eq!(db.host(), "127.0.0.1");
        assert_eq!(db.port(), 3306);
        assert_eq!(db.socket(), "");
        assert_eq!(db.user(), "user123");
        assert_eq!(db.password(), "password123");
        assert_eq!(db.database(), "db123");
    }
}
