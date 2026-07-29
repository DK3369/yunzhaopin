<?php

/**
 * Shared guards for legacy payment entry points.
 *
 * Keep protocol fields and URLs unchanged while enforcing typed identifiers,
 * prepared reads, ownership checks, amount equality and callback idempotency.
 */

function yun_payment_request_id()
{
    static $requestId;

    if ($requestId) {
        return $requestId;
    }

    $incoming = isset($_SERVER['HTTP_X_REQUEST_ID']) ? (string) $_SERVER['HTTP_X_REQUEST_ID'] : '';
    if (preg_match('/^[A-Za-z0-9._-]{8,64}$/', $incoming)) {
        $requestId = $incoming;
    } else {
        try {
            $requestId = bin2hex(random_bytes(12));
        } catch (Throwable $e) {
            $requestId = sha1(uniqid('', true));
        }
    }

    return $requestId;
}

function yun_payment_log($event, array $context = array())
{
    $safe = array(
        'request_id' => yun_payment_request_id(),
        'event'      => preg_replace('/[^a-z0-9_.-]/i', '_', (string) $event),
        'ip'         => isset($_SERVER['REMOTE_ADDR']) ? (string) $_SERVER['REMOTE_ADDR'] : ''
    );

    foreach ($context as $key => $value) {
        if (is_scalar($value) || $value === null) {
            $safe[preg_replace('/[^a-z0-9_.-]/i', '_', (string) $key)] = (string) $value;
        }
    }

    error_log('[payment-security] ' . json_encode($safe, JSON_UNESCAPED_SLASHES));
}

function yun_payment_order_id($value)
{
    $value = is_scalar($value) ? (string) $value : '';

    return preg_match('/^[0-9]{1,32}$/D', $value) ? $value : false;
}

function yun_payment_uid($value)
{
    $value = is_scalar($value) ? (string) $value : '';

    if (!preg_match('/^[1-9][0-9]{0,10}$/D', $value)) {
        return false;
    }

    return (int) $value;
}

function yun_payment_table($def, $table)
{
    $def = (string) $def;
    if (!preg_match('/^[A-Za-z0-9_]*$/D', $def)) {
        throw new RuntimeException('Invalid database prefix');
    }

    return '`' . $def . $table . '`';
}

function yun_payment_fetch_member($db, $def, $uid)
{
    $uid = yun_payment_uid($uid);
    if ($uid === false) {
        return array();
    }

    $table = yun_payment_table($def, 'member');
    $row = $db->prepared_select_one(
        "SELECT * FROM {$table} WHERE `uid` = ? LIMIT 1",
        'i',
        array($uid)
    );

    return is_array($row) ? $row : array();
}

function yun_payment_fetch_order($db, $def, $orderId)
{
    $orderId = yun_payment_order_id($orderId);
    if ($orderId === false) {
        return array();
    }

    $table = yun_payment_table($def, 'company_order');
    $row = $db->prepared_select_one(
        "SELECT * FROM {$table} WHERE `order_id` = ? LIMIT 1",
        's',
        array($orderId)
    );

    return is_array($row) ? $row : array();
}

function yun_payment_hash_matches($known, $provided)
{
    $known = is_scalar($known) ? (string) $known : '';
    $provided = is_scalar($provided) ? (string) $provided : '';

    return $known !== '' && $provided !== '' && hash_equals($known, $provided);
}

function yun_payment_cookie_member($db, $def)
{
    $uid = yun_payment_uid(isset($_COOKIE['uid']) ? $_COOKIE['uid'] : null);
    $usertype = isset($_COOKIE['usertype']) ? (string) $_COOKIE['usertype'] : '';
    $shell = isset($_COOKIE['shell']) ? (string) $_COOKIE['shell'] : '';

    if ($uid === false || !preg_match('/^[1-5]$/D', $usertype)) {
        return array();
    }

    $member = yun_payment_fetch_member($db, $def, $uid);
    if (!$member || (string) ($member['usertype'] ?? '') !== $usertype) {
        return array();
    }

    $expected = md5(
        (string) ($member['username'] ?? '') .
        (string) ($member['password'] ?? '') .
        (string) ($member['salt'] ?? '')
    );

    return yun_payment_hash_matches($expected, $shell) ? $member : array();
}

function yun_payment_token_member($db, $def, $uid, $token)
{
    $member = yun_payment_fetch_member($db, $def, $uid);
    if (!$member) {
        return array();
    }

    $expected = md5(
        (string) ($member['username'] ?? '') .
        (string) ($member['password'] ?? '') .
        (string) ($member['salt'] ?? '') .
        (string) ($member['usertype'] ?? '')
    );

    return yun_payment_hash_matches($expected, $token) ? $member : array();
}

function yun_payment_is_pending_order(array $order)
{
    return !empty($order['id'])
        && (string) ($order['order_state'] ?? '') === '1'
        && is_numeric($order['order_price'] ?? null)
        && (float) $order['order_price'] > 0;
}

function yun_payment_order_owned_by(array $order, array $member)
{
    return !empty($member['uid'])
        && (string) ($order['uid'] ?? '') === (string) $member['uid'];
}

function yun_payment_fast_order_matches(array $order)
{
    $cookieFast = isset($_COOKIE['fast']) && is_scalar($_COOKIE['fast']) ? (string) $_COOKIE['fast'] : '';
    $orderFast = isset($order['fast']) && is_scalar($order['fast']) ? (string) $order['fast'] : '';

    return $cookieFast !== ''
        && strlen($cookieFast) <= 128
        && yun_payment_hash_matches($orderFast, $cookieFast);
}

function yun_payment_amount_in_cents($amount)
{
    if (!is_scalar($amount)) {
        return false;
    }

    $amount = trim((string) $amount);
    if (!preg_match('/^(?:0|[1-9][0-9]{0,9})(?:\.([0-9]{1,2}))?$/D', $amount, $match)) {
        return false;
    }

    $fraction = str_pad(isset($match[1]) ? $match[1] : '', 2, '0');
    $whole = strstr($amount, '.', true);
    if ($whole === false) {
        $whole = $amount;
    }

    return ((int) $whole * 100) + (int) $fraction;
}

function yun_payment_callback_amount_matches($orderAmount, $callbackAmount, $paytype)
{
    $expected = yun_payment_amount_in_cents($orderAmount);
    if ($expected === false || $expected <= 0 || !is_scalar($callbackAmount)) {
        return false;
    }

    if ((string) $paytype === 'tenpay') {
        $provided = trim((string) $callbackAmount);
        return preg_match('/^[0-9]{1,12}$/D', $provided) && (int) $provided === $expected;
    }

    $provided = yun_payment_amount_in_cents($callbackAmount);
    return $provided !== false && $provided === $expected;
}

function yun_payment_order_lock($orderId)
{
    $orderId = yun_payment_order_id($orderId);
    if ($orderId === false) {
        return false;
    }

    $path = sys_get_temp_dir() . '/phpyun-pay-' . hash('sha256', $orderId) . '.lock';
    $handle = @fopen($path, 'c');
    if (!$handle || !@flock($handle, LOCK_EX)) {
        if (is_resource($handle)) {
            fclose($handle);
        }
        return false;
    }

    return $handle;
}

function yun_payment_order_unlock($handle)
{
    if (is_resource($handle)) {
        @flock($handle, LOCK_UN);
        fclose($handle);
    }
}

function yun_payment_unserialize_array($value)
{
    if (!is_string($value) || $value === '' || strlen($value) > 1048576) {
        return array();
    }

    $decoded = @unserialize($value, array('allowed_classes' => false));
    return is_array($decoded) ? $decoded : array();
}
