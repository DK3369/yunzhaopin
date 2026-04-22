function myFunction(_this) {
    _this.value = _this.value.replace(/[^0-9]/g, '');
}

/* 选择不同充值积分操作 */
$("ul#integral_num").on("click", "li", function () {

    var integral = $(this).attr('data-integral'); // 充值积分

    if (parseInt(integral) >= parseInt(min_integral) || parseInt(integral) == 0) {

        $(this).addClass("discount_opt_for"); // 点击li添加class
        $(this).siblings('li').removeClass("discount_opt_for"); // 删除兄弟li的class属性
    } else {

        showToast('最低充值：'+ min_integral + jifen, 2);
        return false;
    }

    if (integral == '0') {

        $('.defined').css('display', 'flex');
        $('#price').html(0);

        $('#bank_price').val(0);
        $('#order_price').val(0);
        $('#integral_int').val(integral);
        $("#integralid").val('');
        $('#user_defined').val('');
    } else {

        var integralid  = $(this).attr('data-id'); // 积分类别id
        var discount    = $(this).attr('data-discount'); // 积分折扣

        if (parseInt(discount) > 0) {
            var price   = (integral / pro) * (discount / 100);
        } else {
            var price   = integral / pro;
        }
        price = Math.round(price * 100) / 100;
        $("#integralid").val(integralid);
        $('.defined').css('display', 'none');
        $('#user_defined').val('');

        $('#price').html(price);
        $('#bank_price').val(price);
        $('#order_price').val(price);

        $('#integral_int').val(integral);
    }
})

function checkIntegralNum() {
    var integral = $("#user_defined").val();
    if (parseInt(integral) > 0) {
        var integralval = integral;
        var discountarr = [];
        for (var i in integralclass) {
            if (integralval >= parseInt(integralclass[i]['integral'])) {
                discountarr.push({
                    'discount': integralclass[i]['discount'],
                    'id': integralclass[i]['id']
                });
            }
        }
        if (discountarr.length > 0) {
            var discount = discountarr[discountarr.length - 1].discount;
            $("#integralid").val(discountarr[discountarr.length - 1].id);
        }
        if (parseInt(discount) > 0) {
            var price = (integral / pro) * (discount / 100);
        } else {
            var price = integral / pro;
        }
        price = Math.round(price * 100) / 100;
        $('#price').html(price);
        $('#bank_price').val(price);
        $('#order_price').val(price);
        $('#integral_int').val(integral);
    } else {

        $('#price').html(0);
        $('#bank_price').val(0);
        $('#order_price').val(0);
        $('#integral_int').val(0);
    }
}

function autointegral() {
    var integral = parseInt($("#user_defined").val());
    if (integral) {
        if (min_integral > 0 && integral < min_integral) {
            integral = min_integral;
            $("#user_defined").val(integral);
            $('#integral_int').val(integral);
        } else {
            $("#user_defined").val(integral);
            $('#integral_int').val(integral);
        }
        var integralval = integral;
        var discountarr = [];
        for (var i in integralclass) {
            if (integralval >= parseInt(integralclass[i]['integral'])) {
                discountarr.push({
                    'discount': integralclass[i]['discount'],
                    'id': integralclass[i]['id']
                });
            }
        }
        if (discountarr.length > 0) {
            var discount = discountarr[discountarr.length - 1].discount;
            $("#integralid").val(discountarr[discountarr.length - 1].id);
        } else {
            var discount = 0;
            $("#integralid").val('');
        }
        if (parseInt(discount) > 0) {
            var price = (integral / pro) * (discount / 100);
        } else {
            var price = integral / pro;
        }
        price = Math.round(price * 100) / 100;
        $('#price').html(price);
        $('#bank_price').val(price);
        $('#order_price').val(price);
    }
}

function paycheck(type) {
    var type;
    if(type == "alipay") {

        $('.dredge_body_wx_icon img').attr('src', wap_style + '/images/dredge_To_confirm.png');
        $('.dredge_body_zfb .dredge_body_wx_icon img').attr('src', wap_style + '/images/dredge_affirm.png');

        $("#paytype").val('alipay');
        $(".paybank").css('display', 'none');

        $("#integral_form").attr("action", "index.php?c=dingdan");
    } else if(type == "bank") {

        $('.dredge_body_wx_icon img').attr('src', wap_style + '/images/dredge_To_confirm.png');
        $('.dredge_body_bank .dredge_body_wx_icon img').attr('src', wap_style + '/images/dredge_affirm.png');

        $("#paytype").val('bank');
        $(".paybank").css('display', 'block');

        $("#integral_form").attr("action", "index.php?c=paybank");
    }
}


function bankPicker(){

    bankVue.$data.bank = bankData;
    bankVue.$data.bankIndex = bankIndex;
    bankVue.$data.bankShow = true;
}
function bankTimePicker(){

    bankVue.$data.bankTimeShow = true;
}

function integral_form() {

    var field = getFormValue('integral_form');

    var integral = field.integralid;

    if (parseInt(min_integral) > parseInt(integral)) {

        showToast('最低充值：' + min_integral + pricename, 2)
        return false;
    }

    if (field.price == 0 || field.price_int == 0){

        showToast('请选择或者填写充值'+pricename);
        return false;
    }

    var paytype = field.paytype;

    if (paytype == "") {
        showToast('请选择一种支付方式！', 2);
        return false;
    }

    if (paytype == 'bank') {

        if (field.bank_name == '') {
            showToast('请填写汇款银行！', 2);
            return false;
        } else if (field.bank_number == '') {
            showToast('请填写汇入账号！', 2);
            return false;
        } else if (field.bank_price == '') {
            showToast('请填写汇款金额！', 2);
            return false;
        } else if (field.bank_time == '') {
            showToast('请填写汇款时间！', 2);
            return false;
        }
    }
}