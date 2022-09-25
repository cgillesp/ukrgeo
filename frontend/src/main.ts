import { createApp } from 'vue'
import Home from './Home.vue'
import * as vr from 'vue-router'
import Holder from './holder.vue'
import Admin from './Admin.vue'


const routes = [
  { path: '/', component: Home },
  { path: '/admin', component: Admin}
]

const router = vr.createRouter({
  history: vr.createWebHashHistory(),
  routes
})

let app = createApp(Holder)

app.use(router)
app.mount('#app')
