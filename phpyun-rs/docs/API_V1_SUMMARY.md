# API 规范（v1）

**接口总数**: 305 个 · **请求体类型 (schema)**: 342 个

## 调用约定

| 项 | 约定 |
|---|---|
| 请求方法 | 全部 `POST`（包括读操作） |
| 参数位置 | JSON body，`Content-Type: application/json` |
| 认证 | `Authorization: Bearer <token>`（登录态接口） |
| 响应包络 | `{ "code": 200, "msg": "ok", "data": ... }` |
| 校验失败 | HTTP 400 + `param_invalid: validation.<rule>` |

## 按 Tag 分类（OpenAPI 一级分类）

| Tag | 接口数 | 说明 |
|---|---:|---|
| `mcenter` | 151 | 会员中心（必须登录），求职者+企业 HR 共用 |
| `wap` | 92 | 前台公开接口（求职者/访客可调用，免登录或可选登录） |
| `admin` | 37 | 后台管理接口（仅管理员） |
| `auth` | 20 | 登录/注册/忘记密码/第三方授权/Token 刷新 |
| `upload` | 5 | 文件上传（图片/简历附件/营业执照等） |
| **合计** | **305** | |

## 按 URL 前缀分组（细分模块）

| 模块 | 接口数 |
|---|---:|
| `/v1/admin/ads` | 1 |
| `/v1/admin/app-versions` | 2 |
| `/v1/admin/audit-log` | 1 |
| `/v1/admin/broadcasts` | 2 |
| `/v1/admin/categories` | 1 |
| `/v1/admin/company-certs` | 2 |
| `/v1/admin/dashboard` | 2 |
| `/v1/admin/desc-classes` | 1 |
| `/v1/admin/descriptions` | 1 |
| `/v1/admin/feedback` | 3 |
| `/v1/admin/jobs` | 3 |
| `/v1/admin/nav` | 1 |
| `/v1/admin/orders` | 2 |
| `/v1/admin/recycle-bin` | 1 |
| `/v1/admin/redeem-classes` | 1 |
| `/v1/admin/redeem-orders` | 3 |
| `/v1/admin/reports` | 3 |
| `/v1/admin/rewards` | 3 |
| `/v1/admin/site-settings` | 1 |
| `/v1/admin/users` | 2 |
| `/v1/admin/warnings` | 1 |
| `/v1/mcenter/activity` | 1 |
| `/v1/mcenter/answers` | 2 |
| `/v1/mcenter/applications` | 3 |
| `/v1/mcenter/apply` | 1 |
| `/v1/mcenter/blacklist` | 2 |
| `/v1/mcenter/broadcasts` | 3 |
| `/v1/mcenter/cert` | 3 |
| `/v1/mcenter/chat` | 5 |
| `/v1/mcenter/comments` | 1 |
| `/v1/mcenter/company` | 13 |
| `/v1/mcenter/dashboard` | 2 |
| `/v1/mcenter/entrust` | 2 |
| `/v1/mcenter/entrusts` | 1 |
| `/v1/mcenter/eval-logs` | 2 |
| `/v1/mcenter/eval-papers` | 2 |
| `/v1/mcenter/fans` | 1 |
| `/v1/mcenter/favorites` | 3 |
| `/v1/mcenter/feedback` | 1 |
| `/v1/mcenter/followers` | 1 |
| `/v1/mcenter/follows` | 3 |
| `/v1/mcenter/integral` | 5 |
| `/v1/mcenter/interview-templates` | 2 |
| `/v1/mcenter/interviews` | 3 |
| `/v1/mcenter/invite-reg` | 1 |
| `/v1/mcenter/job-messages` | 3 |
| `/v1/mcenter/jobs` | 8 |
| `/v1/mcenter/messages` | 5 |
| `/v1/mcenter/my` | 3 |
| `/v1/mcenter/my-applications` | 2 |
| `/v1/mcenter/my-views` | 1 |
| `/v1/mcenter/oauth-bindings` | 2 |
| `/v1/mcenter/once-jobs` | 2 |
| `/v1/mcenter/password` | 1 |
| `/v1/mcenter/profile` | 1 |
| `/v1/mcenter/profile-views` | 1 |
| `/v1/mcenter/questions` | 6 |
| `/v1/mcenter/ratings` | 3 |
| `/v1/mcenter/recommend` | 4 |
| `/v1/mcenter/redeem` | 3 |
| `/v1/mcenter/referrals` | 2 |
| `/v1/mcenter/remarks` | 3 |
| `/v1/mcenter/reports` | 1 |
| `/v1/mcenter/resume` | 16 |
| `/v1/mcenter/resume-downloads` | 3 |
| `/v1/mcenter/resume-share-tokens` | 2 |
| `/v1/mcenter/saved-searches` | 2 |
| `/v1/mcenter/search-history` | 1 |
| `/v1/mcenter/sign` | 2 |
| `/v1/mcenter/vip` | 5 |
| `/v1/mcenter/warnings` | 3 |
| `/v1/mcenter/zph` | 3 |
| `/v1/wap/ads` | 2 |
| `/v1/wap/advice` | 1 |
| `/v1/wap/announcements` | 1 |
| `/v1/wap/answers` | 1 |
| `/v1/wap/app-version` | 1 |
| `/v1/wap/articles` | 2 |
| `/v1/wap/captcha` | 1 |
| `/v1/wap/categories` | 3 |
| `/v1/wap/cert` | 1 |
| `/v1/wap/claim` | 1 |
| `/v1/wap/companies` | 9 |
| `/v1/wap/descriptions` | 3 |
| `/v1/wap/dict` | 8 |
| `/v1/wap/eval-papers` | 3 |
| `/v1/wap/forgetpw` | 5 |
| `/v1/wap/friend-links` | 1 |
| `/v1/wap/gongzhao` | 1 |
| `/v1/wap/home` | 1 |
| `/v1/wap/hot-searches` | 1 |
| `/v1/wap/hr-docs` | 2 |
| `/v1/wap/integral` | 1 |
| `/v1/wap/jobs` | 10 |
| `/v1/wap/legal` | 1 |
| `/v1/wap/login` | 2 |
| `/v1/wap/logout` | 1 |
| `/v1/wap/map` | 2 |
| `/v1/wap/me` | 1 |
| `/v1/wap/nav` | 1 |
| `/v1/wap/oauth` | 6 |
| `/v1/wap/once-jobs` | 1 |
| `/v1/wap/parts` | 1 |
| `/v1/wap/pay` | 1 |
| `/v1/wap/qna` | 3 |
| `/v1/wap/questions` | 2 |
| `/v1/wap/ratings` | 2 |
| `/v1/wap/redeem` | 2 |
| `/v1/wap/refresh` | 1 |
| `/v1/wap/regions` | 1 |
| `/v1/wap/register` | 1 |
| `/v1/wap/resume-share` | 1 |
| `/v1/wap/resumes` | 4 |
| `/v1/wap/search` | 1 |
| `/v1/wap/share` | 3 |
| `/v1/wap/site` | 5 |
| `/v1/wap/sms` | 1 |
| `/v1/wap/specials` | 4 |
| `/v1/wap/stats` | 1 |
| `/v1/wap/upload` | 5 |
| `/v1/wap/usertype` | 1 |
| `/v1/wap/zph` | 3 |

