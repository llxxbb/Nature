import hash from "hash-it";
import { D3Node, DataType, NatureData } from "./node";
import { Meta } from "./meta";

export class InstanceQueryCondition {
    id: string = "0";
    other: OtherInsCond = new OtherInsCond();

    toFromInstance() {
        let rtn = new FromInstance;
        rtn.id = this.id;
        rtn.other.meta = this.other.meta.name;
        rtn.other.para = this.other.para;
        rtn.other.state_version = this.other.staVer;
        return rtn;
    }
    static fromInstance(ins: Instance) {
        let rtn = new InstanceQueryCondition;
        rtn.id = ins.id;
        rtn.other = new OtherInsCond();
        rtn.other.meta = ins.meta;
        rtn.other.para = ins.path.para;
        rtn.other.staVer = ins.path.state_version;
        return rtn;
    }
}

export class OtherInsCond{
    meta: Meta = new Meta;
    para: String = "";
    staVer: number = 0;
}

export class Modifier{
    meta: String="";
    para:String="";
    state_version:number=0;
}
export class Instance {
    id: string = "0";
    path: Modifier = new Modifier
    data: BizObject = new BizObject;
    create_time: Date = new Date;
    meta: Meta = undefined as any as Meta;

    getKey() {
        let para = this.path.para ? this.path.para : "";
        let ver = this.path.state_version ? this.path.state_version : 0
        return this.path.meta + "|" + this.id + "|" + para + "|" + ver
    }
    keyNoMeta() {
        let para = this.path.para ? this.path.para : "";
        let ver = this.path.state_version ? this.path.state_version : 0
        return this.id + "|" + para + "|" + ver
    }

    static toD3Node(cIns: Instance) {
        const nd = new NatureData;
        nd.dataType = DataType.INSTANCE;
        nd.data = cIns;
        let node = new D3Node;
        node.setState(cIns.meta.isState())
        node.name = cIns.meta.levels[cIns.meta.levels.length - 1];
        node.setClassForSame(cIns.id == "0" ? "id" + cIns.path.para : "id" + cIns.id);
        node.title = cIns.getKey();
        node.data = nd;
        node.id = hash(cIns.getKey());
        node.nodeBG = cIns.meta.d3node?.nodeBG as string;
        node.nodeType = cIns.meta.meta_type;
        return node;
    }
}

export class BizObject {
    content: string = "";
    context: Map<String, String> = new Map;
    sys_context: Map<String, String> = new Map;
    states: Set<String> = new Set;
    from?: FromInstance;
}

export class FromInstance {
    id: string = "";
    other: Modifier = new Modifier();
}