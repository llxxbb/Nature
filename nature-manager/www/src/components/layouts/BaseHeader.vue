<script lang="ts" setup>
import { toggleDark } from "~/composables";
</script>

<template>
  <el-menu class="el-menu-demo" mode="horizontal">
    <logo-icon />
    <el-menu-item index="1">Domain Mode</el-menu-item>
    <el-menu-item index="2">Relation Mode</el-menu-item>
    <el-menu-item index="3">Instance Mode</el-menu-item>
    <el-menu-item @click="toggleDark()">
      <button
        class="border-none w-full bg-transparent cursor-pointer"
        style="height: var(--ep-menu-item-height)"
      >
        <i inline-flex i="dark:ep-moon ep-sunny" />
      </button>
    </el-menu-item>
  </el-menu>
</template>

<script lang="ts">

import LogoIcon from '~/components/LogoIcon.vue'

export default {
  components:{
    LogoIcon
  }
};
</script>



