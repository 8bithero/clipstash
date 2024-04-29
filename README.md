# ClipStash

Run ClipStash server
```
cargo run --bin httpd
```

## ClipClient - Client that uses the ClipStash API
### Build client:
```
cargo run --bin clipclient
```

### Client Usage instructions
```
USAGE:
    clipclient --api-key <api-key> [addr] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --api-key <api-key>

ARGS:
    <addr>     [env: CLIPSTASH_ADDR=]  [default: http://127.0.0.1:8000]

SUBCOMMANDS:
    get
    help      Prints this message or the help of the given subcommand(s)
    new
    update
```

### Generating a new API Key
1. Start the ClipStash server.
2. Visit http://127.0.0.1:8000/api/clip/key
3. The new API key will appear in the server logs.


### Create New Clip
```
cargo run --bin clipclient -- --api-key <api_key> new <any string content>
```

Example usage:
```
cargo run --bin clipclient -- --api-key 8w8ix6vuQoQKUbA+6GKybw== new "Hello world!"
```

This will return the following:
```
Clip {
    clip_id: ClipId(
        DbId(
            00000000-0000-0000-0000-000000000000,
        ),
    ),
    shortcode: ShortCode(
        "4dbcc3431c",
    ),
    content: Content(
        "Hello world!",
    ),
    title: Title(
        None,
    ),
    posted: Posted(
        Time(
            2024-04-29T12:58:40Z,
        ),
    ),
    expires: Expires(
        None,
    ),
    password: Password(
        None,
    ),
    hits: Hits(
        0,
    ),
}
```
Note, the clip_id will always returned as `00000000-0000-0000-0000-000000000000` to avoid leaking internal database IDs.

### Get Clip
```
cargo run --bin clipclient -- --api-key <api_key> get <shortcode>
```

Example usage:
```
cargo run --bin clipclient -- --api-key 8w8ix6vuQoQKUbA+6GKybw== get 4dbcc3431c
```
