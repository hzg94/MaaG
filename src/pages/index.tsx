import {Button} from "antd";
import React, {useState} from 'react'
import {AdbDeviceList} from "@/api/adb/adb";

export default () => {


    return (
        <div>
            <h1>AA</h1>
            <Button onClick={() => {
                AdbDeviceList().then(x => console.log(x))
            }}>click</Button>
        </div>
    );
}
