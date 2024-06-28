// server.js
import express from 'express';
import https from 'https';
import path from 'path';
import fs from 'fs';
// const exppress = require('express')
// const https = require('https');
// const path = require('path');
// const fs = require('fs');

const app = express();
const port = 3000;

app.get("/", (req, res) => {
    res.send("Hello from the embedded Express server!");
});

app.get("/checkUpdate", (req, res) => {
    console.log('<=====开始检查版本=====>')

    // Step1 在OSS中配置一份manifest.json文件,内容如下
    const mainfestJson = {
        "pub_date": "2020-06-22T19:25:57Z",
        "notes": "Test version",
        "version": "v1.0.3",
        "platforms": {
            "darwin-x86_64": {
                "signature": "Content of app.tar.gz.sig",
                "url": "https://github.com/username/reponame/releases/download/v1.0.0/app-x86_64.app.tar.gz"
            },
            "darwin-aarch64": {
                "signature": "Content of app.tar.gz.sig",
                "url": "https://github.com/username/reponame/releases/download/v1.0.0/app-aarch64.app.tar.gz"
            },
            "windows-x86_64": {
                "signature": "Content of app.msi.sig",
                "url": "https://github.com/username/reponame/releases/download/v1.0.0/app-x64.msi.zip"
            },
            "linux-x86_64": {
                "signature": "Content of app.AppImage.tar.gz.sig",
                "url": "https://github.com/username/reponame/releases/download/v1.0.0/app-amd64.AppImage.tar.gz"
            }
        }
    };
    console.log('mainfestJson====>',mainfestJson)
    // Step2 通过请求获取manifest.json内容
    // const mainfestJson = await request('OSS链接')

    // Step3 从本地读取当前APP版本号
    const version = '0.0.2';

    // Step4 判断当前版本与manifest.json中的版本是否一致，不一致则提示更新
    const needUpdate = mainfestJson.version !== version;
    console.log(`当前APP版本为${version}，云端版本为${mainfestJson.version}，${needUpdate ? '':'不'}需要更新APP`)
    if (needUpdate) {
        res.status(200).json(mainfestJson);
    } else {
        res.status(204).json();
    }
});

const privateKey = fs.readFileSync('./key.pem', 'utf8');
const certificate = fs.readFileSync('./cert.pem', 'utf8');
// 根据私钥和证书创建 HTTPS 服务选项。
const credentials = { key: privateKey, cert: certificate };

// 创建 HTTPS 服务器。
const httpsServer = https.createServer(credentials, app);
// 指定端口并启动服务器。
httpsServer.listen(port, () => {
    console.log('HTTPS Server running on port 8443');
});
