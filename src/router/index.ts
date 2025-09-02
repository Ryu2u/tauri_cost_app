import {createRouter, createWebHistory} from '@ionic/vue-router';
import {RouteRecordRaw} from 'vue-router';

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        redirect: '/ledgerPage'
    },
    {
        path: '/',
        component: () => import('../views/HomePage.vue'),
        children: [
            {
                path: '',
                redirect: '/ledgerPage'
            },
            {
                path: 'ledgerPage',
                component: () => import('../views/LedgerPage.vue')
            },
            {
                path: 'assetsPage',
                component: () => import('../views/AssetsPage.vue')
            },
            {
                path: 'statisticsPage',
                component: () => import('../views/StatisticsPage.vue')
            }
        ]
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes
})

export default router
