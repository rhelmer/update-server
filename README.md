Firefox Update
==============

A server and client for Firefox updates.

The client may be run standalone, or as a Windows service.
The server depends on PostgreSQL, which must be populated

The protocol is inspired by Google's Omaha client+server,
but is modernized and simplified.

Building and running
====================

Server
------

`cd server/`

# Create database and load schema.
`createdb updates`
`psql -f schema.sql updates`

# Load sample data.
`psql -f sample_data.sql updates`

# Build and run.
`cargo run`

The server is now running on port 9999.

Client
------

`cd client/`

# Build and run. Pass the JSON to use for the update request, and the completion request.
`cargo run --request=./request.json --complete=./complete.json`

The client will attempt to connect to port 9999 and download any available updates.