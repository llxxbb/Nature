<template>
  <el-config-provider namespace="ep">
    <el-container>
      <el-header class="header"><logo-menu /></el-header>
      <el-main><domain-mode /></el-main>
    </el-container>
  </el-config-provider>
</template>

<script lang="ts">
export default {
  name: "App",
};
</script>

<style lang="sass" scoped>
.header
  border-bottom-style: outset
  border-bottom-color: cornsilk

.ep-header
  margin: 10px
  height: 42px
</style>