import { createApp } from "vue";
import "./main.css";
import App from "./App.vue";
import { setup_log_listener } from "./ts/helpers/logger.ts";

const app = createApp(App);
app.mount("#app");
setup_log_listener();