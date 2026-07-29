<?php

require_once dirname(__DIR__) . '/app/include/payment_security.php';

$tests = array();

function payment_smoke($name, $ok)
{
    global $tests;
    $tests[] = array('name' => $name, 'ok' => (bool) $ok);
}

payment_smoke('numeric order accepted', yun_payment_order_id('202607290001') === '202607290001');
payment_smoke('SQL injection order rejected', yun_payment_order_id("1' OR '1'='1") === false);
payment_smoke('route-like order rejected', yun_payment_order_id('../../etc/passwd') === false);
payment_smoke('Alipay amount matches', yun_payment_callback_amount_matches('19.90', '19.9', 'alipay'));
payment_smoke('Alipay amount mismatch rejected', !yun_payment_callback_amount_matches('19.90', '19.91', 'alipay'));
payment_smoke('Tenpay cents match', yun_payment_callback_amount_matches('19.90', '1990', 'tenpay'));
payment_smoke('Tenpay yuan value rejected', !yun_payment_callback_amount_matches('19.90', '19.90', 'tenpay'));
payment_smoke('negative amount rejected', yun_payment_amount_in_cents('-1.00') === false);
payment_smoke('over-precision amount rejected', yun_payment_amount_in_cents('1.001') === false);

class PaymentSecuritySmokePayload
{
    public $value = 'unsafe';
}

$decoded = yun_payment_unserialize_array(serialize(new PaymentSecuritySmokePayload()));
payment_smoke(
    'serialized object is not instantiated',
    !$decoded || !($decoded instanceof PaymentSecuritySmokePayload)
);

$lock = yun_payment_order_lock('202607290001');
payment_smoke('order callback lock acquired', is_resource($lock));
yun_payment_order_unlock($lock);

$failed = 0;
foreach ($tests as $test) {
    echo ($test['ok'] ? 'PASS' : 'FAIL') . ' ' . $test['name'] . PHP_EOL;
    if (!$test['ok']) {
        $failed++;
    }
}

echo sprintf('%d/%d payment security checks passed', count($tests) - $failed, count($tests)) . PHP_EOL;
exit($failed ? 1 : 0);
