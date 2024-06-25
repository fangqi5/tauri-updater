// server.js
import express from 'express';

const app = express();
const port = 3000;

app.get("/", (req, res) => {
    res.send("Hello from the embedded Express server!");
});

app.get("/checkUpdate", (req, res) => {
    // Step1 在OSS中配置一份manifest.json文件,内容如下
    const mainfestJson = {
        pub_date: Date.now(),
        version: "0.0.1",
        platforms: {
            "darwin-x86_64": {
                signature: "Content of app.tar.gz.sig",
                url: "https://github.com/username/reponame/releases/download/v1.0.0/app-x86_64.app.tar.gz",
            },
            "darwin-aarch64": {
                signature: "Content of app.tar.gz.sig",
                url: "https://github.com/username/reponame/releases/download/v1.0.0/app-aarch64.app.tar.gz",
            },
            "windows-x86_64": {
                signature: "Content of app.msi.sig",
                url: "https://github.com/username/reponame/releases/download/v1.0.0/app-x64.msi.zip",
            },
            "linux-x86_64": {
                signature: "Content of app.AppImage.tar.gz.sig",
                url: "https://github.com/username/reponame/releases/download/v1.0.0/app-amd64.AppImage.tar.gz",
            },
        },
    };
    console.log('mainfestJson====>',mainfestJson)
    // Step2 通过请求获取manifest.json内容
    // const mainfestJson = await request('OSS链接')

    // Step3 从本地读取当前APP版本号
    const version = '0.0.1';

    // Step4 判断当前版本与manifest.json中的版本是否一致，不一致则提示更新
    const needUpdate = mainfestJson.version !== version;
    console.log(`当前APP版本为${version}，云端版本为${mainfestJson.version}，${needUpdate ? '':'不'}需要更新APP`)
    if (needUpdate) {
        res.status(200).send(mainfestJson);
    } else {
        res.status(204).send();
    }
});

app.listen(port, () => {
    console.log(`Server is running at http://localhost:${port}`);
});
