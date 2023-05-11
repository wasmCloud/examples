# Leaderboard Actor
This actor was created as part of a [blog post](https://cosmonic.com/blog/engineering/building-with-cosmonic-part-1-deploy-leaderboard-service) illustrating the ease with which people can create new actors and deploy them.

This actor exposes the following RESTful API to manipulate leaderboards and their scores:

| Resource | Method | Description |
|---|---|---|
| `/leaderboards` | GET | Retrieves the names and IDs of all leaderboards |
| `/leaderboards` | POST | Creates a new leaderboard |
| `/leaderboards/{id}` | GET | Retrieves a leaderboard with a given ID |
| `/leaderboards/{id}/scores` | POST | Adds a score to the leaderboard. Note that the score could "fall off" the bottom if low enough |
