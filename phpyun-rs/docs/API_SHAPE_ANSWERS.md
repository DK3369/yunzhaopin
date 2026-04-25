# Frontend API Shape Answers

Answers to the 10 shape-related questions raised by the frontend, plus a list of real paths that were mistakenly reported as missing.

---

## 1. Endpoints Mistakenly Reported as Missing (path spelling differences)

| Path the frontend assumed | Actual path | Notes |
|---|---|---|
| `POST /v1/mcenter/account/logout-apply` | `POST /v1/mcenter/account/logout/apply` | Includes the `/apply` sub-path |
| `transferInfo` (account split) | `POST /v1/mcenter/account/split` | body: `{old_password, new_username, new_password}` -> returns `{old_uid, new_uid}` |
| Personal mobile / email verification | `POST /v1/mcenter/cert/mobile/send`, `/cert/mobile/verify`, `/cert/email/send` | Verification for changing contact information |
| part (part-time) module | `/v1/mcenter/my-part-applications`, etc. | Handler already exists, see `mcenter/part.rs` |

---

## 2. Answers to the 10 Shape Questions

### Q1. Does `POST /v1/wap/login` need `usertype`?

**No.** The login layer does not differentiate by identity:

- Current `LoginForm` fields (captcha has been hardened to **required**):
  ```json
  {
    "username": "zhangsan",
    "password": "s3cret",
    "authcode": "ABCD",
    "captcha_cid": "uuid-from-/wap/captcha"
  }
  ```
- The server uses `username` to look up `phpyun_member`. The `usertype` of each row in the database is the identity (1 = job seeker / 2 = employer / 3 = administrator). The JWT claim in the response carries `usertype`.

**Alignment with PHPYun behavior**: In the PHP version, the same username also corresponds to only one identity. If the same phone number needs to be used as both "job seeker" and "employer", use `POST /v1/mcenter/account/split` (account split).

### Q2. `GET /v1/mcenter/dashboard` — Is it the same endpoint for job seekers vs. employers?

**No.** They are two separate endpoints with different fields:

| Endpoint | User | Fields |
|---|---|---|
| `GET /v1/mcenter/dashboard` | Job seeker | `unread_messages`, `unread_chats`, `apply_count`, `interview_count`, `favorite_count`, `view_count`, `integral_balance`, `signday` |
| `GET /v1/mcenter/com-dashboard` | Employer | `applies_received`, `applies_unread`, `interviews_sent`, `resume_downloads`, `unread_chats`, `unread_messages`, `integral_balance` |

Non-employer users calling `com-dashboard` will receive 403.

### Q3. Does `GET /v1/mcenter/my-applications` support the `?state=1|3|4|7` filter?

**Not currently supported.** Current behavior: with no filter parameter, it returns **all** applications in reverse chronological order.
I'll add this in batch 1 (see "Batch 1 Changes" below).

### Q4. Semantics of `GET /v1/mcenter/profile-views` — "viewed me" or "viewed by me"?

**"Who viewed me".** Requires `?kind=`:
- `kind=2` — employer view: "who visited my company homepage"
- `kind=3` — job seeker view: "who viewed my resume"

The corresponding `GET /v1/mcenter/my-views?kind=1|2|3` is "jobs / companies / resumes I have visited".

### Q5. Filter on `GET /v1/mcenter/my-views` — Does it support `?kind=job|company|resume`?

**Supported, but using integers rather than strings**: `?kind=1` (job) / `?kind=2` (company) / `?kind=3` (resume). **`kind` is required.**

### Q6. Which fields can `PUT /v1/mcenter/profile` modify — only email?

**Yes, currently only email is supported.** This will be expanded to a set of basic fields in batch 1 (see changes below).

> Note: The resume main table fields `name / nametype / sex / birthday / marriage / education / telphone / email / photo`
> go through `PUT /v1/mcenter/resume`. Do not mix these into `/profile`.

### Q7. Are `/v1/mcenter/chat/*` and PHPYun's job inquiry (zxmsg) the same system?

**Currently no.** `chat_service::send` hardcodes `ref_kind = REF_NONE`, so it can only send plain private messages. PHPYun's `zxmsg` carries `ref_kind=REF_JOB + ref_id=job_id` pointing to a specific job.

**To be implemented**: `POST /v1/mcenter/chat/send` will accept optional `ref_kind` + `ref_id` and forward them to the service for persistence. This is in scope for batch 2.

**Route list**:
```
POST /v1/mcenter/chat/send
GET  /v1/mcenter/chat/conversations
GET  /v1/mcenter/chat/with/{peer}
PUT  /v1/mcenter/chat/with/{peer}/read
GET  /v1/mcenter/chat/unread-count
```

### Q8. Field names for `POST /v1/mcenter/favorites`?

**Your `{kind, target_id}` is exactly right**:
```json
{ "kind": 1, "target_id": 12345 }
```
- `kind`: 1 = job / 2 = company / 3 = resume
- Delete: `DELETE /v1/mcenter/favorites/{kind}/{target_id}`
- Existence check: `GET /v1/mcenter/favorites/exists/{kind}/{target_id}`

### Q9. Body of `PUT /v1/mcenter/jobs/{id}/status`?

**Current body**:
```json
{ "status": 0 }   // 0 = open, 2 = closed
```
**Note that this is a numeric enum, not the strings `'open'/'closed'`.** It corresponds to the `phpyun_com_job.status` column in PHPYun.

Batch close goes through `POST /v1/mcenter/jobs/batch/close` (body `{ids: [1,2,3]}`).

### Q10. Pagination fields `page + page_size` (vs. PHPYun's `page + limit`)

**Standardized on `page` + `page_size`**:
- `page` starts at 1, defaults to 1
- `page_size` defaults to 20, clamped to 1..=200
- No `per_page` / `size` aliases

The response format is uniform:
```json
{
  "data": {
    "items": [...],
    "total": 1234,
    "page": 1,
    "page_size": 20
  }
}
```

---

## Batch 1 Changes (Implemented, see next section) [done]

1. `PUT /v1/mcenter/account/username` — One-time username change
2. `POST /v1/wap/register/check` — Pre-registration check (whether username/mobile/email is already taken)
3. `GET /v1/wap/register/config` — Returns configuration such as password rules and username rules
4. `DELETE /v1/mcenter/blacklist` — Clear my blacklist
5. `DELETE /v1/mcenter/vip/orders/{order_no}` — Cancel an unpaid order
6. `POST /v1/mcenter/resume/refresh` — Refresh my resume (bump `lastupdate`)
7. `GET /v1/mcenter/jobs/counts` — Badge counts for hiring / pending review / closed
8. `GET /v1/mcenter/integral/consumes` — Integral (points) consumption details
9. `PUT /v1/mcenter/profile` — Expand modifiable fields to basic fields such as `{email?, telphone?, realname?}`
10. `GET /v1/mcenter/my-applications?state=` + `GET /v1/mcenter/applications?state=` — Application status filtering
