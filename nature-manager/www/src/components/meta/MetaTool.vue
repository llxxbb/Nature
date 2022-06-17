<template>
  <el-input
    v-model="query"
    placeholder="find a domain"
    @input="onQueryChanged"
  />
</template>

<script>
export default {
  data() {
    return {
      query: "",
    };
  },
  methods: {
    onQueryChanged() {},
  },
};
</script>
