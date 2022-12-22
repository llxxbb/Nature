<template>
  <tool-tip ref="toolTip" class="tip-meta" bgColor="#f6facc">
    <table class="table">
      <tbody>
        <tr v-show="meta?.name">
          <th scope="row">key:&ensp;</th>
          <td class="wrap">{{ meta?.name }}</td>
        </tr>
        <tr v-show="meta?.states" class="text-wrap">
          <th scope="row">states:&ensp;</th>
          <td class="wrap">{{ meta?.states }}</td>
        </tr>
        <tr v-show="meta?.fields">
          <th scope="row">fields:&ensp;</th>
          <td class="wrap">{{ meta?.fields }}</td>
        </tr>
        <tr v-show="meta?.config">
          <th scope="row">config:&ensp;</th>
          <td class="wrap">
            <pre><code>{{ JSON.stringify(meta?.configObj, null, "  ") }}</code></pre>
          </td>
        </tr>
        <tr v-show="meta?.description">
          <th scope="row">desc:&ensp;</th>
          <td class="wrap">{{ meta?.description }}</td>
        </tr>
        <tr v-show="meta?.create_time">
          <th scope="row">created:&ensp;</th>
          <td>{{ meta?.create_time }}</td>
        </tr>
      </tbody>
    </table>
  </tool-tip>
</template>
<script lang="ts">
// 当鼠标移动到节点时显示`Meta` 的信息
import { D3Node, DataType } from "@/domain/node";
import { Vue, Options } from "vue-class-component";
import ToolTip from "./ToolTip.vue";

@Options({
  components: { ToolTip },
  data() {
    return {
      meta: undefined,
    };
  },
  methods: {
    setPara(node: D3Node, x: number, y: number) {
      this.meta = undefined;
      if (!node) return;
      if (!node.data) return;
      if (node.data.dataType != DataType.META) return;
      this.meta = node.data.data;
      this.$refs.toolTip.show(x, y);
    },
    hide() {
      this.$refs.toolTip.hide();
    },
  },
})
export default class MetaToolTip extends Vue {}
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