## Tag: `mcenter` (151 个)

| 方法 | 路径 | 说明 |
|---|---|---|
| POST | `/v1/mcenter/activity` | My activity log |
| POST | `/v1/mcenter/answers/comments` | Comment on an answer (aligned with PHP `wap/ask::forcomment_… |
| POST | `/v1/mcenter/answers/support` | Upvote an answer |
| POST | `/v1/mcenter/applications` | Employer views all received applications |
| POST | `/v1/mcenter/applications/browse` | Mark as read (idempotent) |
| POST | `/v1/mcenter/applications/invite` | Invite to interview |
| POST | `/v1/mcenter/apply` | Job seeker submits a resume application to a job |
| POST | `/v1/mcenter/blacklist` | My blacklist |
| POST | `/v1/mcenter/blacklist/remove` | Unblock |
| POST | `/v1/mcenter/broadcasts` | Broadcasts visible to me |
| POST | `/v1/mcenter/broadcasts/read` | Mark as read |
| POST | `/v1/mcenter/broadcasts/unread-count` | Unread broadcast count |
| POST | `/v1/mcenter/cert/email/send` | Send email verification link (delivered via event bus; SMTP … |
| POST | `/v1/mcenter/cert/mobile/send` | Send verification code to a new mobile number |
| POST | `/v1/mcenter/cert/mobile/verify` | Verify SMS code and change mobile number |
| POST | `/v1/mcenter/chat/conversations` | My conversation list (one latest message per conversation) |
| POST | `/v1/mcenter/chat/send` | Send a private message |
| POST | `/v1/mcenter/chat/unread-count` | Total count of my unread private messages (for the frontend … |
| POST | `/v1/mcenter/chat/with` | Fetch the most recent N messages of a conversation (ordered … |
| POST | `/v1/mcenter/chat/with/read` | Mark all messages from the peer in a conversation as read |
| POST | `/v1/mcenter/comments/delete` | Delete my own comment |
| POST | `/v1/mcenter/company` | Get my company profile |
| POST | `/v1/mcenter/company/cert` | My certification status |
| POST | `/v1/mcenter/company/hrs` | Main company: list HRs |
| POST | `/v1/mcenter/company/hrs/remove` | Main company: remove HR |
| POST | `/v1/mcenter/company/interviews` | Employer views interview invitations they have sent |
| POST | `/v1/mcenter/company/interviews/cancel` | Employer cancels an interview |
| POST | `/v1/mcenter/company/interviews/create` | Employer creates an interview invitation (based on an apply … |
| POST | `/v1/mcenter/company/invite-codes` | Main company: generate invite code |
| POST | `/v1/mcenter/company/invite-codes/revoke` | Main company: revoke invite code |
| POST | `/v1/mcenter/company/join` | HR: join a company with an invite code |
| POST | `/v1/mcenter/company/my-companies` | HR: companies I have joined |
| POST | `/v1/mcenter/company/news` | My news list |
| POST | `/v1/mcenter/company/products` | My product list |
| POST | `/v1/mcenter/dashboard` | Member center — first-screen aggregate counts |
| POST | `/v1/mcenter/dashboard/year-report` | HR-side yearly report data — counterpart of PHP `wap/ajax::l… |
| POST | `/v1/mcenter/entrust` | List my headhunter bindings (paginated, newest first) |
| POST | `/v1/mcenter/entrust/delete` | Unbind a headhunter by `lt_uid` or by row `id` |
| POST | `/v1/mcenter/entrusts` | List jobseekers who have entrusted me (call as headhunter, u… |
| POST | `/v1/mcenter/eval-logs` | My assessment history |
| POST | `/v1/mcenter/eval-logs/detail` |  |
| POST | `/v1/mcenter/eval-papers/messages` | Leave a public message on an assessment paper. Counterpart o… |
| POST | `/v1/mcenter/eval-papers/submit` | Submit assessment answers |
| POST | `/v1/mcenter/fans` | Paginated list of jobseekers who have favorited my company's… |
| POST | `/v1/mcenter/favorites` | Toggle favorite — favorited ↔ unfavorited. The response's `d… |
| POST | `/v1/mcenter/favorites/exists` | Whether the current user has already favorited a given targe… |
| POST | `/v1/mcenter/favorites/remove` | Remove favorite |
| POST | `/v1/mcenter/feedback` | Submit feedback (anonymous allowed) |
| POST | `/v1/mcenter/followers` | Followers of the current user (employers see who follows the… |
| POST | `/v1/mcenter/follows` | Toggle follow — followed ↔ unfollowed. |
| POST | `/v1/mcenter/follows/exists` | Cheap probe used by frontend to render the follow-button sta… |
| POST | `/v1/mcenter/follows/list` | Targets I am following (filtered by kind). |
| POST | `/v1/mcenter/integral/balance` | My points balance |
| POST | `/v1/mcenter/integral/exchange` | Exchange item |
| POST | `/v1/mcenter/integral/history` | Exchange history |
| POST | `/v1/mcenter/integral/transfer` | Points transfer |
| POST | `/v1/mcenter/integral/transfers` | My transfer records (received + sent) |
| POST | `/v1/mcenter/interview-templates` | Interview template list |
| POST | `/v1/mcenter/interview-templates/update` | Update or soft-delete an interview template (body with `"sta… |
| POST | `/v1/mcenter/interviews` | Job seeker views received interview invitations |
| POST | `/v1/mcenter/interviews/accept` | Accept interview |
| POST | `/v1/mcenter/interviews/reject` | Reject interview |
| POST | `/v1/mcenter/invite-reg` | Send invitation registration email |
| POST | `/v1/mcenter/job-messages` | Employer lists every message left on any of their jobs. |
| POST | `/v1/mcenter/job-messages/hide` | Employer hides a message they own. |
| POST | `/v1/mcenter/job-messages/reply` | Employer answers a single message. |
| POST | `/v1/mcenter/jobs` | Employer views their own list of published jobs |
| POST | `/v1/mcenter/jobs/batch/close` | Batch close |
| POST | `/v1/mcenter/jobs/batch/delete` | Batch delete |
| POST | `/v1/mcenter/jobs/batch/refresh` | Batch refresh |
| POST | `/v1/mcenter/jobs/detail` | Employer views the details of one of their own jobs |
| POST | `/v1/mcenter/jobs/refresh` | Refresh job (bumps `lastupdate` so it sorts to the top of th… |
| POST | `/v1/mcenter/jobs/status` | Open / close (online / offline) |
| POST | `/v1/mcenter/jobs/update` | Update job (re-enters review after editing) |
| POST | `/v1/mcenter/messages` | Message list |
| POST | `/v1/mcenter/messages/delete` | Delete message |
| POST | `/v1/mcenter/messages/read` | Mark as read |
| POST | `/v1/mcenter/messages/read-all` | Mark all as read |
| POST | `/v1/mcenter/messages/unread-summary` | Aggregate unread counts across every notification channel — … |
| POST | `/v1/mcenter/my-applications` | Job seeker views their own application list |
| POST | `/v1/mcenter/my-applications/withdraw` | Withdraw an application |
| POST | `/v1/mcenter/my-views` | Jobs / companies / resumes I have viewed |
| POST | `/v1/mcenter/my/answers` | My answers |
| POST | `/v1/mcenter/my/attended-questions` | Questions I follow |
| POST | `/v1/mcenter/my/questions` | Questions I asked |
| POST | `/v1/mcenter/oauth-bindings` | Third-party providers bound to the current user |
| POST | `/v1/mcenter/oauth-bindings/unbind` | Unbind the specified third-party provider |
| POST | `/v1/mcenter/once-jobs/orders` | My pending one-off-posting orders (`type=25`, `order_state=1… |
| POST | `/v1/mcenter/once-jobs/orders/cancel` | Cancel a pending one-off-posting order (sets `order_state = … |
| POST | `/v1/mcenter/password` | Change password (requires old password verification) |
| POST | `/v1/mcenter/profile` | Current user summary |
| POST | `/v1/mcenter/profile-views` | Who has viewed me |
| POST | `/v1/mcenter/questions` | Ask a question |
| POST | `/v1/mcenter/questions/answers` | Answer |
| POST | `/v1/mcenter/questions/answers/accept` | Accept an answer (only the questioner can) |
| POST | `/v1/mcenter/questions/attention` | Follow / unfollow |
| POST | `/v1/mcenter/questions/delete` | Delete my question |
| POST | `/v1/mcenter/questions/support` | Upvote a question |
| POST | `/v1/mcenter/ratings` | Rate / update rating |
| POST | `/v1/mcenter/ratings/get-mine` | Get my rating for a target |
| POST | `/v1/mcenter/ratings/unrate` | Withdraw rating |
| POST | `/v1/mcenter/recommend/email` | Recommend a job (`kind=job`) or resume (`kind=resume`) to a … |
| POST | `/v1/mcenter/recommend/email/quota` | Pre-flight check for email-recommendation quota. Counterpart… |
| POST | `/v1/mcenter/recommend/jobs` | Recommend jobs based on my expectations + resume education |
| POST | `/v1/mcenter/recommend/resumes` | Company: recommend resumes based on the edu of the first act… |
| POST | `/v1/mcenter/redeem/orders` | My redeem orders |
| POST | `/v1/mcenter/redeem/orders/cancel` | Cancel my pending order |
| POST | `/v1/mcenter/redeem/rewards/redeem` | Submit a redeem order |
| POST | `/v1/mcenter/referrals` | My referral list |
| POST | `/v1/mcenter/referrals/summary` | Summary: number of invitees + accumulated points |
| POST | `/v1/mcenter/remarks` | My remarks list |
| POST | `/v1/mcenter/remarks/delete` | Delete a remark |
| POST | `/v1/mcenter/remarks/get-one` | Get a specific remark |
| POST | `/v1/mcenter/reports` | Submit a report |
| POST | `/v1/mcenter/resume` | Get the current job seeker's resume |
| POST | `/v1/mcenter/resume-downloads` | Company downloads a resume |
| POST | `/v1/mcenter/resume-downloads/inbox` | Job seeker view: who has downloaded me |
| POST | `/v1/mcenter/resume-downloads/outbox` | Company view: resumes I have downloaded |
| POST | `/v1/mcenter/resume-share-tokens` | Create a share link |
| POST | `/v1/mcenter/resume-share-tokens/revoke` | Revoke share |
| POST | `/v1/mcenter/resume/completion` | My resume completeness |
| POST | `/v1/mcenter/resume/edus` | Education history list |
| POST | `/v1/mcenter/resume/edus/update` | Update an education history entry (or soft delete — body wit… |
| POST | `/v1/mcenter/resume/expects` | List job expectations |
| POST | `/v1/mcenter/resume/expects/update` | Update or soft-delete a job expectation (body with `"status"… |
| POST | `/v1/mcenter/resume/languages` |  |
| POST | `/v1/mcenter/resume/languages/update` |  |
| POST | `/v1/mcenter/resume/projects` |  |
| POST | `/v1/mcenter/resume/projects/update` |  |
| POST | `/v1/mcenter/resume/skills` |  |
| POST | `/v1/mcenter/resume/skills/update` |  |
| POST | `/v1/mcenter/resume/status` | Change resume visibility status |
| POST | `/v1/mcenter/resume/timeline` | My resume timeline (newest first) |
| POST | `/v1/mcenter/resume/works` |  |
| POST | `/v1/mcenter/resume/works/update` | Update or soft-delete a work experience entry (sending `"sta… |
| POST | `/v1/mcenter/saved-searches` | My saved searches |
| POST | `/v1/mcenter/saved-searches/notify` | Toggle notification switch |
| POST | `/v1/mcenter/search-history` | My search history |
| POST | `/v1/mcenter/sign` | Check in |
| POST | `/v1/mcenter/sign/status` | Check-in status |
| POST | `/v1/mcenter/vip/current` | My current VIP status |
| POST | `/v1/mcenter/vip/orders` | Create an order (returns order_no, hand it to the frontend t… |
| POST | `/v1/mcenter/vip/orders/mock-paid` | **Dev only**: simulates a payment callback (in production, s… |
| POST | `/v1/mcenter/vip/packages` | List of purchasable packages (filtered by current user's use… |
| POST | `/v1/mcenter/vip/quote` |  |
| POST | `/v1/mcenter/warnings` | Warnings I have received |
| POST | `/v1/mcenter/warnings/read` | Mark as read |
| POST | `/v1/mcenter/warnings/unread-count` | Unread warning count |
| POST | `/v1/mcenter/zph/com-status` | Pre-apply status for an employer on a job fair. |
| POST | `/v1/mcenter/zph/my-reservation` | My reservation for a specific job fair |
| POST | `/v1/mcenter/zph/reserve` | Reserve a job-fair slot |

## Tag: `wap` (92 个)

| 方法 | 路径 | 说明 |
|---|---|---|
| POST | `/v1/wap/ads` | List active ads for a slot |
| POST | `/v1/wap/ads/click` | Record an ad click. Counterpart of PHP `index/index::clickhi… |
| POST | `/v1/wap/advice` | Submit advice/feedback |
| POST | `/v1/wap/announcements` | Announcement list |
| POST | `/v1/wap/answers/comments` | List comments under an answer (aligned with PHP `wap/ask::ge… |
| POST | `/v1/wap/app-version` | Get the latest version for a platform |
| POST | `/v1/wap/articles` | Public article list |
| POST | `/v1/wap/articles/hits` | Bump and return the new hit count. Counterpart of PHP
`wap/a… |
| POST | `/v1/wap/categories` | Get all categories under a kind (flat list with parent_id; c… |
| POST | `/v1/wap/categories/children` | Get the direct children of a given parent node |
| POST | `/v1/wap/categories/recommended` | Recommended categories (hand-picked by admin via `rec=1` fla… |
| POST | `/v1/wap/cert/email/verify` | Confirm a pending email change. Body carries the one-time to… |
| POST | `/v1/wap/claim` | Claim a company |
| POST | `/v1/wap/companies` | Public company list (filter by keyword / region / industry) |
| POST | `/v1/wap/companies/autocomplete` | Lightweight company name autocomplete — counterpart of PHP
`… |
| POST | `/v1/wap/companies/detail` | Public company detail |
| POST | `/v1/wap/companies/hot` | Featured companies on the homepage. |
| POST | `/v1/wap/companies/jobs` | Public job list for a given company (paginated) |
| POST | `/v1/wap/companies/news` | Company news list |
| POST | `/v1/wap/companies/news/detail` | Company news detail |
| POST | `/v1/wap/companies/products` | Company product list |
| POST | `/v1/wap/companies/products/detail` | Company product detail |
| POST | `/v1/wap/descriptions` | Single-page list (visible only) |
| POST | `/v1/wap/descriptions/by-name` | Look up a description by its hand-typed `name` (PHPYun's `ph… |
| POST | `/v1/wap/descriptions/classes` | Class list |
| POST | `/v1/wap/dict/cities` | Province / centrally-administered municipality dictionary |
| POST | `/v1/wap/dict/cities/by-province` |  |
| POST | `/v1/wap/dict/educations` | Education levels |
| POST | `/v1/wap/dict/experiences` | Work experience |
| POST | `/v1/wap/dict/industries` | Industry categories |
| POST | `/v1/wap/dict/job-categories` | Top-level job categories |
| POST | `/v1/wap/dict/job-types` | Job types (full-time / part-time / internship / ...) |
| POST | `/v1/wap/dict/salaries` | Salary ranges |
| POST | `/v1/wap/eval-papers` | Assessment list |
| POST | `/v1/wap/eval-papers/messages` | Public list of leave-messages on an assessment paper. Counte… |
| POST | `/v1/wap/eval-papers/recent-examinees` | Recent examinees who have taken this paper, grouped by uid. … |
| POST | `/v1/wap/friend-links` | List friend links |
| POST | `/v1/wap/gongzhao` | Joint recruitment list |
| POST | `/v1/wap/home` | Home page |
| POST | `/v1/wap/hot-searches` | Top N hot search keywords |
| POST | `/v1/wap/hr-docs` | HR toolbox list |
| POST | `/v1/wap/hr-docs/download` | Track a download click — counterpart of PHP `hr/index::ajax_… |
| POST | `/v1/wap/integral/items` | List points-mall items |
| POST | `/v1/wap/jobs` | Public job list (paginated + searchable). Response `data` fi… |
| POST | `/v1/wap/jobs/contact` | Resolve the contact info for a single job. Counterpart of PH… |
| POST | `/v1/wap/jobs/detail` | Public job detail -- returned as a nested map **grouped by b… |
| POST | `/v1/wap/jobs/hits` | Bump and return the new job-hit count. Counterpart of PHP
`w… |
| POST | `/v1/wap/jobs/messages` | Public list of approved Q&A for a job. |
| POST | `/v1/wap/jobs/messages/hide` | Author / employer hides one of their messages. |
| POST | `/v1/wap/jobs/messages/post` | Jobseeker leaves a public message — requires login + image c… |
| POST | `/v1/wap/jobs/same-company` | Other jobs from the same company |
| POST | `/v1/wap/jobs/share-text` | Get pre-formatted share text — counterpart of PHP `wap/job::… |
| POST | `/v1/wap/jobs/similar` | Similar jobs (same job1 category) |
| POST | `/v1/wap/legal` | Stable-slug shortcut for PHP `wap/index::about/contact/priva… |
| POST | `/v1/wap/map/companies` | Nearby companies |
| POST | `/v1/wap/map/jobs` | Nearby jobs |
| POST | `/v1/wap/nav` | Get navigation for the specified position (header/footer/sid… |
| POST | `/v1/wap/once-jobs/pay` | Create a payment order for a one-off shop posting — counterp… |
| POST | `/v1/wap/parts` | Public part-time list |
| POST | `/v1/wap/pay/callback` | Gateway callback: authenticate via the shared secret in the … |
| POST | `/v1/wap/qna/categories` | Q&A category list (aligned with PHP `wap/ask::qclass_action`… |
| POST | `/v1/wap/qna/hotweek` | Hot questions of the week (aligned with PHP `wap/ask::hotwee… |
| POST | `/v1/wap/qna/top-answerers` | Top answerers in the last N days — counterpart of PHP
`ask::… |
| POST | `/v1/wap/questions` | Question list |
| POST | `/v1/wap/questions/answers` | Answer list |
| POST | `/v1/wap/ratings/list` | Rating list (newest first) |
| POST | `/v1/wap/ratings/summary` | Rating summary (count + avg) |
| POST | `/v1/wap/redeem/classes` | Redeem mall classes |
| POST | `/v1/wap/redeem/rewards` | Reward list (active only) |
| POST | `/v1/wap/regions/city-domain` | Resolve a `(lng, lat)` to the configured sub-site domain — c… |
| POST | `/v1/wap/resume-share/view` | View a resume via a one-time share token. Token goes in the … |
| POST | `/v1/wap/resumes` | Public resume list — **searchable by companies only** |
| POST | `/v1/wap/resumes/default-expect` | Resolve a jobseeker's default `phpyun_resume_expect.id` from… |
| POST | `/v1/wap/resumes/detail` | Public resume detail — companies must have downloaded/applie… |
| POST | `/v1/wap/resumes/expects/hits` | Bump the per-job-intent hit count on a resume. Counterpart o… |
| POST | `/v1/wap/search` | Global search |
| POST | `/v1/wap/share/companies` | Company share link |
| POST | `/v1/wap/share/jobs` | Job share link |
| POST | `/v1/wap/share/resumes` | Public resume share link (non-token version — login required… |
| POST | `/v1/wap/site/map-config` | Front-end map widget configuration. Counterpart of PHP
`ajax… |
| POST | `/v1/wap/site/pages` | Site page |
| POST | `/v1/wap/site/settings` | List public settings |
| POST | `/v1/wap/site/sub-sites` | List configured sub-sites. PHP equivalent: `wap/site::cache_… |
| POST | `/v1/wap/site/sub-sites/match` | Find the matching sub-site for a city triplet. PHP `wap/site… |
| POST | `/v1/wap/specials` | Special list |
| POST | `/v1/wap/specials/apply` | Company signs up to a special event — counterpart of PHP
`wa… |
| POST | `/v1/wap/specials/companies` | Participating companies (phpyun_special_company JOIN phpyun_… |
| POST | `/v1/wap/specials/jobs` | Jobs inside a special (reuses the rich JobSummary: 34 fields… |
| POST | `/v1/wap/stats/overview` | Site overview statistics |
| POST | `/v1/wap/zph` | Job-fair list |
| POST | `/v1/wap/zph/companies` | Participating-company list |
| POST | `/v1/wap/zph/jobs` | Jobs participating in a recruitment fair. Counterpart of PHP… |

## Tag: `admin` (37 个)

| 方法 | 路径 | 说明 |
|---|---|---|
| POST | `/v1/admin/ads` |  |
| POST | `/v1/admin/app-versions` |  |
| POST | `/v1/admin/app-versions/delete` |  |
| POST | `/v1/admin/audit-log` | List audit log entries |
| POST | `/v1/admin/broadcasts` |  |
| POST | `/v1/admin/broadcasts/delete` |  |
| POST | `/v1/admin/categories` |  |
| POST | `/v1/admin/company-certs` | Review queue |
| POST | `/v1/admin/company-certs/review` | Approve / reject |
| POST | `/v1/admin/dashboard/overview` | Review queue + activity snapshot |
| POST | `/v1/admin/dashboard/recent-signups` | Recent signups |
| POST | `/v1/admin/desc-classes` |  |
| POST | `/v1/admin/descriptions` |  |
| POST | `/v1/admin/feedback` | Feedback list |
| POST | `/v1/admin/feedback/batch/status` | Batch mark as resolved |
| POST | `/v1/admin/feedback/status` | Mark feedback as resolved |
| POST | `/v1/admin/jobs` | Job review queue |
| POST | `/v1/admin/jobs/batch/state` | Batch review jobs |
| POST | `/v1/admin/jobs/state` | Review a job |
| POST | `/v1/admin/nav` |  |
| POST | `/v1/admin/orders` | Global order list |
| POST | `/v1/admin/orders/status` | Refund / cancel order |
| POST | `/v1/admin/recycle-bin` | Recycle bin list |
| POST | `/v1/admin/redeem-classes` | List classes |
| POST | `/v1/admin/redeem-orders` | Order list |
| POST | `/v1/admin/redeem-orders/approve` | Approve order (no refund, awaiting shipment) |
| POST | `/v1/admin/redeem-orders/reject` | Reject order (refund integral + restore stock) |
| POST | `/v1/admin/reports` | Report queue |
| POST | `/v1/admin/reports/batch/status` | Batch process reports |
| POST | `/v1/admin/reports/status` | Process a report |
| POST | `/v1/admin/rewards` | Reward list |
| POST | `/v1/admin/rewards/flags` | Recommended / hot flags |
| POST | `/v1/admin/rewards/status` | Toggle online/offline |
| POST | `/v1/admin/site-settings` | All settings (including non-public) |
| POST | `/v1/admin/users` | User list (admin) |
| POST | `/v1/admin/users/status` | Freeze / unfreeze a user |
| POST | `/v1/admin/warnings` | Admin: list warnings |

## Tag: `auth` (20 个)

| 方法 | 路径 | 说明 |
|---|---|---|
| POST | `/v1/wap/captcha` | Issue an image CAPTCHA |
| POST | `/v1/wap/forgetpw/appeal` | Submit an account appeal — counterpart of PHP `forgetpw/inde… |
| POST | `/v1/wap/forgetpw/reset` | Reset password |
| POST | `/v1/wap/forgetpw/reset-by-email` | Reset the password using an emailed code. |
| POST | `/v1/wap/forgetpw/send-email` | Send a 6-digit password-reset code via email.
Counterpart of… |
| POST | `/v1/wap/forgetpw/send-sms` | Send forgot-password verification code (call `GET /v1/wap/ca… |
| POST | `/v1/wap/login` | Username/password login |
| POST | `/v1/wap/login/sms` | SMS dynamic-code login (aligned with the `act_login=1` branc… |
| POST | `/v1/wap/logout` | Log out (revoke the current access token) |
| POST | `/v1/wap/me` | Current logged-in user summary (uses L1 moka → L2 Redis → DB… |
| POST | `/v1/wap/oauth/bind` | Logged-in user binds a third-party account to the current ui… |
| POST | `/v1/wap/oauth/login` | Third-party login: exchange id_token for access+refresh toke… |
| POST | `/v1/wap/oauth/qq/authorize-url` | Generate the QQ Connect authorization redirect URL.
Counterp… |
| POST | `/v1/wap/oauth/qq/code-login` | Exchange a QQ Connect code for a JWT. |
| POST | `/v1/wap/oauth/weibo/authorize-url` | Generate the Weibo authorization redirect URL.
Counterpart o… |
| POST | `/v1/wap/oauth/weibo/code-login` | Exchange a Weibo code for a JWT. |
| POST | `/v1/wap/refresh` | Exchange refresh_token for new access+refresh (old refresh i… |
| POST | `/v1/wap/register` | Account registration (SMS + image captcha + auto login) |
| POST | `/v1/wap/sms/send` | Send SMS code (call `GET /v1/wap/captcha` first to obtain ci… |
| POST | `/v1/wap/usertype/select` | First-time role selection — counterpart of PHP `wap/login::s… |

## Tag: `upload` (5 个)

| 方法 | 路径 | 说明 |
|---|---|---|
| POST | `/v1/wap/upload/attachment` | Upload attachment (10MB, pdf/doc/docx; for resume attachment… |
| POST | `/v1/wap/upload/avatar` | Upload avatar (1MB image) |
| POST | `/v1/wap/upload/cert` | Upload verification image (5MB) -- company business license … |
| POST | `/v1/wap/upload/company-logo` | Upload company logo (2MB image, employer only) |
| POST | `/v1/wap/upload/resume-photo` | Upload resume photo (2MB image, jobseeker only) |

## 在线文档

- **Swagger UI**: <http://dev.test/yapi/docs/>
- **OpenAPI JSON**: <http://dev.test/yapi/api-docs/v1/openapi.json>
