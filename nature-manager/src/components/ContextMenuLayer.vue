<template>
  <div
    v-show="show"
    class="my-menu"
    :style="{ top: para.top + 'px', left: para.left + 'px' }"
  >
    <ul class="list-group">
      <li
        v-show="modeDomain()"
        class="list-group-item item list-group-item-action"
        @click="click(1)"
      >
        <img src="../assets/domain.svg" />
        Domain Mode
      </li>
      <li
        v-show="modeRelation()"
        class="list-group-item item list-group-item-action"
        @click="click(2)"
      >
        <img src="../assets/relation.svg" />
        Relation Mode
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

export enum LayoutMode {
  domain,
  relation,
  instance,
}

export class LMPara {
  left = 0;
  top = 0;
  mode: LayoutMode = LayoutMode.relation;
}

@Options({
  data() {
    return {
      show: false,
    };
  },
  props: {
    para: LMPara,
  },
  emits: ["changed"],
  methods: {
    showMenu(para: LMPara) {
      this.para = para;
      this.show = true;
    },
    hideMenu() {
      this.show = false;
    },
    modeDomain() {
      return (
        this.para.mode == LayoutMode.relation ||
        this.para.mode == LayoutMode.instance
      );
    },
    modeRelation() {
      return (
        this.para.mode == LayoutMode.domain ||
        this.para.mode == LayoutMode.instance
      );
    },
    click(selected: number) {
      this.show = false;
      let rtn = selected == 1 ? LayoutMode.domain : LayoutMode.relation;
      this.$emit("changed", rtn);
    },
  },
})
export default class LayerContextMenu extends Vue {
  para = new LMPara();
}
</script>
<style scoped lang="stylus">
.my-menu {
  z-index: 900;
  position: absolute;
}
</style>
