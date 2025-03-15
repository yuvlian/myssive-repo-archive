from pkg.config import MONGODB_URL, DB_NAME
from motor import motor_asyncio

CLIENT = motor_asyncio.AsyncIOMotorClient(MONGODB_URL)


def get_database():
    return CLIENT.get_database(DB_NAME)
