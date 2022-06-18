<template>
  <el-config-provider namespace="ep">
    <el-container>
      <el-header class="header"><logo-menu ref="logo"></logo-menu></el-header>
      <el-main><domain-mode ref="domain" /></el-main>
    </el-container>
  </el-config-provider>
</template>

<script lang="ts">
import { NatureMode } from "./business/enum/mode";
export default {
  name: "App",
  methods: {
    changeMode(event: Event, mode: NatureMode) {
      event.preventDefault();
      this.$refs.logo.setMode(mode);
    },
    KeyDown(event: KeyboardEvent) {
      if (event.ctrlKey) {
        switch (event.key) {
          case "d": {
            this.changeMode(event, NatureMode.Domain);
            return;
          }
          case "r": {
            this.changeMode(event, NatureMode.Relation);
            return;
          }
          case "i": {
            this.changeMode(event, NatureMode.Instance);
            return;
          }
        }
      }
    },
  },
  mounted() {
    window.addEventListener("keydown", this.KeyDown, true); //监听按键事件
  },
  beforeUnmount() {
    window.removeEventListener("keydown", this.KeyDown);
  },
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