//! Points mall (physical redemption / gift work order) -- aligned with PHPYun `redeem`.
//!
//! Difference from `integral_service::exchange` (digital redemption, takes effect immediately):
//! - This goes through "submit work order -> admin review -> shipping" workflow
//! - On redeem, stock is locked + points deducted immediately; refunded if admin rejects
//! - Per-user limit is based on the cumulative `num` of "pending + approved" work orders
//!   the current user has on that reward
//!
//! Atomicity: if any of stock / points / work order writes fails, do reverse compensation.
//! Because writes cross tables and services, we don't use a single transaction; we follow
//! the compensation pattern from integral_service.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::error::InfraError;
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::integral::repo as integral_repo;
use phpyun_models::redeem::{
    entity::{RedeemClass, RedeemOrder, Reward},
    repo as redeem_repo,
};

// ---------- Categories ----------

/// 60s TTL cache: points-mall categories rarely change, and every mall page hits this.
/// Cache key is `Option<u64>` (None = all / Some(pid) = subcategory).
static CLASSES_CACHE: std::sync::OnceLock<
    moka::future::Cache<Option<u64>, std::sync::Arc<Vec<RedeemClass>>>,
> = std::sync::OnceLock::new();

fn classes_cache(
) -> &'static moka::future::Cache<Option<u64>, std::sync::Arc<Vec<RedeemClass>>> {
    CLASSES_CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(64)
            .time_to_live(std::time::Duration::from_secs(60))
            .build()
    })
}

/// Invalidate-on-write (called after admin creates/deletes a category) -- expire the entire
/// category tree in sync to avoid mismatch between subtree and parent.
async fn invalidate_classes_cache() {
    if let Some(c) = CLASSES_CACHE.get() {
        c.invalidate_all();
    }
}

pub async fn list_classes(
    state: &AppState,
    parent_id: Option<u64>,
) -> AppResult<std::sync::Arc<Vec<RedeemClass>>> {
    let cache = classes_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with(parent_id, async move {
            let list = redeem_repo::list_classes(&db, parent_id).await?;
            Ok::<_, AppError>(std::sync::Arc::new(list))
        })
        .await
        .map_err(AppError::from_arc)
}

pub async fn create_class(
    state: &AppState,
    actor: &AuthenticatedUser,
    parent_id: u64,
    name: &str,
    sort: i32,
) -> AppResult<u64> {
    let id = redeem_repo::insert_class(state.db.pool(), parent_id, name, sort, clock::now_ts()).await?;
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.redeem_class.create", Actor::uid(actor.uid))
            .target(format!("class:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn delete_class(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = redeem_repo::delete_class(state.db.pool(), id).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam("class_not_found".into())));
    }
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.redeem_class.delete", Actor::uid(actor.uid))
            .target(format!("class:{id}")),
    )
    .await;
    Ok(())
}

// ---------- Reward catalog ----------

pub struct RewardFilter {
    pub only_active: bool,
    pub nid: Option<u64>,
    pub tnid: Option<u64>,
}

pub async fn list_rewards(
    state: &AppState,
    f: &RewardFilter,
    page: Pagination,
) -> AppResult<Paged<Reward>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        redeem_repo::list_rewards(db, f.only_active, f.nid, f.tnid, page.offset, page.limit),
        redeem_repo::count_rewards(db, f.only_active, f.nid, f.tnid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get_reward(state: &AppState, id: u64) -> AppResult<Reward> {
    redeem_repo::get_reward(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("reward_not_found".into())))
}

pub struct NewRewardForm<'a> {
    pub name: &'a str,
    pub pic: &'a str,
    pub content: &'a str,
    pub integral: u32,
    pub stock: u32,
    pub restriction: u32,
    pub nid: u64,
    pub tnid: u64,
}

