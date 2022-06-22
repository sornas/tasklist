<template>
    <h1>Unfinished tasks</h1>
    <n-grid cols="1 m:3" responsive="screen" x-gap="12">
        <n-gi>
            <h2>In progress</h2>
            <TaskList />
        </n-gi>
        <n-gi>
            <h2>Not started</h2>
            <TaskList />
        </n-gi>
        <n-gi>
            <h2>On pause</h2>
        </n-gi>
    </n-grid>
    <h1>Routines</h1>
</template>

<script>
import TaskList from "../components/TaskList.vue"

export default {
    name: 'HomeView',
    components: {
        TaskList
    }
}
</script>