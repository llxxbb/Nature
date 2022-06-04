<template>
  <div>
    <el-row>
      <el-col :span="1">
        <div> </div>
        </el-col>
      <el-col :span="12"
        ><div class="grid-content bg-purple-light">lxb</div
      ></el-col>
    </el-row>
  </div>
</template>

<script lang='ts'>
export default {
  name: "IndexPage",
  props: {
    msg: String,
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.el-row {
  margin-bottom: 20px;
}
.el-row:last-child {
  margin-bottom: 0;
}
.el-col {
  border-radius: 4px;
}

.grid-content {
  border-radius: 4px;
  min-height: 36px;
}
</style>
