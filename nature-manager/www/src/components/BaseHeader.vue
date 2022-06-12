<template>
  <el-dropdown>
    <div style="display: flex; flex: 1">
      <el-image
        style="width: 32px; height: 32px; padding: 10px"
        :src="logo"
        fit="fill"
      />
      <span
        >Nature
        <el-icon class="el-icon--right" style="margin-top: 18px">
          <arrow-down />
        </el-icon>
      </span>
    </div>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item>Domain Mode</el-dropdown-item>
        <el-dropdown-item>Relation Mode</el-dropdown-item>
        <el-dropdown-item>Instance Mode</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script lang="ts" setup>
import { toggleDark } from "~/composables";
import { ArrowDown } from "@element-plus/icons-vue";
import logo from "~/assets/logo.png";
</script>
<style scoped>
.example-showcase .el-dropdown-link {
  cursor: pointer;
  color: var(--el-color-primary);
  display: flex;
  align-items: center;
}
</style>

