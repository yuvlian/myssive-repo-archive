module Tiny = Tiny_httpd


let risky_api_check _ =
  Tiny.Response.make_string (Ok 
    "{\n\
     \"data\": {},\n\
     \"message\": \"OK\",\n\
     \"retcode\": 0\n\
    }"
  )


let mdk_shield_login _ =
  Tiny.Response.make_string (Ok 
    "{\n\
     \"data\": {\n\
       \"account\": {\n\
         \"area_code\": \"**\",\n\
         \"email\": \"yulian@naver.com\",\n\
         \"country\": \"ID\",\n\
         \"is_email_verify\": \"1\",\n\
         \"token\": \"p\",\n\
         \"uid\": \"1\"\n\
       },\n\
       \"device_grant_required\": false,\n\
       \"reactivate_required\": false,\n\
       \"realperson_required\": false,\n\
       \"safe_mobile_required\": false\n\
     },\n\
     \"message\": \"OK\",\n\
     \"retcode\": 0\n\
    }"
  )


let granter_login_v2 _ =
  Tiny.Response.make_string (Ok 
    "{\n\
     \"data\": {\n\
       \"account_type\": 1,\n\
       \"combo_id\": \"1\",\n\
       \"combo_token\": \"p\",\n\
       \"data\": \"{\\\"guest\\\":false}\",\n\
       \"heartbeat\": false,\n\
       \"open_id\": \"1\"\n\
     },\n\
     \"message\": \"OK\",\n\
     \"retcode\": 0\n\
    }"
  )
