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
          <el-dropdown-item @click="setMode(0)">
            <i v-if="selectedMode(0)" inline-flex i="ep-check" />Domain Mode
          </el-dropdown-item>
          <el-dropdown-item @click="setMode(1)">
            <i v-if="selectedMode(1)" inline-flex i="ep-check" />Relation Mode
          </el-dropdown-item>
          <el-dropdown-item @click="setMode(2)">
            <i v-if="selectedMode(2)" inline-flex i="ep-check" />Instance Mode
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

export default {
  data() {
    return {
      mode: NatureMode.Domain,
    };
  },
  methods: {
    initTitle(): String {
      return getModeTitle(this.mode);
    },
    selectedMode(mode: NatureMode) {
      if (this.mode == mode) return true;
    },
    setMode(mode: NatureMode) {
      this.mode = mode;
    },
    changeTheme() {
      toggleDark();
    },
  },
};
</script>
