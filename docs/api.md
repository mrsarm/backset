# Backset API

Here is the list of endpoints.

## Endpoints usage

All the examples are provided with *HTTPie*, and assumed the default
port `8558` was not changed (env `PORT`), nor the default URI prefix `/`
(env `APP_URI`).

### Health check

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

### Tenants endpoints

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

#### GET /tenants/{id}

Having a tenant with id `my-tenant`:

```shell
http :8558/tenants/my-tenant
HTTP/1.1 200 OK
content-type: application/json
...

{
    "id": "my-tenant",
    "name": "Tenant Name",
    "created_at":"2023-04-21T09:20:40.128477"
}
```

#### GET /tenants

Query arguments:

- `q`: optional, a string used to search by id or name.
- `page_size`: optional integer, default 50.
- `offset`: optional integer, default 0.
- `sort`: optional, default "id". Possible options are "id", "name" or "created_at",
  and using the "-" prefix the sorting is from z-a instead of a-z, e.g. use
  "-name" to sort by name in reverse order.
- `include_total`: optional boolean, default true. If true include a count of the
  total records in the database in the field `total`.

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
    "page_size": 5,
    "total": 32
}
```

#### DELETE /tenants/{id}

```shell
$ http DELETE :8558/tenants/my-tenant
HTTP/1.1 200 OK
content-type: application/json
...

{ "deleted": 1 }
```

The tenant cannot have elements before deletion. In case it
has, an HTTP 400 is returned, unless the endpoint is called
with `?force=true`, in which case all elements will be deleted
as well, and the `deleted` field in the JSON response will
inform the number of elements deleted + 1 (the tenant).

#### PUT /tenants/{id}

To create a new tenant with `PUT` or override tenant name from existent one:

```shell
$ http PUT :8558/tenants/my-tenant --raw '{"name": "New Tenant Name"}'
HTTP/1.1 200 OK
content-type: application/json
...

{
    "id": "my-tenant",
    "name": "New Tenant Name",
    "created_at": "2023-05-19T20:04:26.331117"
}
```

### Elements in tenants endpoints

In the examples is assumed a tenant "collections" was
created (e.g. `http :8558/tenants id=collections name="Collections API"`).

#### POST /{tenant}

Any field can be provided except `created_at` that is autogenerated.
If `id` is not provided, a big random number is used to assign the
PK of the new record.

```shell
http :8558/collections --raw '{"id": "1234", "name": "Obj name"}'
HTTP/1.1 201 Created
content-type: application/json
...

{
    "created_at": "2023-09-26T01:22:34.787066",
    "id": "1234",
    "name": "Obj name"
}
```

#### GET /{tenant}/{id}

Having a tenant with id `collections` with a record with id `1234`:

```shell
http :8558/collections/1234
HTTP/1.1 200 OK
content-type: application/json
...

{
    "created_at": "2023-09-26T01:22:34.787066",
    "id": "1234",
    "name": "Obj name"
}
```

#### GET /{tenant}

List all elements from a tenant.

Query arguments:

- `page_size`: optional integer, default 50.
- `offset`: optional integer, default 0.
- `include_total`: optional boolean, default true. If true include a count of the
  total records in the database in the field `total`.

```shell
$ http ":8558/collections?page_size=5&offset=10"
HTTP/1.1 200 OK
content-type: application/json
...

{
    "data": [
        {
            "created_at": "2023-09-26T02:04:38.980746",
            "id": "1235",
            "name": "Obj name"
        },
        {
            "created_at": "2023-09-26T02:02:27.173042",
            "id": "fixed-id",
            "name": "Example created with an id set",
            "some-field": ["val", 1]
        },
        ...
    ],
    "offset": 10,
    "page_size": 3,
    "total": 14
}
```

#### DELETE /{tenant}/{id}



```shell
$ http DELETE :8558/collections/1235
HTTP/1.1 204 No Content
date: Sun, ...
```

#### PUT /{tenant}/{id}

Create new element or override element values (except `created_at` that is preserved):

```shell
$ http PUT :8558/collections/1235 --raw '{"name": "New obj name", "another": "prop"}'
HTTP/1.1 200 OK
content-type: application/json
...

{
    "id": "1235",
    "name": "New obj name",
    "another": "prop",
    "created_at": "2023-05-19T20:04:26.331117"
}
```
