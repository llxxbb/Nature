<template>
  <div
    v-show="show"
    class="my-menu"
    :style="{ top: para.top + 'px', left: para.left + 'px' }"
  >
    <ul class="list-group">
      <!-- navigator -->
      <li
        v-show="canNavigateInstance()"
        class="list-group-item item list-group-item-action"
      >
        <img src="../assets/nav.svg" />
        data-flow navigate
        <div class="container">
          <div class="row">
            <div class="col">
              <div v-show="canNavigateLeft()">
                <img
                  class="btn btn-outline-primary float-start"
                  src="../assets/nav-left.svg"
                  @click="leftInstance"
                />
              </div>
            </div>
            <div class="col">
              <div v-show="canNavigateRight()">
                <img
                  class="btn btn-outline-primary float-end"
                  src="../assets/nav-right.svg"
                  @click="rightInstance"
                />
              </div>
            </div>
          </div>
        </div>
      </li>
      <!-- query instance -->
      <li
        v-show="canQueryInstance()"
        class="list-group-item item list-group-item-action"
      >
        <img src="../assets/query-instance.svg" />
        query instance
        <div class="input-group">
          <div class="container">
            <div class="row">
              <div class="col">
                <input
                  v-model="instanceId"
                  type="text"
                  title="default 0"
                  class="form-control"
                  placeholder="id"
                />
              </div>
            </div>
            <div class="row">
              <div class="col">
                <input
                  v-model="instancePara"
                  type="text"
                  class="form-control"
                  placeholder="para"
                />
              </div>
            </div>
            <div class="row">
              <div class="col">
                <input
                  v-show="isState()"
                  v-model="instanceStaVer"
                  title="default -1 : for all version"
                  type="text"
                  class="form-control"
                  placeholder="status version"
                />
              </div>
            </div>
            <div class="row">
              <div class="col">
                <button
                  class="btn btn-outline-success"
                  title="show data flow of this `Instance`"
                  @click="query"
                >
                  Data Flow
                </button>
                <button
                  v-show="isState()"
                  class="btn btn-outline-success"
                  title="show recent version of this `Instance`"
                  @click="stateList"
                >
                  State List
                </button>
              </div>
            </div>
          </div>
        </div>
      </li>
      <!-- recent instance -->
      <li
        v-show="canQueryInstance()"
        class="list-group-item item list-group-item-action"
        @click="list"
      >
        <img src="../assets/list.svg" />
        query recent instances
      </li>
      <li
        v-show="false"
        class="list-group-item item list-group-item-action"
        @click="editNode"
      >
        <img src="../assets/node-edit.svg" />
        edit node
      </li>
      <li class="list-group-item item list-group-item-action" v-show="canAdd()">
        <img src="../assets/node-plus.svg" />
        add child node
        <input v-model="metaName" class="form-control" @keyup.enter="addNode" />
      </li>
      <li
        v-show="false"
        class="list-group-item item list-group-item-action"
        @click="deleteNode"
      >
        <img src="../assets/node-minus.svg" />
        delete node
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { Meta } from "@/domain/meta";
import { Instance, OtherInsCond } from "@/domain/instance";
import { D3Node, NatureData, DataType } from "@/domain/node";
import { Options, Vue } from "vue-class-component";
import { InstanceQueryCondition } from "@/domain/instance";
import { INSTANCE_RELATED_AUTO } from "@/config";

export class CMPara {
  left = 0;
  top = 0;
  node: D3Node = (undefined as any) as D3Node;
  id = "";
  para = "";
}

@Options({
  data() {
    return {
      instanceId: "",
      instancePara: "",
      instanceStaVer: "",
      metaName: "",
      show: false,
    };
  },
  props: {
    para: CMPara,
  },
  emits: [
    "dataFlow",
    "list",
    "editNode",
    "addNode",
    "deleteNode",
    "insLeft",
    "insRight",
    "stateList",
  ],
  methods: {
    showMenu(data: CMPara) {
      this.para = data;
      this.instanceId = data.id ? data.id : "";
      this.instancePara = data.para ? data.para : "";
      this.show = true;
    },
    hideMenu() {
      this.show = false;
    },
    leftInstance() {
      this.show = false;
      this.$emit("insLeft", this.para.node);
    },
    rightInstance() {
      this.show = false;
      this.$emit("insRight", this.para.node);
    },
    query() {
      if (!this.checkInput()) return;
      this.$emit('dataFlow', this.genCondition());
    },
    list() {
      this.show = false;
      this.$emit("list", this.getMeta().name);
    },
    stateList() {
      this.show = false;
      let ver =
        this.instanceStaVer === ""
          ? Number.MAX_SAFE_INTEGER
          : this.instanceStaVer;
      const condition = (this.getMeta() as Meta).instanceKey(
        this.instanceId,
        this.instancePara,
        ver
      );
      this.$emit("stateList", condition);
    },
    editNode() {
      this.show = false;
      this.$emit("editNode", this.para.node);
    },
    deleteNode() {
      this.show = false;
      this.$emit("deleteNode", this.para.node);
    },
    addNode() {
      this.show = false;
      this.$emit("addNode", {
        name: this.metaName,
        parent: this.para.node,
      });
      this.metaName = "";
    },
    canNavigateInstance() {
      return this.getInstance() && !INSTANCE_RELATED_AUTO;
    },
    canQueryInstance() {
      if (!this.getNatureData()) return false;
      return true;
    },
    canNavigateLeft() {
      if (!this.para.node) return false;
      return !this.para.node.leftNavDone;
    },
    canNavigateRight() {
      if (!this.para.node) return false;
      return !this.para.node.rightNavDone;
    },
    canAdd() {
      let nd = this.getNatureData();
      if (!nd) return true;
      if (nd.dataType == DataType.INSTANCE) return false;
      return true;
    },
    isState() {
      let nd = this.getNatureData();
      if (!nd) return false;
      if (nd.dataType == DataType.META) return nd.data.isState();
      else if (nd.dataType == DataType.INSTANCE)
        return (nd.data as Instance).meta.isState();
      return false;
    },
    genCondition() {
      this.show = false;
      // init and meta
      let meta: Meta;
      meta = this.getMeta();
      // init state version
      let staVer: number = 0;
      if (this.instanceStaVer.length > 0)
        staVer = new Number(this.instanceStaVer) as number;
      if (staVer == 0 && meta.isState()) staVer = -1;
      // query
      let cond = new InstanceQueryCondition();
      cond.id = this.instanceId;
      cond.other = new OtherInsCond();
      cond.other.meta = meta;
      cond.other.para = this.instancePara;
      cond.other.staVer = staVer;
      this.instanceId = "";
      this.instancePara = "";
      this.instanceStaVer = "";
      return cond;
    },
    checkInput() {
      if (this.instanceId === "" && this.instancePara === "") {
        alert("please input [id] and | or [para]");
        return false;
      }
      return true;
    },
  },
})
export default class NodeContextMenu extends Vue {
  para: CMPara = new CMPara();

  private getMeta(): Meta {
    let data = this.para.node.data as NatureData;
    if (data.dataType == DataType.META) {
      return data.data;
    } else {
      return (data.data as Instance).meta;
    }
  }
  private getInstance(): Instance | null {
    let nd = this.getNatureData();
    if (!nd) return null;
    if (nd.dataType == DataType.INSTANCE) return nd.data as Instance;
    return null;
  }
  private getNatureData(): NatureData | null {
    let node = this.para.node as D3Node;
    if (!node) return null;
    return node.data as NatureData;
  }
}
</script>
<style scoped lang="stylus">
.my-menu {
  z-index: 900;
  position: absolute;
}
</style>
