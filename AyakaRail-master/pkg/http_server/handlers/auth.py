from fastapi import APIRouter
from pkg.http_server.models import (
    Empty,
    AuthRsp,
    AppLoginByPassword,
    ExtraUserInfo,
    UserInfo,
    UserToken,
    ComboGranterLoginV2,
    MdkAccount,
    MdkShieldLogin,
)
from pkg.config import UID

r = APIRouter()


@r.post("/account/risky/api/check")
async def on_risky_api_check():
    return AuthRsp[Empty]()


@r.post("/{game_biz}/account/ma-passport/api/appLoginByPassword")
async def on_login_by_password():
    return AuthRsp[AppLoginByPassword](
        data=AppLoginByPassword(
            bind_email_action_ticket="",
            ext_user_info=ExtraUserInfo(
                birth="0",
                guardian_email="",
            ),
            reactivate_action_token="",
            token=UserToken(
                token_type=1,
                token="token",
            ),
            user_info=UserInfo(
                account_name="ayaka",
                aid="1",
                area_code="**",
                token="token",
                email="ahy@k.a",
                is_email_verify="1",
                country="ID",
            ),
        )
    )


@r.post("/{game_biz}/combo/granter/login/v2/login")
async def on_combo_granter_login():
    return AuthRsp[ComboGranterLoginV2](
        data=ComboGranterLoginV2(
            account_type=1,
            combo_id="1",
            combo_token="token",
            data=r"""{"guest":false}""",
            heartbeat=False,
            open_id="1",
        )
    )


async def on_mdk_shield_api():
    return AuthRsp[MdkShieldLogin](
        data=MdkShieldLogin(
            account=MdkAccount(
                area_code="**",
                email="ahy@k.a",
                country="ID",
                is_email_verify="1",
                token="token",
                uid=str(UID),
            ),
            device_grant_required=False,
            reactivate_required=False,
            realperson_required=False,
            safe_mobile_required=False,
        )
    )


r.post("/{game_biz}/mdk/shield/api/login")(on_mdk_shield_api)
r.post("/{game_biz}/mdk/shield/api/verify")(on_mdk_shield_api)
