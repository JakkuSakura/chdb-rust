use chdb_rust::arg::Arg;
use chdb_rust::execute;
use chdb_rust::format::InputFormat;
use chdb_rust::format::OutputFormat;
use chdb_rust::log_level::LogLevel;
use chdb_rust::session::SessionBuilder;

#[test]
fn test_stateful() -> Result<(), chdb_rust::error::Error> {
    //
    // Create session.
    //
    let tmp = tempdir::TempDir::new("chdb-rust")?;
    let session = SessionBuilder::new()
        .with_data_path(tmp.path())
        .with_arg(Arg::LogLevel(LogLevel::Debug))
        .with_arg(Arg::Custom("priority".into(), Some("1".into())))
        .with_auto_cleanup(true)
        .build()?;

    //
    // Create database.
    //

    session.execute(
        "CREATE DATABASE IF NOT EXISTS demo; USE demo",
        Some(&[Arg::MultiQuery]),
    )?;

    //
    // Create table.
    //

    session.execute(
        "CREATE TABLE logs (id UInt64, msg String) ENGINE = Memory",
        None,
    )?;

    //
    // Insert into table.
    //

    session.execute("INSERT INTO logs (id, msg) VALUES (1, 'test')", None)?;

    //
    // Select from table.
    //

    let result = session
        .execute(
            "SELECT * FROM logs",
            Some(&[Arg::OutputFormat(OutputFormat::JSONEachRow)]),
        )?
        .unwrap();

    assert_eq!(result.data_utf8_lossy(), "{\"id\":1,\"msg\":\"test\"}\n");
    Ok(())
}

#[test]
fn test_stateless() {
    let query = format!(
        "SELECT * FROM file('tests/logs.csv', {})",
        InputFormat::CSV.as_str()
    );

    let result = execute(
        &query,
        Some(&[Arg::OutputFormat(OutputFormat::JSONEachRow)]),
    )
    .unwrap()
    .unwrap();

    assert_eq!(result.data_utf8_lossy(), "{\"id\":1,\"msg\":\"test\"}\n");
}
