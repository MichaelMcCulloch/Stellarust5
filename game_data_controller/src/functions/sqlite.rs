use game_data_info_struct_reader::ModelDataPoint;
use rusqlite::Connection;

const SQL_SEL_TABLE_NAMES: &str =
    "SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';";
const SQL_CREATE_TABLE_DATA_POINTS: &str = "CREATE TABLE data_points ( blob TEXT NOT NULL );";
const SQL_SEL_DATA_POINTS: &str = "SELECT * FROM data_points;";
const SQL_INSERT_DATA_POINT: &str = "INSERT INTO data_points (blob) values (?)";

pub(crate) fn write_to_db(data_point: &ModelDataPoint, db_connection: &Connection) {
    let msg = serde_json::to_string(data_point).unwrap();
    db_connection.execute(SQL_INSERT_DATA_POINT, [msg]).unwrap();
}

pub(crate) fn query_models(db_connection: &Connection) -> anyhow::Result<Vec<ModelDataPoint>> {
    let tables = db_connection
        .prepare(SQL_SEL_TABLE_NAMES)?
        .query_map([], |row| row.get::<_, String>(0))?
        .filter_map(|s| s.ok())
        .collect::<Vec<_>>();

    if !tables.contains(&"data_points".to_string()) {
        db_connection.execute(SQL_CREATE_TABLE_DATA_POINTS, [])?;
        log::info!("Populating empty database `stellarust_model_history.db` in game folder");
        Ok(vec![])
    } else {
        let extant_data = db_connection
            .prepare(SQL_SEL_DATA_POINTS)?
            .query_map([], |row| row.get::<_, String>(0))?
            .filter_map(|s| s.ok())
            .filter_map(|s| serde_json::from_str(s.as_str()).ok())
            .map(|model: ModelDataPoint| {
                log::trace!("Discovered {:?} -- {}", model.date, model.campaign_name);
                model
            })
            .collect::<Vec<ModelDataPoint>>();

        Ok(extant_data)
    }
}
