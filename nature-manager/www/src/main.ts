import { createApp } from 'vue'
import 'uno.css'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import "~/styles/index.scss";
import App from './App.vue'

const app = createApp(App)
app.use(ElementPlus)
app.mount('#app')