pub async fn create_reward(
    state: &AppState,
    admin: &AuthenticatedUser,
    f: &NewRewardForm<'_>,
) -> AppResult<u64> {
    let id = redeem_repo::insert_reward(
        state.db.pool(),
        &redeem_repo::NewReward {
            name: f.name,
            pic: f.pic,
            content: f.content,
            integral: f.integral,
            stock: f.stock,
            restriction: f.restriction,
            nid: f.nid,
            tnid: f.tnid,
        },
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.reward.create", Actor::uid(admin.uid))
            .target(format!("reward:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn set_reward_status(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    status: i32,
) -> AppResult<()> {
    let n = redeem_repo::set_reward_status(state.db.pool(), id, status).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam("reward_not_found".into())));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.reward.set_status", Actor::uid(admin.uid))
            .target(format!("reward:{id}"))
            .meta(&serde_json::json!({ "status": status })),
    )
    .await;
    Ok(())
}

pub async fn set_reward_flags(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    is_rec: Option<i32>,
    is_hot: Option<i32>,
) -> AppResult<()> {
    let n = redeem_repo::set_reward_flags(state.db.pool(), id, is_rec, is_hot).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam("reward_not_found".into())));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.reward.set_flags", Actor::uid(admin.uid))
            .target(format!("reward:{id}"))
            .meta(&serde_json::json!({ "is_rec": is_rec, "is_hot": is_hot })),
    )
    .await;
    Ok(())
}

pub async fn delete_reward(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let n = redeem_repo::delete_reward(state.db.pool(), id).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam("reward_not_found".into())));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.reward.delete", Actor::uid(admin.uid))
            .target(format!("reward:{id}")),
    )
    .await;
    Ok(())
}

// ---------- User work orders (redeem flow) ----------

pub struct RedeemForm<'a> {
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub address: &'a str,
    pub num: u32,
}

pub async fn redeem(
    state: &AppState,
    user: &AuthenticatedUser,
    reward_id: u64,
    f: &RedeemForm<'_>,
) -> AppResult<u64> {
    if f.num == 0 {
        return Err(AppError::new(InfraError::InvalidParam("bad_num".into())));
    }
    let reward = get_reward(state, reward_id).await?;
    if reward.status != 1 {
        return Err(AppError::new(InfraError::InvalidParam("reward_unavailable".into())));
    }
    if reward.stock < f.num {
        return Err(AppError::new(InfraError::InvalidParam("out_of_stock".into())));
    }
    if reward.restriction > 0 {
        let already =
            redeem_repo::count_user_orders_for_reward(state.db.reader(), user.uid, reward_id)
                .await?;
        if already + f.num > reward.restriction {
            return Err(AppError::new(InfraError::InvalidParam("over_per_user_limit".into())));
        }
    }
    let total_cost = reward.integral.saturating_mul(f.num);
    if total_cost == 0 {
        return Err(AppError::new(InfraError::InvalidParam("bad_cost".into())));
    }

    let pool = state.db.pool();
    let now = clock::now_ts();

    // 1) Deduct points (most stable, do first; on failure return directly, no compensation needed)
    let deducted = integral_repo::try_deduct(pool, user.uid, total_cost, now).await?;
    if deducted == 0 {
        return Err(AppError::new(InfraError::InvalidParam("insufficient_balance".into())));
    }

    // 2) Lock stock (CAS: stock>=num AND status=1)
    let mut tx = pool.begin().await?;
    let stock_affected = redeem_repo::tx_reserve_stock(&mut tx, reward_id, f.num).await?;
    if stock_affected == 0 {
        let _ = tx.rollback().await;
        // Refund points
        let _ = integral_repo::add_balance(pool, user.uid, total_cost as i32, now).await;
        return Err(AppError::new(InfraError::InvalidParam("out_of_stock".into())));
    }

    // 3) Write work order
    let order_id = match redeem_repo::tx_insert_order(
        &mut tx,
        &redeem_repo::NewOrder {
            uid: user.uid,
            gid: reward_id,
            name: &reward.name,
            linkman: f.linkman,
            linktel: f.linktel,
            address: f.address,
            integral: total_cost,
            num: f.num,
        },
        now,
    )
    .await
    {
        Ok(id) => id,
        Err(e) => {
            let _ = tx.rollback().await;
            // Refund stock + points together
            let _ = integral_repo::add_balance(pool, user.uid, total_cost as i32, now).await;
            // Stock rollback would need a separate UPDATE (the tx is already rolled back,
            // and the deduction is already refunded above; skip here)
            return Err(e.into());
        }
    };
    tx.commit().await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("redeem.create", Actor::uid(user.uid))
            .target(format!("reward:{reward_id}"))
            .meta(&serde_json::json!({ "num": f.num, "cost": total_cost, "order_id": order_id })),
    )
    .await;
    Ok(order_id)
}

