<template>
    <n-list bordered>
        <n-list-item v-for="task in tasks" v-bind:key="task">
            <TaskItem :id="task.id" :status="task.status" :name="task.name"/>
        </n-list-item>
    </n-list>
</template>

<script>
import TaskItem from "./TaskItem.vue"

export default {
    components: {
        TaskItem
    },
    data() {
        return {
            tasks: [
                {
                    id: 1,
                    name: 'Task1',
                    status: 'NotStarted'
                },
                {
                    id: 2,
                    name: 'Task2',
                    status: 'Done'
                },
                {
                    id: 3,
                    name: 'Task3',
                    status: 'Done'
                },
                {
                    id: 4,
                    name: 'Task4',
                    status: 'Started'
                },
                {
                    id: 5,
                    name: 'Task5',
                    status: 'Paused'
                },
                {
                    id: 6,
                    name: 'Task6',
                    status: 'Aborted'
                },
                {
                    id: 7,
                    name: 'Task7',
                    status: 'NotStarted'
                },
            ]
        }
    }
}
</script>