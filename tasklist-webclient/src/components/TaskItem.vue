<template>
    <n-thing content-indented>
        <template #avatar>
            <n-avatar>
                <n-icon>
                    <CheckBoxOutlineBlankOutlined v-if="status === 'NotStarted'" />
                    <TimelapseOutlined v-else-if="status === 'Started'" />
                    <PauseOutlined v-else-if="status === 'Paused'" />
                    <ClearOutlined v-else-if="status === 'Aborted'" />
                    <CheckOutlined v-else-if="status === 'Done'" />
                    <QuestionMarkOutlined v-else />
                </n-icon>
            </n-avatar>
        </template>
        <template #header>
            {{ name }}
        </template>
        <n-select v-model:value="statusRef" :options="options" @update:value="changeStatus" />
    </n-thing>
</template>

<script>
import { ref } from "vue"
import { PauseOutlined, CheckOutlined, CheckBoxOutlineBlankOutlined, ClearOutlined, QuestionMarkOutlined, TimelapseOutlined } from "@vicons/material"
import { useMessage } from "naive-ui"

export default {
    props: [
        'id',
        'status',
        'name',
    ],
    data() {
        return {
            statusRef: ref(this.status)
        }
    },
    components: {
        PauseOutlined,
        CheckOutlined,
        CheckBoxOutlineBlankOutlined,
        ClearOutlined,
        QuestionMarkOutlined,
        TimelapseOutlined
    },
    methods: {
        changeStatus(value) {
            window.$message.success("value of task " + this.id + " changed to " + JSON.stringify(value))
        }
    },
    setup() {
        window.$message = useMessage()
        return {
            options: [
                {
                    label: "Not Started",
                    value: 'NotStarted'
                },
                {
                    label: "Started",
                    value: 'Started'
                },
                {
                    label: "Paused",
                    value: 'Paused'
                },
                {
                    label: "Done",
                    value: 'Done'
                },
                {
                    label: "Aborted",
                    value: 'Aborted'
                }
            ]
        }
    }
}
</script>