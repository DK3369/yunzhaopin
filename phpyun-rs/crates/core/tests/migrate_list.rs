/// Diagnostic: print all migration versions embedded in the binary so we
/// know whether the `sqlx::migrate!` macro picked up the files.
#[test]
fn list_embedded_migrations() {
    let m = sqlx::migrate!("../../migrations/sqlx");
    let count = m.migrations.len();
    println!("Embedded migrations: {}", count);
    for x in m.migrations.iter() {
        println!("  v={} desc={}", x.version, x.description);
    }
    assert!(count > 0, "no migrations were embedded");
}
