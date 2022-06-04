<template>
  <div>
    <IndexPage msg="Welcome to Your Vue.js App" />
  </div>
</template>

<script lang="ts">
import IndexPage from "./components/IndexPage.vue";

export default {
  name: "App",
  components: {
    IndexPage,
  },
};
</script>