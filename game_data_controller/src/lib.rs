mod filter;
mod scan_root;

pub mod controller;

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::*;
    #[test]
    fn test_name() {
        let conn = Connection::open("stellarust.db").unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';",
            )
            .unwrap();

        let tables = stmt
            .query_map([], |row| Ok(row.get::<_, String>(0).unwrap()))
            .unwrap()
            .map(|s| s.unwrap())
            .collect::<Vec<_>>();
        if !tables.contains(&"campaigns".to_string()) {
            conn.execute(
                "CREATE TABLE campaigns (
                    id      INTEGER PRIMARY KEY,
                    name    TEXT NOT NULL
                )",
                [],
            )
            .unwrap();
        }
    }
}
