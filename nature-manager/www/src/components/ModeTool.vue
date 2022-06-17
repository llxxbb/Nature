<template>
  <meta-tool v-if="isDomain()"></meta-tool>
</template>

<script>
import { NatureMode } from "~/business/enum/mode.ts";

export default {
  props: {
    mode: NatureMode,
  },
  methods: {
    isDomain() {
      return this.mode == NatureMode.Domain;
    },
  },
};
</script>
