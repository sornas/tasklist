<template>
  <n-config-provider :theme="darkTheme">
    <n-space >
      <n-layout has-sider style="height: 100vh; display: flex; flex-direction: column;">
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
        <n-layout style="display: flex; flex: 1 0 auto;">
          <span>
            <router-view/>
          </span>
        </n-layout>
      </n-layout>
    </n-space>
    <n-global-style />
  </n-config-provider>
</template>

<script>
import { darkTheme } from 'naive-ui';
import { h, ref } from 'vue';
import { NIcon } from 'naive-ui'
import { BookOutline } from "@vicons/ionicons5"

function renderIcon(icon) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

const menuOptions = [
  {
    label: "Home",
    key: "home",
    icon: renderIcon(BookOutline)
  },
  {
    label: "Not Home",
    key: "nothome",
    icon: renderIcon(BookOutline)
  }
]

export default {
  setup() {
    return {
      darkTheme,
      activeKey: ref(null),
      collapsed: ref(null),
      menuOptions
    }
  }
}
</script>