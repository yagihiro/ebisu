use url::Url;
use mysql;
use ansi_term::Colour::*;

#[derive(Debug)]
pub struct DB {
    database_type: String,
    host: String,
    port: u16,
    socket: String,
    user: String,
    pass: String,
    database: String,

    connection: Option<mysql::Pool>,
}

pub fn connect_with_url(url: &str) -> DB {
    let parsed_url: Url = Url::parse(url).unwrap();
    let mut host = "".to_string();
    let mut port: u16 = 3306;
    let mut socket = "".to_string();
    let mut password = "".to_string();
    let mut database = "".to_string();

    if let Some(x) = parsed_url.host_str() {
        host = x.to_string();
    }
    if let Some(x) = parsed_url.port() {
        port = x;
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

    let mut db = DB {
        database_type: parsed_url.scheme().to_string(),
        host: host,
        port: port,
        socket: socket,
        user: parsed_url.username().to_string(),
        pass: password,
        database: database,
        connection: None,
    };

    let options = db_to_mysql_options(&db);
    let pool = mysql::Pool::new(options);

    match pool {
        Err(e) => {
            warn!(target: "ebisu", "Failed MYSQL connection established: {:?}", e);
            db.connection = None
        }
        Ok(v) => {
            info!(target: "ebisu", "MYSQL Connection established: pool: {:?}", v);
            db.connection = Some(v);
        }
    }

    db
}

fn db_to_mysql_options(db: &DB) -> mysql::conn::Opts {
    let mut builder = mysql::conn::OptsBuilder::default();
    builder.user(Some(db.user()))
        .pass(Some(db.password()))
        .ip_or_hostname(Some(db.host()))
        .tcp_port(db.port())
        .db_name(Some(db.database()))
        .prefer_socket(false);
    builder.into()
}

impl DB {
    pub fn database_type(&self) -> &str {
        &self.database_type
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
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

    pub fn has_connection(&self) -> bool {
        self.connection.is_some()
    }

    pub fn run(&mut self, sql: &str) {
        if let Some(x) = self.connection.as_mut() {
            debug!(target: "ebisu", "{}", Purple.paint(format!("EXEC SQL: {}", sql)));
            let result = x.prep_exec(sql, ());
            match result {
                Err(e) => {
                    warn!(target: "ebisu", "{}", Red.paint(format!("Failed run() with query: {}, error: {}", sql, e)));
                }
                Ok(_) => {}
            }
        }
    }

    pub fn from(&mut self, table: &str) -> DataSet {
        DataSet {
            db: self,
            table: table.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Error {

}

#[derive(Debug)]
pub struct Row {

}

#[derive(Debug)]
pub struct DataSet<'a> {
    db: &'a mut DB,
    table: String,
}

impl<'a> Iterator for DataSet<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Row {})
    }
}

impl<'a> DataSet<'a> {
    // where
    pub fn filter(&'a mut self) -> &'a mut DataSet {
        self
    }

    // order
    pub fn order(&'a mut self) -> &'a mut DataSet {
        self
    }

    // limit
    pub fn limit(&'a mut self) -> &'a mut DataSet {
        self
    }

    // offset
    pub fn offset(&'a mut self) -> &'a mut DataSet {
        self
    }

    // insert
    pub fn insert(&'a mut self, params: &Vec<(String, String)>) -> Result<Row, Error> {
        Ok(Row {})
    }

    // update
    pub fn update(&'a mut self) -> Result<Row, Error> {
        Ok(Row {})
    }

    // delete
    pub fn delete(&'a mut self) -> Result<Row, Error> {
        Ok(Row {})
    }

    // count, sum, avg, max, min
    // group, having,

    // join

    // sub query

    // select for update
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
