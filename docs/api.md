## API

Here is the list of endpoints.

### Endpoints usage

All the examples are provided with *HTTPie*, and assumed the default
port `8558` was not changed (env `PORT`), nor the default URI prefix `/`
(env `APP_URI`).

#### Health check

To check whether the service is running (it does not check the DB connection).

```shell
http :8558/health
HTTP/1.1 200 OK
content-type: application/json
...

{
    "service": "backset",
    "status": "UP"
}
```

#### POST /tenants

```shell
http :8558/tenants --raw '{"id": "tenant-me", "name": "Tenant Name"}'
HTTP/1.1 201 Created
content-type: application/json
...

{
    "id": "tenant-me",
    "name": "Tenant Name",
    "created_at": "2023-05-19T20:04:26.331117"
}
```

#### GET /{id}

Having a tenant with id `my-tenant`:

```shell
http :8558/my-tenant
HTTP/1.1 200 OK
content-type: application/json
...

{
    "id": "my-tenant",
    "name": "Tenant Name",
    "created_at":"2023-04-21T09:20:40.128477"
}
```

(Note that getting a tenant info is not under the
`/tenants` URI but under the root of the API routes) 

#### GET /tenants

Query arguments:

- `q`: optional, a string used to search by id or name.
- `page_size`: optional, default 50.
- `offset`: optional, default 0.
- `sort`: optional, default "id". Possible options are "id", "name" or "created_at",
  and using the "-" prefix the sorting is from z-a instead of a-z, e.g. use
  "-name" to sort by name in reverse order.

```shell
$ http ":8558/tenants?page_size=5&offset=10"
HTTP/1.1 200 OK
content-type: application/json
...

{
    "data": [
        {
            "id": "my-tenant",
            "name": "A Tenant",
            "created_at":"2023-05-01T00:54:38.936738"
        },
        {
            "id": "tags",
            "name": "Tags collection",
            "created_at":"2023-04-20T10:00:11.572824"
        },
        ...
    ],
    "offset": 10,
    "page_size": 2,
    "total": 12
}
```

#### DELETE /tenants/{id}

```shell
$ http DELETE :8558/tenants/my-tentant
HTTP/1.1 204 No Content
...
```
