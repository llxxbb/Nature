<template>
  <el-config-provider namespace="ep">
    <BaseHeader />
    <div style="display: flex">
      <BaseSide />
      <div>
        <img alt="Vue logo" class="element-plus-logo" src="./assets/logo.png" />
        <IndexPage msg="Welcome to Your Vue.js App" />
      </div>
    </div>
  </el-config-provider>
</template>

<script lang="ts">
// import IndexPage from "./components/IndexPage.vue";

export default {
  name: "App",
  components: {
    // IndexPage,
  },
};
</script>