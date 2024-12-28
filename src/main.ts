import { createApp } from "vue";
import App from "./App.vue";
import naive from 'naive-ui'
import { createPinia } from 'pinia'
import '../src/assets/main.css'

const app = createApp(App);
app.use(naive);
app.use(createPinia());
app.mount("#app");
