const api = require('./');
const { setTimeout } = require('timers/promises');
const {EventEmitter} = require('events');

async function main() {
    api.setupLog();
    {
        const filename = '1oo.bin';
        const url = 'https://speed.hetzner.de/100MB.bin';
        const emitter = new EventEmitter();
        emitter.on('progress', (data) => {
            console.info(`${url}: ${Math.round((data.downloaded / data.total) * 100)}%`);
        });
        const downloader = new api.FileDownloader(emitter.emit.bind(emitter));
        console.time(filename);
        downloader.downloadFile(url, filename).then((status) => {
            console.info('completed: ', url, filename);
            console.timeEnd(filename);
        });
    }
}

main().then(() => {}).catch((error) => { console.error('Error: ', error); });