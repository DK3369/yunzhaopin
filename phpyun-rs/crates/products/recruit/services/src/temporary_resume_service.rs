//! Guest quick-apply snapshot into `phpyun_temporary_resume`.

use phpyun_core::{ApiError, AppResult, AppState};
use phpyun_models::temporary_resume::repo as temp_repo;

pub struct Snapshot<'a> {
    pub name: &'a str,
    pub uname: &'a str,
    pub edu: i32,
    pub sex: i32,
    pub exp: i32,
    pub telphone: &'a str,
    pub birthday: &'a str,
    pub hy: i32,
    pub job_classid: &'a str,
    pub city_classid: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub rid: i32,
}

pub async fn insert_snapshot(state: &AppState, row: Snapshot<'_>) -> AppResult<u64> {
    if row.uname.trim().is_empty() || row.telphone.trim().len() < 11 {
        return Err(ApiError::param_invalid("temporary_resume"));
    }
    Ok(temp_repo::insert(
        state.db.pool(),
        temp_repo::Insert {
            name: row.name,
            uname: row.uname,
            edu: row.edu,
            sex: row.sex,
            exp: row.exp,
            telphone: row.telphone,
            birthday: row.birthday,
            hy: row.hy,
            job_classid: row.job_classid,
            city_classid: row.city_classid,
            provinceid: row.provinceid,
            cityid: row.cityid,
            three_cityid: row.three_cityid,
            minsalary: row.minsalary,
            maxsalary: row.maxsalary,
            rid: row.rid,
        },
    )
    .await?)
}
