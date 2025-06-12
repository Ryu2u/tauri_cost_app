import {createRouter, createWebHashHistory} from "vue-router";
//引入组件
// import Home from "../views/Home.vue";
// import About from "../views/About.vue";

const router = createRouter({
    //哈希模式
    history: createWebHashHistory(),
    routes: [
        // 通过数组对象的形式，配置路径对应展示的组件。
        {
            path: '/',
            component: import("../views/home/index.vue")
        }
    ],
});
// 将 router 暴露出去  （export default 抛出方式）
export default router;