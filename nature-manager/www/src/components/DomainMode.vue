<template>
  <el-container>
    <meta-tree style="flex: 0 0 480px" />
    <el-space :fill="true">
      <meta-property style="width: 240px" />
      <relation-list style="flex: 1 0 240px" />
      <instance-list style="flex: 1 0 240px" />
      <instance-property style="flex: 1 0 240px" />
    </el-space>
    <el-container style="flex-wrap: wrap" :fill="'true'"> </el-container>
  </el-container>
</template>

<script>
export default {
  methods: {
    hello() {
      console.info("hello");
    },
  },
};
</script>>