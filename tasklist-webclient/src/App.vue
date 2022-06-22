<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
      <n-layout v-if="windowWidth > 768" has-sider style="height: 100vh; display: flex; flex-direction: column;">
        <n-layout-sider 
          bordered
          collapse-mode="width"
          :collapsed-width="64"
          :width="240"
          :collapsed="collapsed"
          show-trigger
          @collapse="collapsed = true"
          @expand="collapsed = false"
        >
          <n-menu
            v-model:value="activeKey"
            :collapsed="collapsed"
            :collapsed-width="64"
            :collapsed-icon-size="22"
            :options="menuOptions"
          />
        </n-layout-sider>
        <n-layout>
          <span style="display: flex; flex-direction: column; padding: 2rem;">
            <router-view/>
          </span>
        </n-layout>
      </n-layout>
      <span v-else style="display: flex; flex-direction: column; padding: 2rem;">
        <router-view/>
      </span>
    </n-message-provider>
    <n-global-style />
  </n-config-provider>
</template>

<script>
import { darkTheme } from 'naive-ui';
import { h, ref } from 'vue';
import { NIcon } from 'naive-ui'
import { ArchiveOutlined, AssignmentOutlined, AutorenewOutlined, HomeOutlined, PermIdentityOutlined } from "@vicons/material"
import { RouterLink } from "vue-router"

function renderIcon(icon) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

const routines = [
  {
    label: "Example routine 1",
    key: "exr1",
  },
  {
    label: "Example routine 2",
    key: "exr2",
  }
]

const menuOptions = [
  {
    label: () => h(RouterLink, {
      to: {
        name: "home"
      }
    }, { default: () => "Home" }),
    key: "home",
    icon: renderIcon(HomeOutlined)
  },
  {
    label: "Routines",
    key: "routines",
    icon: renderIcon(AutorenewOutlined),
    children: routines
  },
  {
    label: "Unfinished tasks",
    key: "tasks",
    icon: renderIcon(AssignmentOutlined)
  },
  {
    label: () => h(RouterLink, {
      to: {
        name: "archive"
      }
    }, { default: () => "Archive" }),
    key: "archive",
    icon: renderIcon(ArchiveOutlined)
  },
  {
    label: "Profile",
    key: "routines",
    icon: renderIcon(PermIdentityOutlined)
  }
]

export default {
  created() {
    window.addEventListener("resize", this.changeWindowSize)
  },
  unmounted() {
    window.removeEventListener("resize", this.changeWindowSize);
  },
  methods: {
    changeWindowSize() {
      this.windowWidth = window.innerWidth;
    }
  },
  setup() {
    return {
      darkTheme,
      activeKey: ref(null),
      collapsed: ref(true),
      windowWidth: ref(window.innerWidth),
      menuOptions
    }
  }
}
</script>