// show how to use
<template>
  <teleport to="body">
    <div class="modal" tabindex="-1" v-if="ok" @keyup.esc="hide()">
      <div id="tip-border" class="modal-dialog" style="max-width: 900px">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">Tips</h5>
            <button
              ref="bClose"
              type="button"
              class="btn-close"
              data-bs-dismiss="modal"
              aria-label="Close"
              @click="hide"
              autofocus
              v-focus
            ></button>
          </div>
          <div class="modal-body">
            <slot></slot>
          </div>
          <div class="modal-footer">
            <button
              type="button"
              class="btn btn-secondary"
              data-bs-dismiss="modal"
              @click="hide"
            >
              Close
            </button>
          </div>
        </div>
      </div>
    </div>
    <!-- <div id="tip-border" class="modal" v-if="ok">
      <div>Tipos</div>
      <div id="setting-items"></div>
    </div> -->
  </teleport>
</template>
<script lang="ts">
import { Vue, Options } from "vue-class-component";
import * as cfg from "@/config";

@Options({
  data() {
    return {
      ok: false,
    };
  },
  methods: {
    show() {
      this.ok = true;
    },
    hide() {
      this.ok = false;
    },
  },
  directives: {
    focus: {
      mounted(el) {
        el.focus();
      },
    },
  },
})
export default class BaseModal extends Vue {}
</script> 

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="stylus">
.modal {
  display: block;
}
</style>
