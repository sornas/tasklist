import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/HomeView.vue'
import Archive from '../views/ArchiveView.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: Home
  },
  {
    path: '/archive',
    name: 'archive',
    component: Archive
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
