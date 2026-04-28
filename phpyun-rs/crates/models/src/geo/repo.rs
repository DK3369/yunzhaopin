//! Geo search -- the haversine formula is computed at the MySQL layer,
//! avoiding a full table scan.
//!
//! First a coarse filter via the `x` / `y` indexes (a lon/lat bounding box
//! enclosing the target circle), then a precise filter via `HAVING distance <=`.
//! 1 degree of latitude ~= 111 km; longitude is corrected by cos(latitude).

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct JobNear {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    pub provinceid: i32,
    pub cityid: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub lastupdate: i64,
    pub x: f64,
    pub y: f64,
    /// km
    pub distance: f64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyNear {
    pub uid: u64,
    pub name: Option<String>,
    pub cityid: i32,
    pub logo: Option<String>,
    pub x: f64,
    pub y: f64,
    pub distance: f64,
}

fn bbox(x: f64, y: f64, radius_km: f64) -> (f64, f64, f64, f64) {
    // Latitude delta is fixed; longitude delta is multiplied by 1/cos(lat).
    const KM_PER_DEG: f64 = 111.32;
    let lat_delta = radius_km / KM_PER_DEG;
    let denom = y.to_radians().cos().abs().max(1e-6);
    let lon_delta = radius_km / (KM_PER_DEG * denom);
    (x - lon_delta, x + lon_delta, y - lat_delta, y + lat_delta)
}

pub async fn list_jobs_near(
    pool: &MySqlPool,
    x: f64,
    y: f64,
    radius_km: f64,
    now: i64,
    limit: u64,
) -> Result<Vec<JobNear>, sqlx::Error> {
    let (xmin, xmax, ymin, ymax) = bbox(x, y, radius_km);
    // `phpyun_company_job` declares provinceid/cityid/minsalary/maxsalary as
    // nullable int and x/y as nullable varchar(50) (PHP stores coordinates
    // as strings). The bounding-box predicate on `x BETWEEN ? AND ?` already
    // filters out rows with NULL/empty coords, but COALESCE the projected
    // columns so `JobNear` can use plain `i32 / f64` without sqlx panicking
    // on a NULL slipping past the WHERE on a future schema/index change.
    let sql = r#"
        SELECT id, uid, name, com_name,
               COALESCE(provinceid, 0) AS provinceid,
               COALESCE(cityid, 0) AS cityid,
               COALESCE(minsalary, 0) AS minsalary,
               COALESCE(maxsalary, 0) AS maxsalary,
               COALESCE(lastupdate, 0) AS lastupdate,
               CAST(COALESCE(NULLIF(x, ''), '0') AS DECIMAL(10,6)) AS x,
               CAST(COALESCE(NULLIF(y, ''), '0') AS DECIMAL(10,6)) AS y,
               6371 * 2 * ASIN(
                 SQRT(
                   POW(SIN(RADIANS((y - ?) / 2)), 2)
                   + COS(RADIANS(?)) * COS(RADIANS(y))
                     * POW(SIN(RADIANS((x - ?) / 2)), 2)
                 )
               ) AS distance
        FROM phpyun_company_job
        WHERE state = 1 AND status = 0 AND r_status = 1 AND edate > ?
          AND x BETWEEN ? AND ? AND y BETWEEN ? AND ?
        HAVING distance <= ?
        ORDER BY distance ASC
        LIMIT ?
    "#;
    sqlx::query_as::<_, JobNear>(sql)
        .bind(y)
        .bind(y)
        .bind(x)
        .bind(now)
        .bind(xmin)
        .bind(xmax)
        .bind(ymin)
        .bind(ymax)
        .bind(radius_km)
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn list_companies_near(
    pool: &MySqlPool,
    x: f64,
    y: f64,
    radius_km: f64,
    limit: u64,
) -> Result<Vec<CompanyNear>, sqlx::Error> {
    let (xmin, xmax, ymin, ymax) = bbox(x, y, radius_km);
    // Same NULL-coordinate guard as `list_jobs_near` — see notes there.
    let sql = r#"
        SELECT uid, name,
               COALESCE(cityid, 0) AS cityid,
               logo,
               CAST(COALESCE(NULLIF(x, ''), '0') AS DECIMAL(10,6)) AS x,
               CAST(COALESCE(NULLIF(y, ''), '0') AS DECIMAL(10,6)) AS y,
               6371 * 2 * ASIN(
                 SQRT(
                   POW(SIN(RADIANS((y - ?) / 2)), 2)
                   + COS(RADIANS(?)) * COS(RADIANS(y))
                     * POW(SIN(RADIANS((x - ?) / 2)), 2)
                 )
               ) AS distance
        FROM phpyun_company
        WHERE r_status = 1
          AND x BETWEEN ? AND ? AND y BETWEEN ? AND ?
        HAVING distance <= ?
        ORDER BY distance ASC
        LIMIT ?
    "#;
    sqlx::query_as::<_, CompanyNear>(sql)
        .bind(y)
        .bind(y)
        .bind(x)
        .bind(xmin)
        .bind(xmax)
        .bind(ymin)
        .bind(ymax)
        .bind(radius_km)
        .bind(limit)
        .fetch_all(pool)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbox_basic() {
        // Beijing roughly (116.4, 39.9), radius 10km.
        let (xmin, xmax, ymin, ymax) = bbox(116.4, 39.9, 10.0);
        // Latitude delta = 10/111.32 ~= 0.0898.
        assert!((ymax - ymin - 0.1796).abs() < 0.01);
        // Longitude delta = 10/(111.32*cos(39.9 deg)); cos ~= 0.767.
        assert!((xmax - xmin) > (ymax - ymin));
    }

    #[test]
    fn bbox_pole_safety() {
        // Near the poles cos is tiny; the clamp avoids a divide-by-zero panic.
        let _ = bbox(0.0, 89.999, 10.0);
    }
}
