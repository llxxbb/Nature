<template>
  <tool-tip ref="toolTip" class="tip-meta" width="500">
    <table class="table">
      <tbody>
        <tr v-show="relation?.id">
          <th scope="row">id:&ensp;</th>
          <td class="wrap">{{ relation?.id }}</td>
        </tr>
        <tr v-show="relation?.from_meta" class="text-wrap">
          <th scope="row">from:&ensp;</th>
          <td class="wrap">{{ relation?.from_meta }}</td>
        </tr>
        <tr v-show="relation?.to_meta">
          <th scope="row">to:&ensp;</th>
          <td class="wrap">{{ relation?.to_meta }}</td>
        </tr>
        <tr v-show="relation?.settingObj">
          <th scope="row">settings:&ensp;</th>
          <td class="wrap">
            <pre><code>{{ JSON.stringify(relation?.settingObj, null, "  ") }}</code></pre>
          </td>
        </tr>
        <tr v-show="relation?.flag">
          <th scope="row">flag:&ensp;</th>
          <td class="wrap">{{ relation?.flag }}</td>
        </tr>
      </tbody>
    </table>
  </tool-tip>
</template>
<script lang="ts">
// 当鼠标移动到线上时显示`Relation` 的信息
import { Meta } from "@/domain/meta";
import { D3Node, DataType } from "@/domain/node";
import { Vue, Options } from "vue-class-component";
import ToolTip from "./ToolTip.vue";

@Options({
  components: { ToolTip },
  data() {
    return {
      relation: undefined,
    };
  },
  methods: {
    setPara(node: D3Node, x: number, y: number) {
      this.relation = undefined;
      if (!node) return;
      if (!node.data) return;
      if (node.data.dataType != DataType.META) return;
      this.relation = (node.data.data as Meta).getRelation();
      if (!this.relation) return;
      this.$refs.toolTip.show(x, y);
    },
    hide() {
      this.$refs.toolTip.hide();
    },
  },
})
export default class RelationToolTip extends Vue {}
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
