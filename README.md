# Cloudflare Wildcard Tester

This simple JSON API allows you test the wildcard Redirect from Cloudflare

<https://github.com/cloudflare/wildcard>

For those who can't run the code, you can use the following link to test the wildcard redirect:

- <https://wildcard-cf-worker.carrotbot.workers.dev/>

## Usage

There are two query parameters that can be used:
- `url`: The URL to test the wildcard redirect
- `wildcard`: The wildcard pattern to test

<https://wildcard-cf-worker.carrotbot.workers.dev/?url=https://example.com/blog/city-berlin&wildcard=*/blog/city-*>

It will return

```jsonc
{
    "captures": [
        "https://example.com", // 1st capture group
        "berlin" // 2nd capture group
    ]
}
```

This will allow you to use those capture groups in the URL redirect expression to use that. For example, if you want to redirect to `https://example.com/blog/city-berlin` to `https://example.com/blog/city/berlin`, you can use the following expression:

```
wildcard_replace(http.request.full_uri, "*/blog/city-*", "${1}/city/${2}")
```

<!-- ```json
{
  "url": "https://example.com/blog/berlini-berlin",
  "wildcard": "*/blog/berlini-*",
  "result": true
}
```
-->

## Development

To run the code locally, you need to install the wrangler CLI tool:

```bash
npx wrangler dev
```

This will start the server at <http://localhost:8787>

Example:

```bash
npx wrangler dev

 ⛅️ wrangler 3.80.5
-------------------

Running custom build: cargo install -q worker-build && worker-build --release
╭──────────────────────────────────────────────────────────────────────────────────────────────────╮
│  [b] open a browser, [d] open devtools, [l] turn off local mode, [c] clear console, [x] to exit  │
╰──────────────────────────────────────────────────────────────────────────────────────────────────╯
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
    Finished release [optimized] target(s) in 0.06s
[INFO]: ⬇️  Installing wasm-bindgen...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: ✨   Done in 0.35s
[INFO]: 📦   Your wasm pkg is ready to publish at /Users/gianpaj/tmp/wildcard-cf-worker/build.

⎔ Starting local server...
[wrangler:inf] Ready on http://localhost:8787
```

## TODO

- [ ] Add small front-end to test the wildcard matching
