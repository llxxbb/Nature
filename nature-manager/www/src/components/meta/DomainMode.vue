<template>
  <el-container id="domainMode" style="height:100%">
    <el-col :span="12">
      <meta-tree style="flex: 0 0 480px" />
    </el-col>
    <el-divider direction="vertical" />
    <el-col :span="12">
      <el-space
        direction="vertical"
        :size="small"
        style="width: 100%; align-items: inherit"
      >
        <meta-base-property style="height:50%"/>
        <meta-extend-property style="height:50%"/>
      </el-space>
    </el-col>
  </el-container>
</template>

<script>
export default {
  methods: {},
};
</script>>