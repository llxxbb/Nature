<template>
  <tool-tip ref="toolTip" class="tip-meta" width="500" bgColor="#b1fcbc">
    <table class="table">
      <tbody>
        <tr v-show="ins?.getKey()">
          <th scope="row">key:&ensp;</th>
          <td class="wrap">{{ ins?.getKey() }}</td>
        </tr>
        <tr v-show="ins?.data.states" class="text-wrap">
          <th scope="row">states:&ensp;</th>
          <td class="wrap">{{ ins?.data.states }}</td>
        </tr>
        <tr v-show="ins?.data.state_version">
          <th scope="row">state_version:&ensp;</th>
          <td>{{ ins?.data.state_version }}</td>
        </tr>
        <tr v-show="ins?.data.context">
          <th scope="row">context:&ensp;</th>
          <td class="wrap">{{ ins?.data.context }}</td>
        </tr>
        <tr v-show="ins?.data.sys_context">
          <th scope="row">sys_context:&ensp;</th>
          <td class="wrap">{{ ins?.data.sys_context }}</td>
        </tr>
        <tr v-show="from">
          <th scope="row">from:&ensp;</th>
          <td class="wrap">
            <pre><code>{{ from }}</code></pre>
          </td>
        </tr>
        <tr v-show="ins?.data.content">
          <th scope="row">content:&ensp;</th>
          <td class="wrap">{{ ins?.data.content }}</td>
        </tr>
        <tr v-show="ins?.create_time">
          <th scope="row">created:&ensp;</th>
          <td>{{ time }}</td>
        </tr>
      </tbody>
    </table>
  </tool-tip>
</template>

<script lang="ts">
// 当鼠标移动到节点时显示`Instance` 的信息
import { Instance } from "@/domain/instance";
import { D3Node, DataType } from "@/domain/node";
import { Options, Vue } from "vue-class-component";
import ToolTip from "./ToolTip.vue";

@Options({
  components: { ToolTip },
  data() {
    return {
      ins: undefined,
      from: "",
      time: "",
    };
  },
  methods: {
    show(node: D3Node, x: number, y: number) {
      this.ins = undefined;
      if (!node) return;
      if (!node.data) return;
      if (node.data.dataType != DataType.INSTANCE) return;
      const ins = node.data.data as Instance;
      this.ins = ins;
      this.from = JSON.stringify(ins.data.from, null, "  ");
      this.time = new Date(ins.create_time).toLocaleString();
      this.$refs.toolTip.show(x, y);
    },
    hide() {
      this.$refs.toolTip.hide();
    },
  },
})
export default class InstanceToolTip extends Vue {}
</script>
<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="stylus">
.table td, .table th {
  padding: 0rem;
}

.wrap {
  word-break: break-all;
}
</style>
