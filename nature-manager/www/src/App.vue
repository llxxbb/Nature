<template>
  <el-config-provider namespace="ep">
    <el-container>
      <el-header><base-header /></el-header>
      <el-main><domain-mode /></el-main>
    </el-container>
  </el-config-provider>
</template>

<script lang="ts">
export default {
  name: "App",
  components: {},
};
</script>
    BaseHeader