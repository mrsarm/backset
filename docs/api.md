## API

Here is the list of endpoints with sample usage.

### Endpoints usage

All the examples are provided with *HTTPie*.

#### Health check

To check the service is running (it does not check the DB connection).

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
http :8558/tenants --raw '{"name": "Tenant Name"}'
HTTP/1.1 201 Created
content-type: application/json
...

{
    "id": 17,
    "name": "Tenant Name"
}
```

#### GET /tenants/{id}

```shell
http :8558/tenants/17
HTTP/1.1 200 OK
content-type: application/json
...

{
    "id": 17,
    "name": "Tenant Name"
}
```

#### GET /tenants

Query arguments `page_size` (default 50) and `offset` (default 0) are optionals.

```shell
$ http ":8558/tenants?page_size=5&offset=10"
HTTP/1.1 200 OK
content-type: application/json
...

{
    "data": [
        {
            "id": 12,
            "name": "Some Tenant"
        },
        {
            "id": 13,
            "name": "Another Tenant"
        },
        ...
    ],
    "offset": 10,
    "page_size": 5,
    "total": 28
}
```

#### DELETE /tenants/{id}

```shell
$ http DELETE :8558/tenants/8
HTTP/1.1 204 No Content
...
```
