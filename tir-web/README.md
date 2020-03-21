# Tessellation Web

## Live version

https://dcrasch.github.io/tir/

## Build the project

```
npm install
```

## Run the project

```
npm run serve
```


## Deploy the project

Build the distribution

```
npm run build
```

```
scp dist/* you@example.com:/var/www
```

or copy it to your docs directory and use github.io

## Run tests

```
wasm-pack test
```


## NPM Security audit


Fix warnings of github security alerts
```
rm package-lock.json
npm audit fix
```


## Notes

* Set mode: 'production' or it will be very slow
* Use the text-encoding for Internet Edge, or it will not load 

## Links

* https://rustwasm.github.io/docs/wasm-bindgen/
* https://rustwasm.github.io/docs/book/game-of-life/hello-world.html
* https://github.com/rustwasm/wasm-bindgen/tree/master/examples/paint
* https://gtk-rs.org/docs-src/tutorial/closures
* https://github.com/sn99/wasm-template-rust
* https://rustwasm.github.io/book/reference/deploying-to-production.html#ensure-that-your-http-server-uses-the-applicationwasm-mime-type
