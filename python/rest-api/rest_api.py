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
            payload = json.loads(payload)
            for user in self.database["users"]:
                if user["name"] == payload["borrower"]:
                    if payload["lender"] in user["owes"]:
                        user["owes"][payload["lender"]] += payload["amount"]
                    elif payload["lender"] in user["owed_by"]:
                        user["owed_by"][payload["lender"]] -= payload["amount"]
                        if user["owed_by"][payload["lender"]] < 0:
                            user["owes"][payload["lender"]] = -1 * user["owed_by"].pop(payload["lender"])
                    else:
                        user["owes"][payload["lender"]] = payload["amount"]
                    user["balance"] -= payload["amount"]
                    user["owes"] = {name: amount for name, amount in user["owes"].items() if amount != 0}
                    user["owed_by"] = {name: amount for name, amount in user["owed_by"].items() if amount != 0}
                    borrower = user
                if user["name"] == payload["lender"]:
                    if payload["borrower"] in user["owed_by"]:
                        user["owed_by"][payload["borrower"]] += payload["amount"]
                    elif payload["borrower"] in user["owes"]:
                        user["owes"][payload["borrower"]] -= payload["amount"]
                        if user["owes"][payload["borrower"]] < 0:
                            user["owed_by"][payload["borrower"]] = -1 * user["owes"].pop(payload["borrower"])
                    else:
                        user["owed_by"][payload["borrower"]] = payload["amount"]
                    user["balance"] += payload["amount"]
                    user["owes"] = {name: amount for name, amount in user["owes"].items() if amount != 0}
                    user["owed_by"] = {name: amount for name, amount in user["owed_by"].items() if amount != 0}
                    lender = user
            response_users = sorted([lender, borrower], key=lambda user: user["name"])
            return json.dumps({"users": response_users})
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

if __name__ == "__main__":
    database = {
            "users": [
                {"name": "Adam", "owes": {"Bob": 3.0}, "owed_by": {}, "balance": -3.0},
                {"name": "Bob", "owes": {}, "owed_by": {"Adam": 3.0}, "balance": 3.0},
            ]
        }
    api = RestAPI(database)
    payload = json.dumps({"lender": "Adam", "borrower": "Bob", "amount": 2.0})
    response = api.post("/iou", payload)
    print(response)