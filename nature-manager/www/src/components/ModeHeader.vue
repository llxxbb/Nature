<template>
  <el-row style="align-items: center">
    <el-dropdown>
      <el-row style="align-items: center">
        <img
          src="../assets/logo.png"
          width="32"
          height="32"
          style="padding-right: 10px"
        />
        <span> Nature <i inline-flex i="ep-arrow-down" /> </span>
      </el-row>
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item
            v-for="i in getModes()"
            :key="i"
            @click="setMode(i)"
          >
            <i v-show="selectedMode(i)" inline-flex i="ep-check" />{{
              getMenuTitle(i)
            }}
          </el-dropdown-item>

          <el-dropdown-item divided @click="changeTheme()">
            <span style="padding-right: 10px">theme</span>
            <i inline-flex i="dark:ep-moon ep-sunny" />
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
    <el-col :span="1"></el-col>
    <span> {{ initTitle() }}: </span>
    <el-row style="flex: 1; align-items: center"
      ><mode-tool :mode="mode"></mode-tool
    ></el-row>
  </el-row>
</template>

<script lang="ts">
import { NatureMode, getModeTitle } from "~/business/enum/mode";
import { toggleDark } from "~/composables";
import { range } from "~/util";

export default {
  data() {
    return {
      mode: NatureMode.Domain,
    };
  },
  methods: {
    getModes() {
      return range(0, 3);
    },
    getMenuTitle(mode: NatureMode) {
      return getModeTitle(mode);
    },
    initTitle(): String {
      return getModeTitle(this.mode);
    },
    selectedMode(mode: NatureMode) {
      if (this.mode == mode) return true;
    },
    setMode(mode: NatureMode) {
      this.mode = mode;
      this.$emit("modeChanged", mode);
    },
    changeTheme() {
      toggleDark();
    },
  },
  emits: ["modeChanged"],
};
</script>
