//! Guest quick-apply snapshot into `phpyun_temporary_resume`.

use phpyun_core::{clock, ApiError, AppResult, AppState};
use phpyun_models::resume::{expect, repo as resume_repo};
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

/// PHP `fastToudi` after register: write resume basics + a job expectation.
pub async fn after_register(
    state: &AppState,
    uid: u64,
    uname: &str,
    sex: i32,
    birthday: &str,
    edu: i32,
    telphone: &str,
    expect_name: &str,
    job_classid: i64,
    city_classid: i64,
    salary: i32,
    hy: i32,
) -> AppResult<()> {
    let pool = state.db.pool();
    let now = clock::now_ts();
    resume_repo::ensure_uid_only(pool, uid).await?;
    let birthday = birthday.trim();
    resume_repo::update(
        pool,
        uid,
        resume_repo::ResumeUpdate {
            name: Some(uname),
            nametype: None,
            sex: Some(sex),
            birthday: if birthday.is_empty() {
                None
            } else {
                Some(birthday)
            },
            marriage: None,
            education: Some(edu),
            telphone: Some(telphone),
            email: None,
            photo: None,
            exp: None,
            living: None,
            domicile: None,
            height: None,
            weight: None,
            address: None,
            description: None,
            qq: None,
            idcard: None,
            idcard_pic: None,
        },
        now,
    )
    .await?;
    let input = expect::ExpectInput {
        name: Some(expect_name),
        job_classid,
        city_classid,
        salary,
        minsalary: salary,
        maxsalary: None,
        r#type: 0,
        report: 0,
        jobstatus: 0,
        hy,
    };
    if let Some(eid) = expect::find_default_id_by_uid(pool, uid).await? {
        let _ = expect::update(pool, eid, uid, &input, now).await?;
    } else {
        let _ = expect::create(pool, uid, &input, now).await?;
    }
    Ok(())
}
