<template>
  <el-col :span="8">
    <el-input
      v-model="query"
      placeholder="find a Meta definition"
      @input="onQueryChanged"
    />
  </el-col>
  <el-icon> <search /> </el-icon>
</template>

<script lang="ts">
import { Search } from "@element-plus/icons-vue";

export default {
  components: {
    Search,
  },
  data() {
    return {
      query: "",
    };
  },
  methods: {
    onQueryChanged() {},
  },
};
</script>