pub async fn list_my_orders(
    state: &AppState,
    user: &AuthenticatedUser,
    status: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<RedeemOrder>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        redeem_repo::list_orders(db, Some(user.uid), status, page.offset, page.limit),
        redeem_repo::count_orders(db, Some(user.uid), status),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

/// User cancellation (pending-review only)
pub async fn cancel_my_order(
    state: &AppState,
    user: &AuthenticatedUser,
    order_id: u64,
) -> AppResult<()> {
    let order = redeem_repo::get_order(state.db.reader(), order_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("order_not_found".into())))?;
    if order.uid != user.uid {
        return Err(AppError::new(InfraError::InvalidParam("not_owner".into())));
    }
    refund_order(state, &order, /*expected_status=*/ 0, "redeem.user_cancel").await
}

// ---------- Admin approval ----------

pub async fn list_orders_admin(
    state: &AppState,
    status: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<RedeemOrder>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        redeem_repo::list_orders(db, None, status, page.offset, page.limit),
        redeem_repo::count_orders(db, None, status),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn approve_order(
    state: &AppState,
    admin: &AuthenticatedUser,
    order_id: u64,
) -> AppResult<()> {
    let pool = state.db.pool();
    let mut tx = pool.begin().await?;
    let n = redeem_repo::tx_set_order_status(&mut tx, order_id, /*expected=*/ 0, /*new=*/ 1).await?;
    if n == 0 {
        let _ = tx.rollback().await;
        return Err(AppError::new(InfraError::InvalidParam("order_not_pending".into())));
    }
    tx.commit().await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.redeem.approve", Actor::uid(admin.uid))
            .target(format!("order:{order_id}")),
    )
    .await;
    Ok(())
}

pub async fn reject_order(
    state: &AppState,
    admin: &AuthenticatedUser,
    order_id: u64,
) -> AppResult<()> {
    let order = redeem_repo::get_order(state.db.reader(), order_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("order_not_found".into())))?;
    refund_order(state, &order, /*expected_status=*/ 0, "admin.redeem.reject").await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.redeem.reject", Actor::uid(admin.uid))
            .target(format!("order:{order_id}")),
    )
    .await;
    Ok(())
}

/// Refund: CAS status 0 -> 2, return stock and points.
/// Reused for user cancel / admin reject.
async fn refund_order(
    state: &AppState,
    order: &RedeemOrder,
    expected_status: i32,
    reason_tag: &'static str,
) -> AppResult<()> {
    let pool = state.db.pool();
    let now = clock::now_ts();

    let mut tx = pool.begin().await?;
    let n = redeem_repo::tx_set_order_status(&mut tx, order.id, expected_status, /*new=*/ 2).await?;
    if n == 0 {
        let _ = tx.rollback().await;
        return Err(AppError::new(InfraError::InvalidParam("order_not_pending".into())));
    }
    let _ = redeem_repo::tx_return_stock(&mut tx, order.gid, order.num).await?;
    tx.commit().await?;

    // Refund points (ON DUPLICATE KEY UPDATE add-value on the pool)
    integral_repo::add_balance(pool, order.uid, order.integral as i32, now).await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(reason_tag, Actor::uid(order.uid))
            .target(format!("order:{}", order.id))
            .meta(&serde_json::json!({ "refund": order.integral, "stock": order.num })),
    )
    .await;
    Ok(())
}
