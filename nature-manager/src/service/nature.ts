import { DOMAIN_SEPARATOR, INSTANCE_RECENT_SIZE, INSTANCE_RELATED_AUTO, URL_BY_ID, URL_BY_KEY, URL_DOWNSTREAM, URL_META_GT, URL_RELATION_GT } from "@/config";
import { InstanceQueryCondition, Instance, FromInstance } from "@/domain/instance";
import { Meta } from "@/domain/meta";
import { D3Node } from "@/domain/node";
import { Relation } from "@/domain/relation";
import { metaDefined, relationDefined } from "@/testData/natureData";

const axios = require('axios').default;

let allMeta: Meta[];
let metaIdMax = 0;
let metaMap: Map<String, Meta> = new Map;
export class Nature {
    // data for relation mode
    async getRelation() {
        // // mock data for test-----------------------
        // // get meta list
        // allMeta = await getAllMetaMock();
        // // get relation list
        // let relationList = await getAllRelationMock();

        // real data-----------------------
        // get meta list
        allMeta = await getAllMeta();
        // get relation list
        let relationList = await getAllRelation();

        let metaList = allMeta.slice(); // copy it, and not modify the original
        allMeta.forEach(one => {
            one.initD3Node();
            metaMap.set(one.name, one);
        });
        // assembly relation tree-------------------
        // find max id
        metaList.forEach(one => { if (one.id > metaIdMax) metaIdMax = one.id })
        let idIncrease = metaIdMax;
        // process each relation
        relationList.forEach(r => {
            // find relation meta and it's index
            let from = findMeta(metaList, metaMap, r, (m, r) => m.name == r.from_meta);
            let to = findMeta(metaList, metaMap, r, (m, r) => m.name == r.to_meta, true);
            // if from == to set to unfound
            if (from.index != -2 && to.index == from.index) to.index = -1;
            // check to
            if (to.index == -1) to.meta = shadowMeta(to.meta, r.id, ++idIncrease)
            if (to.index == -2) setNodeId(to, ++idIncrease);
            // add relation
            to.meta.setRelation(r);
            (from.meta.d3node as D3Node).addChild(to.meta.d3node as D3Node);
            // remove "to" from metaList
            if (to.index > -1) metaList.splice(to.index, 1)
            // check from
            if (from.index == -2) {
                setNodeId(from, ++idIncrease);
                metaList.push(from.meta);
            }
        })
        // make tree
        let root = makeMetaRootNode(metaList);
        return root;
    }

    // data for domain mode
    async getDomain() {
        let unique = new Map<String, D3Node>();
        let root = makeMetaRootNode([]);
        let idSeq = 1;
        allMeta.forEach(one => {
            // init parent
            let path = DOMAIN_SEPARATOR;
            let parent = unique.get(path);
            if (!parent) {
                // only root has no parent
                parent = root;
                unique.set(path, parent);
            }
            // init child
            for (let index = 0; index < one.levels.length; index++) {
                // find parent
                const level = one.levels[index];
                path = path + level + DOMAIN_SEPARATOR;
                let child = unique.get(path);
                if (index < one.levels.length - 1) {
                    if (!child) {
                        child = makeParentDomainNode(one.levels, index, ++idSeq)
                        parent.addChild(child);
                        unique.set(path, child);
                    }
                    parent = child;
                } else {
                    if (!child) {
                        child = Object.assign(new D3Node, one.d3node);
                        child.setChildren(undefined);
                        child.setClassForSame("id" + ++idSeq);
                        child.id = ++idSeq;
                        parent.addChild(child);
                        unique.set(path, child);
                    }
                }
            }
        })
        return root;
    }

    async getInstance(condition: InstanceQueryCondition) {
        let ins = await this.getOneInstance(condition);
        if (!ins) return null;
        let rtn = Instance.toD3Node(ins as any as Instance);
        if (INSTANCE_RELATED_AUTO) return await this.fetchInstanceAuto(rtn);
        return rtn;
    };

    async getOneInstance(condition: InstanceQueryCondition) {
        let useVersion = true
        const meta = condition.other.meta;
        if (meta.isState() && condition.other.staVer == -1) useVersion = false;
        let instance;
        if (useVersion) {
            instance = await getInstanceById(condition.toFromInstance());
        } else {
            // get last version of `State-Meta`
            const data = {
                other: {
                    key_le: meta.instanceKey(condition.id, condition.other.para, Number.MAX_SAFE_INTEGER),
                }
            };
            let insResult = await getInstanceList(data)
            if (insResult.length > 0) instance = insResult[0]
        }
        if (!instance) alert("Sorry! not found");
        return instance;
    }

    private async fetchInstanceAuto(from: D3Node) {
        // upstream first
        let up = await this.getUpstream(from)
        while (!up.leftNavDone) {
            up = await this.getUpstream(up)
        }
        // fetch downstream
        await this.getDownRecursively(up);
        // return 
        return up;
    }

    private async getDownRecursively(up: D3Node) {
        await this.getDownstream(up);
        if (!up.hasChild()) return;
        const nodes = up.getChildren() as D3Node[];
        for (let index = 0; index < nodes.length; index++) {
            const one = nodes[index];
            await this.getDownRecursively(one)
        }
    }

    // fetch upstream
    async getUpstream(currentNode: D3Node) {
        currentNode.leftNavDone = true;
        let instance = currentNode.data.data as Instance
        let from = instance.data.from;
        if (!from) return currentNode;
        let rtnRaw = await getInstanceById(from);
        if (!rtnRaw) return currentNode;
        let rtn = Instance.toD3Node(rtnRaw);
        rtn.addChild(currentNode)
        return rtn;
    }

