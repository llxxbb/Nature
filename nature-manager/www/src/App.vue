<template>
  <el-config-provider namespace="ep">
    <el-container>
      <el-header><logo-menu /></el-header>
      <el-main><domain-mode /></el-main>
    </el-container>
  </el-config-provider>
</template>

<script lang="ts">
export default {
  name: "App",
};
</script>
    BaseHeader