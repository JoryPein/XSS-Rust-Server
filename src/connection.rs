use postgres::{Client, NoTls, Error};

pub struct PostgresClient {
    client: Client,
}

impl PostgresClient {

    pub fn new(host: &str, dbname: &str, user: &str, password: &str) -> Result<PostgresClient, Error> {
        let db_config = format!(
            "host={} dbname={} user={} password={}", 
            host, dbname, user, password
        );
        Ok(PostgresClient {
            client: Client::connect(&db_config, NoTls)?
        })
    }

    pub fn create_table(&mut self) {
        let exec_result = self.client.batch_execute("
            CREATE TABLE public.data (
                id           SERIAL      NOT NULL PRIMARY KEY,
                http_version VARCHAR(20) NULL,
                method       VARCHAR(20) NULL,
                url          TEXT        NULL,
                headers      TEXT        NULL,
                body         TEXT        NULL,
                peer_addr    VARCHAR(40) NULL,
                created_at   TIMESTAMPTZ default CURRENT_TIMESTAMP
            )
        ");
        if let Ok(_) = exec_result {
            // println!("create table success");
        } else {
            // println!("create table failed: {:?}", exec_result);
        }
    }

    pub fn insert_data(&mut self, http_version:&str, method: &str, url: &str, headers: &str, body: &str, peer_addr: &str) {
        if let Ok(_) = self.client.execute(
            "INSERT INTO public.data (
                http_version, method, url, headers, body, peer_addr
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )",
            &[&http_version, &method, &url, &headers, &body, &peer_addr],
        ) {
            // println!("inserted data success");
        } else {
            // println!("inserted data failed");
        }
    }
}