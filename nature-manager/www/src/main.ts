import { createApp } from 'vue'
import 'uno.css'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'

// import * as wasm from "nature-manager";
// wasm.greet()

import init, {greet} from "nature-manager";
init().then((_exports)=>{
    greet() 
});

const app = createApp(App)
app.use(ElementPlus)
app.mount('#app')
