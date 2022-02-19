# NodeJS module (Rust underhood)
## Powered by @napi-rs

Just an example how you can use **Node EventEmitter** and **@napi-rs** (rust)
Module implements class, where constructor is accepting **.emit** function

Example of usage is in **example.js** file and listed below
```javascript
    {
        const api = require('./') // includes itself
        const filename = '1oo.bin';
        const url = 'https://speed.hetzner.de/100MB.bin';
        const emitter = new EventEmitter();
        emitter.on('progress', (data) => {
            console.info(`${url}: ${Math.round((data.downloaded / data.total) * 100)}%`);
        });
        const downloader = new api.FileDownloader(emitter.emit.bind(emitter)); // yeah, constructor accept emit function to use it in Rust
        downloader.downloadFile(url, filename).then((status) => {
            console.info('completed: ', url, filename);
        });
    }
```

## Tech

- [<https://napi.rs>] - enhance your nodeJS within Rust power!
- [<https://www.rust-lang.org>] - lovely and powerful language