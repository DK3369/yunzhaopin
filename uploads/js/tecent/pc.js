// Tencent captcha callback.
function tecentCallback(res) {
	// var url = web_url+'/index.php?m=tecentcode'
    if (res.ret === 0) {
        $("input[name='verify_token']").val(res.ticket);
        $("input[name='verify_str']").val(res.randstr);
        // Submit operation.
        var type = $('#bind-captcha').attr('data-type');
        var dataid = $('#bind-captcha').attr('data-id');
        // Submit the form.
        if(type=='submit'){
            $('#'+dataid).submit();
        }else{
            // Simulate a click.
            $("#"+dataid).trigger("click");
        }
        // $.post(url, {
        //     ticket: res.ticket,
        //     str: res.randstr
        // }, function(data) {
        //     if(data == 0) {// Secondary verification failed; refresh the captcha.
        //         $('#bind-captcha').click();
        //     } else {
        //         $("input[name='verify_token']").val(res.ticket);
        //         // Submit operation.
        //         var type = $('#bind-captcha').attr('data-type');
        //         var dataid = $('#bind-captcha').attr('data-id');
        //         // debugger
        //         // Submit the form.
        //         if(type=='submit'){
        //             $('#'+dataid).submit();
        //         }else{
        //             // Simulate a click.
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
                // Call the method to show the captcha.
                captcha.show();
            } catch (error) {
                // On load errors, call the captcha JS load-error handler.
                // Generate a fallback ticket or handle it as needed.
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
