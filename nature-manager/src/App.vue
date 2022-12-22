<template>
  <Logo />
  <show-area />
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import Logo from "./components/Logo.vue";
import ShowArea from "./components/ShowArea.vue";

@Options({
  components: {
    Logo,
    ShowArea,
  },
})
export default class App extends Vue {}
</script>

