from typing import Generic, TypeVar
from pydantic import BaseModel

T = TypeVar("T", bound=BaseModel)


class Empty(BaseModel):
    pass


class AuthRsp(BaseModel, Generic[T]):
    data: T = Empty()
    message: str = "OK"
    retcode: int = 0


class ExtraUserInfo(BaseModel):
    birth: str
    guardian_email: str


class UserToken(BaseModel):
    token_type: int
    token: str


class UserInfo(BaseModel):
    account_name: str
    aid: str
    area_code: str
    token: str
    email: str
    is_email_verify: str
    country: str


class AppLoginByPassword(BaseModel):
    bind_email_action_ticket: str
    ext_user_info: ExtraUserInfo
    reactivate_action_token: str
    token: UserToken
    user_info: UserInfo


class ComboGranterLoginV2(BaseModel):
    account_type: int
    combo_id: str
    combo_token: str
    data: str
    heartbeat: bool
    open_id: str


class MdkAccount(BaseModel):
    area_code: str
    email: str
    country: str
    is_email_verify: str
    token: str
    uid: str


class MdkShieldLogin(BaseModel):
    account: MdkAccount
    device_grant_required: bool
    reactivate_required: bool
    realperson_required: bool
    safe_mobile_required: bool
