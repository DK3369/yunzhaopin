// 腾讯验证码回调函数
function tecentCallback(res) {
	// var url = web_url+'/index.php?m=tecentcode'
    if (res.ret === 0) {
        $("input[name='verify_token']").val(res.ticket);
        $("input[name='verify_str']").val(res.randstr);
        //提交操作
        var type = $('#bind-captcha').attr('data-type');
        var dataid = $('#bind-captcha').attr('data-id');
        //提交表单
        if(type=='submit'){
            $('#'+dataid).submit();
        }else{
            //模拟点击
            $("#"+dataid).trigger("click");
        }
        // $.post(url, {
        //     ticket: res.ticket,
        //     str: res.randstr
        // }, function(data) {
        //     if(data == 0) {// 二次验证未通过，刷新验证码
        //         $('#bind-captcha').click();
        //     } else {
        //         $("input[name='verify_token']").val(res.ticket);
        //         //提交操作
        //         var type = $('#bind-captcha').attr('data-type');
        //         var dataid = $('#bind-captcha').attr('data-id');
        //         // debugger
        //         //提交表单
        //         if(type=='submit'){
        //             $('#'+dataid).submit();
        //         }else{
        //             //模拟点击
        //             $("#"+dataid).trigger("click");
        //         }
        //     }
        // });
    } else {
        $("input[name='verify_token']").val();
        $("input[name='verify_str']").val();
    }
}

$(document).ready(function(){
	if(document.getElementById("bind-captcha")){
        document.getElementById('bind-captcha').onclick = function(){
            try {
                var captcha = new TencentCaptcha(tecentappid, tecentCallback, {});
                // 调用方法，显示验证码
                captcha.show();
            } catch (error) {
                // 加载异常，调用验证码js加载错误处理函数
                // 生成容灾票据或自行做其它处理
                var ticket = 'terror_1001_' + tecentappid + Math.floor(new Date().getTime() / 1000);
                callback({
                    ret: 0,
                    randstr: '@'+ Math.random().toString(36).substr(2),
                    ticket,
                    errorCode: 1001,
                    errorMessage: 'jsload_error',
                });
            }
        }
	}

    $("#popup-submit").click(function(){
        $("input[name='verify_token']").val('');
        $("input[name='verify_str']").val('');
    });
});

