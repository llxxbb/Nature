<template>
  <div ref="treeContainer" style="flex: 1">
    <!-- <el-button @click="treeHeight">click</el-button> -->
    <el-tree-v2
      ref="tree"
      :data="createData(4, 30, 5)"
      :props="props"
      :height="treeHeight"
    />
  </div>
</template>

<script lang="ts">
import { ElTreeV2 } from "element-plus";
import { onMounted } from "@vue/runtime-core";
interface Tree {
  id: string;
  label: string;
  children?: Tree[];
}

export default {
  data() {
    return {
      props: {
        value: "id",
        label: "label",
        children: "children",
      },
      treeHeight: window.innerHeight - 52
    };
  },
  // setup() {
  //   onMounted(() => {
  //     // console.info(this.$refs.treeContainer.offsetHeight);
  //     // this.$refs.tree.height = this.$refs.treeContainer.offsetHeight;
  //     this.$refs.tree.height = window.innerHeight - 52;
  //   });
  // },
  methods: {
    getKey(prefix: string, id: number) {
      return `${prefix}-${id}`;
    },
    createData(
      maxDeep: number,
      maxChildren: number,
      minNodesNumber: number,
      deep = 1,
      key = "node"
    ): Tree[] {
      let id = 0;
      return Array.from({ length: minNodesNumber })
        .fill(deep)
        .map(() => {
          const childrenNumber =
            deep === maxDeep ? 0 : Math.round(Math.random() * maxChildren);
          const nodeKey = this.getKey(key, ++id);
          return {
            id: nodeKey,
            label: nodeKey,
            children: childrenNumber
              ? this.createData(
                  maxDeep,
                  maxChildren,
                  childrenNumber,
                  deep + 1,
                  nodeKey
                )
              : undefined,
          };
        });
    },
  },
};
</script>

<style lang="sass" scoped>
</style>