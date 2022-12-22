<template>
  <div
    id="tool-win"
    v-show="visiable"
    :style="{
      top: top + 'px',
      left: left + 'px',
      width: width + 'px',
      'background-color': bgColor,
    }"
  >
    <slot></slot>
  </div>
</template>
<script lang="ts">
// 通用的信息浮窗
import { Vue, Options } from "vue-class-component";

@Options({
  props: {
    width: {
      type: String,
      default: "400",
    },
    bgColor: {
      type: String,
      default: "#fed2be",
    },
  },
  data() {
    return {
      left: 0,
      top: 0,
      visiable: false,
    };
  },
  methods: {
    show(x: number, y: number) {
      this.meta = undefined;
      this.left = x - this.width / 2;
      this.top = y + 10;
      this.visiable = true;
    },
    hide() {
      this.visiable = false;
    },
  },
})
export default class ToolTip extends Vue {}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="stylus">
#tool-win {
  position: fixed;
  z-index: 100;
  background-attachment: fixed;
  margin: auto;
  top: 100px;
  left: 0px;
  border: 2px solid #fefe90;
  border-radius: 15px;
}
</style>
