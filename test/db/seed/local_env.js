db.user.insertOne(
    {
        "_id": "7fa1d398-1697-4545-8372-b0c1332cce6d",
        "email": "benjaminjacobberg@gmail.com",
        "username": "benjamin",
        "active": true
    }
);

db.organization.insertOne(
    {
        "_id": "cc7980ed-b0fb-4118-847c-412a6dcc5fce",
        "organization_id": "6047f92e-d7c6-4b77-9950-08c27c350b18",
        "name": "benjamin",
        "members": ["7fa1d398-1697-4545-8372-b0c1332cce6d"]
    }
);

db.team.insertOne(
    {
        "_id": "cc7980ed-b0fb-4118-847c-412a6dcc5fce",
        "organization_id": "6047f92e-d7c6-4b77-9950-08c27c350b18",
        "name": "benjamin",
        "members": ["7fa1d398-1697-4545-8372-b0c1332cce6d"]
    }
);