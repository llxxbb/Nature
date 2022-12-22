// show how to use
<template>
  <base-modal ref="bm">
    <h3>node</h3>
    <table class="table table-striped table-hover">
      <thead>
        <tr>
          <th scope="col" class="col-2">aspect</th>
          <th scope="col" class="col-7">note</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row"><i class="bi bi-circle"></i></th>
          <td>node shape under relation mode</td>
        </tr>
        <tr>
          <th scope="row"><i class="bi bi-square"></i></th>
          <td>node shape under domain mode</td>
        </tr>
        <tr>
          <!-- <th scope="row"><i class="bi bi-app"></i></th> -->
          <th scope="row">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              version="1.1"
              width="16"
              height="16"
            >
              <rect
                rx="5"
                width="16"
                height="16"
                style="fill: none; stroke: black; stroke-width: 1.5"
              />
            </svg>
          </th>
          <td>node shape under instance mode</td>
        </tr>
        <tr>
          <th scope="row" :style="{ color: cfgDef.COLOR_TEXT_NORMAL }">
            node label
          </th>
          <td>for none state node</td>
        </tr>
        <tr>
          <th scope="row" :style="{ color: cfgDef.COLOR_TEXT_STATE }">
            node label
          </th>
          <td>for state node</td>
        </tr>
        <tr>
          <th scope="row">
            <svg width="48" height="16" xmlns="http://www.w3.org/2000/svg">
              <line
                x1="0"
                y1="8"
                x2="48"
                y2="8"
                :stroke="cfgDef.COLOR_STROKE_NORMAL"
                stroke-width="2"
              />
            </svg>
          </th>
          <td>
            stroke with solid line, is noumenon of the node, relations can
            follows it.
          </td>
        </tr>
        <tr>
          <th scope="row">
            <svg width="48" height="16" xmlns="http://www.w3.org/2000/svg">
              <line
                x1="0"
                y1="8"
                x2="48"
                y2="8"
                :stroke="cfgDef.COLOR_STROKE_NORMAL"
                stroke-width="2"
                stroke-dasharray="4,4"
              />
            </svg>
          </th>
          <td>
            stroke with Dash line, is shadow of the node, no relation can
            follows it.
          </td>
        </tr>
        <tr>
          <th scope="row">
            <svg width="24" height="16" xmlns="http://www.w3.org/2000/svg">
              <line
                x1="0"
                y1="8"
                x2="24"
                y2="8"
                :stroke="cfgDef.COLOR_STROKE_SAME"
                stroke-width="2"
              />
            </svg>
            &nbsp;
            <svg width="24" height="16" xmlns="http://www.w3.org/2000/svg">
              <line
                x1="0"
                y1="8"
                x2="24"
                y2="8"
                :stroke="cfgDef.COLOR_STROKE_SAME"
                stroke-width="2"
                stroke-dasharray="4,4"
              />
            </svg>
          </th>
          <td>for same node</td>
        </tr>
        <tr>
          <th scope="row">
            <svg
              width="16"
              height="16"
              version="1.1"
              :fill="cfgDef.COLOR_FILL_BUSINESS"
              xmlns="http://www.w3.org/2000/svg"
            >
              <circle cx="8" cy="8" r="8" />
            </svg>
          </th>
          <td>for MetaType.Business</td>
        </tr>
        <tr>
          <th scope="row">
            <svg
              width="16"
              height="16"
              version="1.1"
              :fill="cfgDef.COLOR_FILL_SYSTEM"
              xmlns="http://www.w3.org/2000/svg"
            >
              <circle cx="8" cy="8" r="8" />
            </svg>
          </th>
          <td>for MetaType.System</td>
        </tr>
        <tr>
          <th scope="row">
            <svg
              width="16"
              height="16"
              version="1.1"
              :fill="cfgDef.COLOR_FILL_MULTI"
              xmlns="http://www.w3.org/2000/svg"
            >
              <circle cx="8" cy="8" r="8" />
            </svg>
          </th>
          <td>for MetaType.Multi</td>
        </tr>
        <tr>
          <th scope="row">
            <svg
              width="16"
              height="16"
              version="1.1"
              :fill="cfgDef.COLOR_FILL_LOOP"
              xmlns="http://www.w3.org/2000/svg"
            >
              <circle cx="8" cy="8" r="8" />
            </svg>
          </th>
          <td>for MetaType.Loop</td>
        </tr>
        <tr>
          <th scope="row">
            <svg
              width="16"
              height="16"
              version="1.1"
              :fill="cfgDef.COLOR_FILL_DYNAMIC"
              xmlns="http://www.w3.org/2000/svg"
            >
              <circle cx="8" cy="8" r="8" />
            </svg>
          </th>
          <td>for MetaType.Dynamic</td>
        </tr>
        <tr>
          <th scope="row">
            <svg
              width="16"
              height="16"
              version="1.1"
              :fill="cfgDef.COLOR_FILL_NULL"
              xmlns="http://www.w3.org/2000/svg"
            >
              <circle cx="8" cy="8" r="8" />
            </svg>
          </th>
          <td>for MetaType.Null</td>
        </tr>
        <tr>
          <th scope="row">
            <svg
              width="16"
              height="16"
              version="1.1"
              :fill="cfgDef.COLOR_FILL_DISABLED"
              xmlns="http://www.w3.org/2000/svg"
            >
              <circle cx="8" cy="8" r="8" />
            </svg>
          </th>
          <td>node disabled</td>
        </tr>
        <tr>
          <th scope="row">
            <svg
              width="16"
              height="16"
              version="1.1"
              :fill="cfgDef.COLOR_FILL_UNDEFINED"
              xmlns="http://www.w3.org/2000/svg"
            >
              <circle cx="8" cy="8" r="8" />
            </svg>
          </th>
          <td>node undefined</td>
        </tr>
        <tr>
          <th scope="row">move node</th>
          <td>drag one to another</td>
        </tr>
      </tbody>
    </table>
    <h3>other</h3>
    <table class="table table-striped table-hover">
      <thead>
        <tr>
          <th scope="col" class="col-2">aspect</th>
          <th scope="col" class="col-7">note</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row">zoom</th>
          <td>scroll the mouse wheel on the blank</td>
        </tr>
        <tr>
          <th scope="row">translate</th>
          <td>click and drag on the blank</td>
        </tr>
      </tbody>
    </table>
  </base-modal>
</template>
<script lang="ts">
import { Vue, Options } from "vue-class-component";
import * as cfg from "@/config";
import BaseModal from "./ModalBase.vue";

@Options({
  components: {
    BaseModal,
  },
  data() {
    return {
      cfgDef: cfg,
    };
  },
  methods: {
    show() {
      this.$refs.bm.ok = true;
    },
  },
})
export default class Tips extends Vue {}
</script> 

