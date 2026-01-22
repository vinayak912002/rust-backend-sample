## CRUD operations
- `\users` post - the request body will have the data (name, age)
- `\users` get - returns a list of users
- `\users\{id}` get - returns the details of that particular user
- `\users\{id}` post - update the user with the given id
- `\users\{id}` del - delete the user with the given id

## Database
- we will be using mysql as a database for this application
### we will also implement a caching strategy to reduce the number of requests to the database