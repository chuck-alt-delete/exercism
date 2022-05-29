"""REST API for IOUs"""
import json

class RestAPI:
    def __init__(self, database: dict[str:list[dict]] = None) -> None:
        if database:
            users = sorted(database["users"], key=lambda user: user["name"])
        self.database = {"users": users}

    def get(self, url: str, payload: str = None) -> str:
        if url != "/users":
            raise Exception("must GET /users")
        if not payload:
            self.database["users"].sort(key=lambda user: user["name"])
            return json.dumps(self.database)
        payload = json.loads(payload)
        user_names = payload["users"]
        user_names.sort()
        response = {"users": []}
        for user_name in user_names:
            for user in self.database["users"]:
                if user_name == user["name"]:
                    response["users"].append(user)
                    break
        return json.dumps(response)


    def post(self, url, payload=None):
        if url == "/iou":
            pass
        if url == "/add":
            payload = json.loads(payload)
            # Check if name is already in database
            for user in self.database["users"]:
                if payload["user"] == user["name"]:
                    raise Exception("user already exists")
            new_user = {
                "name": payload["user"],
                "owes": {},
                "owed_by": {},
                "balance": 0}
            self.database["users"].append(new_user)
            return json.dumps(new_user)
