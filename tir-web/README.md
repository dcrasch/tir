# Tessellation Web

## Live version

https://dcrasch.github.io/tir/

## Build the project

### Requirements

* rust
* wasm-pack
* trunk https://trunkrs.dev/


## deployment

Use `--public-url` to set the base url.
```
trunk build --release
scp ../docs/* you@example.com:/var/www
```

or copy it to your docs directory and use github.io

## development

Start dev server and opens a browser.
```
trunk serve --release --open
```

## Notes

* Set mode: 'production' or it will be very slow
* Doesn't work in old Edge browsers (12-18) https://caniuse.com/mdn-api_textencoder_encoding

## Links

* https://trunkrs.dev/
* https://rustwasm.github.io/docs/wasm-bindgen/
* https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
* https://github.com/rustwasm/wasm-bindgen/tree/master/examples/paint
* https://gtk-rs.org/docs-src/tutorial/closures
* https://github.com/sn99/wasm-template-rust
* https://rustwasm.github.io/book/reference/deploying-to-production.html#ensure-that-your-http-server-uses-the-applicationwasm-mime-type
