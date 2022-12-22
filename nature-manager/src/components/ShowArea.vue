<template>
  <instance-selector ref="insSelector" @flow="dataFlow"></instance-selector>
  <meta-tool-tip ref="tipMeta"></meta-tool-tip>
  <instance-tool-tip ref="tipIns"></instance-tool-tip>
  <relation-tool-tip ref="tipRelation"></relation-tool-tip>
  <node-context-menu
    ref="nodeMenu"
    @dataFlow="dataFlow"
    @list="instanceList"
    @addNode="addNode"
    @editNode="editNode"
    @deleteNode="deleteNode"
    @insLeft="navigateLeft"
    @insRight="navigateRight"
    @stateList="stateList"
  ></node-context-menu>
  <layer-context-menu
    ref="layerMenu"
    @changed="modeChanged"
  ></layer-context-menu>
  <svg
    id="showArea"
    ref="showArea"
    :class="bgMode"
    xmlns="http://www.w3.org/2000/svg"
  />
</template>

<script lang="ts">
import { Instance, InstanceQueryCondition } from "@/domain/instance";
import { Meta } from "@/domain/meta";
import { D3Node, DataType, Shape, TreePara } from "@/domain/node";
import { Nature } from "@/service/nature";
import { HierarchyPointNode } from "d3";
import { Options, Vue } from "vue-class-component";
import { D3Tree } from "../service/d3tree";
import InstanceSelector from "./ModalInstanceSelector.vue";
import LayerContextMenu, { LayoutMode } from "./ContextMenuLayer.vue";
import MetaToolTip from "./ToolTipMeta.vue";
import InstanceToolTip from "./ToolTipInstance.vue";
import NodeContextMenu from "./ContextMenuNode.vue";
import RelationToolTip from "./ToolTipRelation.vue";

@Options({
  components: {
    NodeContextMenu,
    LayerContextMenu,
    InstanceSelector,
    MetaToolTip,
    InstanceToolTip,
    RelationToolTip,
  },
  data() {
    return {
      relationData: null,
      domainData: null,
      tree: null,
      treePara: (null as unknown) as TreePara,
      nature: (null as unknown) as Nature,
      currentMode: LayoutMode.relation,
      bgMode: "mode_relation",
    };
  },
  computed: {
    center() {
      let area = this.$refs.showArea!;
      return [area.clientWidth, area.clientHeight];
    },
  },
  methods: {
    showNodeMenu(e: MouseEvent, d: D3Node) {
      let id, para;
      if (d.data && d.data.dataType == DataType.INSTANCE) {
        let ins = d.data.data as Instance;
        id = ins.id.toString();
        para = ins.path.para ? ins.path.para : "";
      }
      let data = {
        top: e.clientY,
        left: e.clientX,
        node: d,
        id,
        para,
      };
      this.$refs.nodeMenu.showMenu(data);
    },
    hideNodeMenu() {
      this.$refs.nodeMenu.hideMenu();
    },
    showLayoutMenu(e: MouseEvent) {
      let para = {
        top: e.clientY,
        left: e.clientX,
        mode: this.currentMode,
      };
      this.$refs.layerMenu.showMenu(para);
    },
    hideLayoutMenu() {
      this.$refs.layerMenu.hideMenu();
    },
    modeChanged(selected: LayoutMode) {
      if (selected == LayoutMode.domain) {
        this.setMode(LayoutMode.domain);
        this.treePara.data = this.domainData;
        this.treePara.shape = Shape.rect;
      } else {
        this.setMode(LayoutMode.relation);
        this.treePara.data = this.relationData;
        this.treePara.shape = Shape.circle;
      }
      this.tree.show(this.treePara);
    },
    setMode(mode: LayoutMode) {
      this.currentMode = mode;
      this.bgMode = "mode_" + LayoutMode[mode];
    },
    async dataFlow(e: InstanceQueryCondition) {
      let data = await this.nature.getInstance(e);
      if (!data) return;
      this.setMode(LayoutMode.instance);
      this.treePara.data = data;
      this.treePara.shape = Shape.rectR;
      this.tree.show(this.treePara);
    },
    async navigateLeft(d: D3Node) {
      let data = await this.nature.getUpstream(d);
      this.treePara.data = data;
      this.treePara.shape = Shape.rectR;
      this.tree.show(this.treePara);
    },
    async navigateRight(d: D3Node) {
      let data = await this.nature.getDownstream(d);
      this.treePara.data = data;
      this.treePara.shape = Shape.rectR;
      this.tree.show(this.treePara);
    },
    async instanceList(e: string) {
      if (!e || e === "") return;
      let data = await this.nature.getInstanceList(e);
      this.$refs.insSelector.show(data);
    },
    async stateList(e: string) {
      if (!e || e === "") return;
      let data = await this.nature.getStateList(e);
      this.$refs.insSelector.show(data);
    },
    addNode(e: { name: string; parent: D3Node }) {
      let newNode = new D3Node();
      newNode.name = e.name;
      e.parent.addChild(newNode);
      this.tree.show(this.treePara);
    },
    editNode(e: Meta) {
      console.log("editNode");
    },
    deleteNode(e: Meta) {
      console.log("deleteNode");
    },
    nodeMoved(
      source: HierarchyPointNode<D3Node>,
      target: HierarchyPointNode<D3Node>
    ) {
      source.data.moveTo(target.data);
      this.tree.show(this.treePara);
    },
    showNodeTip(e: MouseEvent, d: D3Node) {
      if (
        this.currentMode == LayoutMode.domain ||
        this.currentMode == LayoutMode.relation
      )
        this.$refs.tipMeta.setPara(d, e.clientX, e.clientY);
      else this.$refs.tipIns.show(d, e.clientX, e.clientY);
    },
    hideNodeTip() {
      this.$refs.tipMeta.hide();
      this.$refs.tipIns.hide();
    },
    showLinkTip(e: MouseEvent, d: D3Node) {
      if (this.currentMode == LayoutMode.relation)
        this.$refs.tipRelation.setPara(d, e.clientX, e.clientY);
    },
    hideLinkTip() {
      this.$refs.tipRelation.hide();
    },
  },
  async mounted() {
    this.nature = new Nature();
    this.relationData = await this.nature.getRelation();
    this.domainData = await this.nature.getDomain();

    this.treePara = {
      target: "#showArea",
      size: {
        width: this.center[0],
        height: this.center[1],
      },
      data: this.relationData,
      event: {
        showNodeMenu: this.showNodeMenu,
        hideNodeMenu: this.hideNodeMenu,
        showLayoutMenu: this.showLayoutMenu,
        hideLayoutMenu: this.hideLayoutMenu,
        nodeMoved: this.nodeMoved,
        navigateLeft: this.navigateLeft,
        navigateRight: this.navigateRight,
        showNodeTip: this.showNodeTip,
        hideNodeTip: this.hideNodeTip,
        showLinkTip: this.showLinkTip,
        hideLinkTip: this.hideLinkTip,
      },
      shape: Shape.circle,
    };
    this.tree = new D3Tree();
    this.tree.show(this.treePara);
  },
})
export default class ShowArea extends Vue {}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="stylus">
.mode_relation {
  background-color: #f1d5d5;
}

.mode_domain {
  background-color: #daf5f6;
}

.mode_instance {
  background-color: #eafce4;
}

#showArea {
  position: fixed;
  z-index: 1;
  top: 0px;
  bottom: 0px;
  left: 0px;
  right: 0px;
  width: 100%;
  height: 100%;
}

.same {
}

</style>
