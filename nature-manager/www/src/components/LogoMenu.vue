<template>
  <el-row>
    <el-dropdown>
      <div style="display: flex; flex: 1">
        <el-image
          style="width: 32px; height: 32px; padding: 10px"
          :src="logo"
          fit="fill"
        />
        <span class="padding">
          Nature <el-icon> <arrow-down /> </el-icon>
        </span>
      </div>
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item
            :icon="selectedMode(NatureMode.Domain)"
            @click="setMode(NatureMode.Domain)"
            >Domain Mode</el-dropdown-item
          >
          <el-dropdown-item
            :icon="selectedMode(NatureMode.Relation)"
            @click="setMode(NatureMode.Relation)"
            >Relation Mode</el-dropdown-item
          >
          <el-dropdown-item
            :icon="selectedMode(NatureMode.Instance)"
            @click="setMode(NatureMode.Instance)"
            >Instance Mode</el-dropdown-item
          >
        </el-dropdown-menu>
      </template>
    </el-dropdown>
    <el-col :span="1"></el-col>
    <span class="padding">
      {{ getModeTitle(mode) }}:
      <mode-tool></mode-tool>
    </span>
  </el-row>
</template>

<script lang="ts" setup>
import { toggleDark } from "~/composables";
import { ArrowDown, Check } from "@element-plus/icons-vue";
import logo from "~/assets/logo.png";
import { NatureMode, getModeTitle } from "~/business/enum/mode";
</script>
<script lang="ts">
export default {
  components: {
    ArrowDown,
    toggleDark,
    logo,
  },
  data() {
    return {
      mode: NatureMode.Domain,
    };
  },
  methods: {
    selectedMode(mode: NatureMode) {
      if (this.mode == mode) return Check;
    },
    setMode(mode: NatureMode) {
      this.mode = mode;
    },
  },
  props: {},
};
</script>

<style lang="scss" scoped>
.padding {
  padding-top: 18px;
}
</style>

