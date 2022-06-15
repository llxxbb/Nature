<template></template>

<script>
import { NatureMode, getModeTitle } from "~/business/enum/mode.ts";

export default {
  props: {
    mode: NatureMode,
  },
  methods: {
    getMode() {
      return getModeTitle(this.mode);
    },
  },
};
</script>
