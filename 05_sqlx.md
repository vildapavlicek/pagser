1. add sqlx
2. impl each rpc endpoint with select
3. map to request

# Introduction
With server and logging being done it is time to take a look at database client and implement some selects so we return data from database.

# Sqlx
To interact with our DB we will use [sqlx](https://github.com/launchbadge/sqlx) which has async API as well as connection pool.