import {useEffect} from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import {
    checkUpdate,
    installUpdate,
} from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process';
import { Modal,notification } from 'antd'
import "./App.css";

const { confirm } = Modal;

function App() {

    const check = async () => {
        try {
            const res:any = await checkUpdate()
            console.log("shouldUpdate======>",res)
            const { shouldUpdate, manifest = {} } = res;
            if (shouldUpdate) {
                // You could show a dialog asking the user if they want to install the update here.
                console.log(
                    `Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
                )

                confirm({
                    title: '检测到APP有更新，请确认是否更新?',
                    content: 'Some descriptions',
                    async onOk() {
                        try {
                            notification.info({
                                message: '正在下载更新...',
                                duration: 3000,
                            });
                            await installUpdate();
                            await relaunch();
                        } catch (e) {
                            notification.error({
                                message: '下载更新失败',
                                description: e?.toString() || '',
                            });
                        }
                    },
                    onCancel() {
                        console.log('Cancel');
                    },
                });

            }
        } catch (error) {
            console.error(error)
        }
    }
    async function onSubmit() {
       try{
           const res = await invoke("install_pnpm");
           notification.success({
               message: 'Success',
               description: `Error: ${res}`,
           });
       }catch (error){
           notification.error({
               message: 'Error executing command',
               description: `Error: ${error}`,
           });
       }
    }

    useEffect(()=>{
        check()
    },[])

    return (
        <div className="container">
            <h1>Welcome to Tauri!</h1>

            <div className="row">
                <a href="https://vitejs.dev" target="_blank">
                    <img
                        src="/vite.svg"
                        className="logo vite"
                        alt="Vite logo"
                    />
                </a>
                <a href="https://tauri.app" target="_blank">
                    <img
                        src="/tauri.svg"
                        className="logo tauri"
                        alt="Tauri logo"
                    />
                </a>
                <a href="https://reactjs.org" target="_blank">
                    <img
                        src={reactLogo}
                        className="logo react"
                        alt="React logo"
                    />
                </a>
            </div>

            <p>Click on the Tauri, Vite, and React logos to learn more.</p>

            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    onSubmit()
                }}
            >
                {/*<button type="submit">检查更新</button>*/}
                <button type="submit">获取版本</button>
            </form>

        </div>
    );
}

export default App;