    // fill downstream
    async getDownstream(currentNode: D3Node) {
        // fetch data
        let res = await axios.post(URL_DOWNSTREAM, (currentNode.data.data as Instance).getKey());
        let rtnRaw: Instance[] = res.data.Ok
        rtnRaw.forEach(d => {
            let ins = rawToInstance(d)
            let one = Instance.toD3Node(ins)
            currentNode.addChild(one);
            one.leftNavDone = true;
        })
        currentNode.rightNavDone = true;
        const root = currentNode.findRoot();
        return root;
    }

    async getInstanceList(meta: string) {
        const data = {
            other: {
                meta,
                limit: INSTANCE_RECENT_SIZE
            }
        };
        return await getInstanceList(data);
    }

    async getStateList(condition: string) {
        const data = {
            other: {
                key_le: condition,
                limit: INSTANCE_RECENT_SIZE
            }
        };
        return await getInstanceList(data);
    }
}

function setNodeId(metaIndex: { meta: Meta; index: number; }, idIncrease: number) {
    const node = metaIndex.meta.d3node as D3Node;
    node.id = idIncrease;
    node.setClassForSame("id" + node.id);
}

async function getInstanceList(condition: any) {
    let res = await axios.post(URL_BY_KEY, condition);
    let rtn: Instance[] = [];
    if (!res) return rtn;
    if (res.data.Err) {
        console.warn(res.data.Err);
        return rtn;
    }
    res.data.Ok.forEach((d: any) => rtn.push(rawToInstance(d)));
    return rtn;
}
function rawToInstance(raw: any) {
    let ins: Instance = Object.assign(new Instance, raw);
    let meta = metaMap.get(ins.path.meta);
    if (!meta) meta = Meta.fromName(ins.path.meta);
    ins.meta = meta;
    return ins;
}

async function getInstanceById(condition: FromInstance) {
    let res = await axios.post(URL_BY_ID, condition);
    if (res.data.Err) {
        alert("remote err: " + JSON.stringify(res.data.Err));
        return null;
    }
    if (!res.data.Ok) {
        return null;
    }
    return rawToInstance(res.data.Ok);
}

function makeParentDomainNode(levels: string[], end: number, nodeId: number) {
    let rtn = new D3Node
    rtn.id = nodeId;
    rtn.setClassForSame("id" + nodeId);
    rtn.isShadow = true;
    rtn.name = levels[end]
    return rtn
}

function makeMetaRootNode(metaList: Meta[]) {
    const children = metaList.map(d => d.d3node);
    let root = new D3Node;
    root.setChildren(children as D3Node[]);
    root.name = "root";
    root.setClassForSame("idRoot");
    return root;
}

function shadowMeta(m: Meta, relationId: number, nodeId: number) {
    var rtn: Meta = Object.assign(new Meta, m);
    rtn.resetD3Node();
    const node = rtn.d3node;
    if (!node) throw new Error("imposable!");
    node.id = nodeId;
    node.isShadow = true;
    node.title = rtn.name + "|" + relationId;
    return rtn;
}

function findMeta(metaList: Meta[], metaMap: Map<String, Meta>, r: Relation, predicate: (m: Meta, r: Relation) => boolean, isTo = false) {
    let index = -1; // not found in `metaList`
    let name: string = isTo ? r.to_meta : r.from_meta;
    let meta = metaMap.get(name);
    if (!meta) {
        // meta not found in meta table but used in relation
        meta = Meta.fromName(name);
        metaMap.set(name, meta);
        var node = meta.d3node as D3Node;
        node.undefined = true
        index = -2; // not found and create new, so need not copy it to shadow;
    }
    if (meta?.meta_type == "N") 
        return { meta, index }
    // normal check
    let found = metaList.find((m, idx) => {
        if (predicate(m, r)) {
            index = idx;
            return true;
        }
    })
    if (found) return { meta: found, index };
    return { meta, index }
}

async function getAllRelationMock() {
    let rtn: Relation[] = [];
    relationDefined.forEach(one => {
        let m = Object.assign(new Relation, one);
        rtn.push(m);
    })
    return rtn;
}

async function getAllRelation() {
    return await getItems<Relation>(URL_RELATION_GT,
        item => {
            let rtn = Object.assign(new Relation, item);
            rtn.init();
            return rtn;
        }, items => {
            return items[items.length - 1].id;
        });
}

async function getAllMetaMock() {
    let meta: Meta[] = [];
    metaDefined.forEach(one => {
        let m = Object.assign(new Meta, one);
        m.init()
        meta.push(m);
    })
    return meta;
}

/**
 * @returns All Meta defined in database table
 * 
 */
async function getAllMeta() {
    return await getItems<Meta>(URL_META_GT,
        item => {
            let rtn = Object.assign(new Meta, item);
            rtn.init();
            return rtn;
        }, items => {
            return items[items.length - 1].id;
        });
}

async function getItems<T>(url: string, toT: (item: T) => T, idFun: (items: T[]) => number) {
    let id = 0;
    let size = 1000;
    let go = true;
    let all: T[] = [];
    while (go) {
        let rtnR = await axios.get(url + "/" + id + "/" + size);    // get one page
        let rtn = rtnR as { data: { Ok: T[] } }
        let dataReturned = rtn.data.Ok;
        dataReturned.forEach(i => {
            let myMeta = toT(i);
            all.push(myMeta)
        })
        if (dataReturned.length < size) break;
        id = idFun(dataReturned);   // remeber id for next page 
    }
    return all;
};
