# Application Update server

The protocol is inspired by Google's [Omaha protocol](https://github.com/google/omaha/blob/master/doc/ServerProtocolV3.md) but is modernized and simplified.

It should be simple to implement new clients in any language, but there is
a [supported client](https://github.com/rhelmer/update-client#readme)
available.

## Getting started

### Build and run.
`cargo run`

The server is now running on port 8000.

## Protocol

The server expects an HTTP form POST with a JSON document:

```
{
    "product": "your_app",
    "version": 666,
    "platform": "macOS",
    "locale": "en-US"
}
```

If no updates are available, the server will return the
HTTP code `204 No Content`.

If updates are available, the server will return a list
of available updates along with a request ID:

```
{
    "available_updates": [
        { "update_type": "my_important_data",
          "url": "https://localhost:8080/src/important_data_v1.json",
          "hash_function": "sha512",
          "hash_value": "abc123",
          "size": 1234,
          "version": "1.0"
        },
        { "update_type": "my_important_binary_blob",
          "url": "https://localhost:8080/src/important_binary_v2.zip",
          "hash_function": "sha512",
          "hash_value": "321cba",
          "size": 4321,
          "version": "2.0"
        }
    ],
    "request_id": "xyz321"
}
```

The client may send a "completion" POST to the server to
indicate success or failure to download and apply updates:

```
{
    "updates": {
        "blocklist": {
            "status": "success",
            "version": "1.0"
        },
        "gecko_media_plugins": {
            "status": "failed",
            "reason": "bad checksum",
            "version": "2.0",
            "hash_function": "sha512",
            "hash_value": "abc123",
            "size": 1234
        }
    }
    "request_id": "xyz321",
}
```

The client sends back the server-defined request ID, which the server will record.
