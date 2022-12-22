<template>
  <base-modal ref="bm">
    <table class="table table-striped table-hover">
      <thead>
        <tr>
          <th scope="col">id | para | staVer</th>
          <th scope="col">status</th>
          <th scope="col">function</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in instances" :key="item.keyNoMeta()">
          <th scope="row">{{ item.keyNoMeta() }}</th>
          <td>{{ item.data.states }}</td>
          <td>
            <button
              class="btn btn-outline-success"
              title="show data flow of this `Instance`"
              @click="getFlow(item)"
            >
              data flow
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </base-modal>
</template>

<script lang="ts">
// used to select one from the version list of one `instance`.
import { Instance, InstanceQueryCondition } from "@/domain/instance";
import { Options, Vue } from "vue-class-component";
import BaseModal from "./ModalBase.vue";

@Options({
  components: {
    BaseModal,
  },
  data() {
    return {
      instances: [],
    };
  },
  methods: {
    show(data: Instance[]) {
      this.instances = data;
      this.$refs.bm.ok = true;
    },
    getFlow(ins: Instance) {
      this.$refs.bm.ok = false;
      this.$emit("flow", InstanceQueryCondition.fromInstance(ins));
    },
  },
  emits: ["flow"],
})
export default class InstanceSelector extends Vue {}
</script>