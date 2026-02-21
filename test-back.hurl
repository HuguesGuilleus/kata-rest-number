#!/usr/bin/env -S hurl --test --variable='SERVER=http://localhost:8000'

# Delete (init the server)
DELETE {{SERVER}}/
HTTP 200
Content-Type: application/json
`0`

# GET and HEAD
GET {{SERVER}}/
HTTP 200
Content-Type: application/json
`0`
HEAD {{SERVER}}/
HTTP 200
Content-Type: application/json
``

# Path not found
GET {{SERVER}}/unknown-path
HTTP 404
Content-Type: application/json
`"404 Path not found"\r\n`

# Increment
POST {{SERVER}}/
HTTP 200
Content-Type: application/json
`1`
POST {{SERVER}}/
HTTP 200
Content-Type: application/json
`2`
GET {{SERVER}}/
HTTP 200
Content-Type: application/json
`2`

# Set by path
PUT {{SERVER}}/set-by-path/-13/
HTTP 400
Content-Type: application/json
`"400 Need a path as /set-by-path/:int"\r\n`
PUT {{SERVER}}/set-by-path/-13
HTTP 200
Content-Type: application/json
`-13`

# Set by query
PUT {{SERVER}}/set-by-query?nb
HTTP 400
Content-Type: application/json
`"400 Need integer in query with name nb"\r\n`
PUT {{SERVER}}/set-by-query?nb=-36
HTTP 200
Content-Type: application/json
`-36`

# Set by header
PUT {{SERVER}}/set-by-header
HTTP 400
Content-Type: application/json
`"400 Need integer in header with name x-nb"\r\n`
PUT {{SERVER}}/set-by-header
x-nb: -42
HTTP 200
Content-Type: application/json
`-42`

# Set by body
PUT {{SERVER}}/set-by-body
HTTP 400
Content-Type: application/json
`"400 Need integer in the body"\r\n`
PUT {{SERVER}}/set-by-body
`-42`
HTTP 200
Content-Type: application/json
`-42`

# Check not PUT method to put path
POST {{SERVER}}/set-by-query
HTTP 405
Content-Type: application/json
`"405 Method not allowed"\r\n`
POST {{SERVER}}/set-by-header
HTTP 405
Content-Type: application/json
`"405 Method not allowed"\r\n`
POST {{SERVER}}/set-by-body
HTTP 405
Content-Type: application/json
`"405 Method not allowed"\r\n`