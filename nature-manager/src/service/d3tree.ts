import { COLOR_STROKE_NORMAL, NODE_SCALE, NODE_SIZE, COLOR_RELATION_NORMAL, COLOR_RELATION_DISABLED, COLOR_STROKE_SAME } from '@/config';
import { TreePara, D3Node, Position, Shape } from '@/domain/node';
import * as d3 from "d3";
import { BaseType, HierarchyPointNode } from "d3";

var ParaData: TreePara
var GForNode: d3.Selection<SVGGElement, unknown, HTMLElement, any>;
var GForLink: d3.Selection<SVGGElement, unknown, HTMLElement, any>;
var CurrentNode: HierarchyPointNode<unknown>;
var SVG: d3.Selection<d3.BaseType, unknown, HTMLElement, any>;
var TargetToDrop: d3.HierarchyPointNode<unknown> | null;
var DragStart: boolean = false;
var SelectNodeX = 0;
var SelectNodeY = 0;

// make the pointer of mouser out fo the node when drag
var Offset = 0.04;
export class D3Tree {
    show(para: TreePara) {
        ParaData = para;
        SVG = initSvg(para);

        let g = initG(SVG, para);

        let nodes = appendProperty(para);

        // draw line first! otherwise you will see the line goes into the node
        GForLink = g.append("g")
            .attr("fill", "none")
            .attr("stroke-opacity", 0.4)
            .attr("stroke-width", 0.01 * NODE_SCALE)
        drawLinks(GForLink, nodes);

        // set font property to node
        GForNode = g.append("g")
            .attr("font-family", "sans-serif")
            .attr("font-size", 0.04 * NODE_SCALE)
            .attr("stroke-linejoin", "round")
            .attr("stroke-width", 0.006 * NODE_SCALE);
        drawNode(GForNode, nodes);

    }
}

function initSvg(para: TreePara) {
    let svg = d3.select(para.target);
    // add viewBox for pan and zoom
    svg.attr("viewBox", `0, 0, 1000, 1000`)
        .on("click", () => hideContextMenu(para))
        .on("contextmenu", showLayerContextMenu)
    // clear exists node
    svg.select("g").remove();
    return svg;
}

function hideContextMenu(para: TreePara) {
    if (para.event && para.event.hideNodeMenu)
        para.event.hideNodeMenu();
    if (para.event && para.event.hideLayoutMenu)
        para.event.hideLayoutMenu();
}

function initG(svg: d3.Selection<d3.BaseType, unknown, HTMLElement, any>, para: TreePara) {
    let g = svg.append("g");
    g.attr("transform", `translate(0,${0.5 * NODE_SCALE})`);
    g.attr("id", "All-G");

    svg.call(
        d3.zoom<any, any>()
            .extent([[0, 0], [para.size.width, para.size.height]])
            .scaleExtent([0.1, 10])
            .on("zoom", item => {
                hideContextMenu(para)
                g.attr("transform", item.transform);
            }));

    return g;
}

function appendProperty(para: TreePara) {
    // append depth, height, children, parent properties to datum
    let hierarchy = d3.hierarchy(para.data);
    hierarchy.sort((a, b) => (a.data.name < b.data.name ? -1 : 1));
    let tree = d3.tree();
    tree.nodeSize([0.00007 * NODE_SIZE, 0.0004 * NODE_SIZE])
    // append x, y properties to datum
    let nodes = tree(hierarchy);
    nodes.each(n => {
        n.x = n.x * NODE_SCALE
        n.y = n.y * NODE_SCALE
    })
    return nodes;
}

function drawLinks(g: d3.Selection<SVGGElement, unknown, HTMLElement, any>, nodes: d3.HierarchyPointNode<unknown>) {
    let linkFn = d3.linkHorizontal<any, any>()
        .x((d) => d.y)
        .y((d) => d.x);

    g.selectAll("path")
        .data(nodes.links())
        .join("path")
        .attr("d", linkFn)
        .attr("stroke", d => (d.target.data as D3Node).relationDisabled ? COLOR_RELATION_DISABLED : COLOR_RELATION_NORMAL)
        .on("mouseover", (e, d) => {
            if (ParaData.event?.showLinkTip) ParaData.event.showLinkTip(e, d.target.data as D3Node)
        })
        .on("mouseout", (_e, _d) => {
            if (ParaData.event?.hideLinkTip) ParaData.event.hideLinkTip()
        })
}

function drawNode(
    upperG: d3.Selection<SVGGElement, unknown, HTMLElement, any>,
    nodes: d3.HierarchyPointNode<unknown>,
) {
    upperG.selectAll("g")
        .data(nodes.descendants(), d => {
            let item = d as HierarchyPointNode<D3Node>
            return item.data.name
        })
        .join(
            newNodes,
            nodeChanged
        )
}

function newNodes(enterData: d3.Selection<d3.EnterElement, d3.HierarchyPointNode<unknown>, SVGGElement, unknown>) {
    var enter = enterData.append("g")
        .attr("id", d => {
            const data = (d as unknown as HierarchyPointNode<D3Node>);
            return "g" + data.data.id
        })

    dragEvent(enter);

    // draw node
    const nodeItem = enter.append(getShape());
    shapePropertySet(nodeItem)
    nodeItem.attr("stroke", COLOR_STROKE_NORMAL)
        .attr("stroke-width", `${0.005 * NODE_SCALE}`)
        .attr("stroke-dasharray", d => {
            if ((d.data as D3Node).isShadow)
                return `${0.005 * NODE_SCALE}, ${0.01 * NODE_SCALE}`
            else return `100,0`
        })
        .attr("fill", d => {
            const node = d.data as any as D3Node;
            return node.nodeBG;
        })
        .attr("id", d => `c${(d.data as D3Node).id}`)
        // used to select same meta
        .attr("class", d => ((d.data) as any as D3Node).getClassForSame())

    appendText(enter)

    // add folder icon
    let folder = enter.filter(d => (d.data as D3Node).hasChild());
    addFolderIcon(folder);
    enter.append("text")
        .text((d) => (d.data as D3Node).nodeType)
        .attr("y", `${0.015 * NODE_SCALE}`)
        // distance from text to node
        .attr("x", `${-0.014 * NODE_SCALE}`)
        .attr("font-weight", `${0.6 * NODE_SCALE}`)
        .attr("opacity", 0.4)

    enter.attr("transform", (d: Position) => `translate(${d.y},${d.x})`);

    const nodeEvent = enter.append(getShape());
    // node event
    shapePropertySet(nodeEvent)
    nodeEvent.attr("opacity", "0")
        .on("click", (e, d) => {
            let hasChild = (d as HierarchyPointNode<D3Node>).data.hasChild();
            if (hasChild) toggle(e, d)
            changeCurrent(e, d)
        })
        .on("contextmenu", showNodeContextMenu)
        .on("mouseover", mouseOver)
        .on("mouseout", (_e, d) => {
            TargetToDrop = null
            // remove same
            d3.selectAll(".same")
                .attr("class", d => {
                    return (d as HierarchyPointNode<D3Node>).data.getClassForSame()
                })
                .attr("stroke", COLOR_STROKE_NORMAL)
            if (ParaData.event?.hideNodeTip) ParaData.event.hideNodeTip()
        })
    return enter;
}

function mouseOver(e: any, d: d3.HierarchyPointNode<unknown>) {
    const d3node = d.data as D3Node;
    if (d != CurrentNode && DragStart) TargetToDrop = d;
    // show same
    const same = d3.selectAll("." + d3node.getClassForSame());
    same.attr("stroke", COLOR_STROKE_SAME);
    same.attr("class", d => {
        return "same " + (d as HierarchyPointNode<D3Node>).data.getClassForSame();
    });
    if (ParaData.event?.showNodeTip) ParaData.event.showNodeTip(e, d.data as D3Node)
}

function getShape(): string {
    if (ParaData.shape == Shape.circle) return "circle";
    else if (ParaData.shape == Shape.rect) return "rect";
    else return "rect";
}

function shapePropertySet(nodeEvent: d3.Selection<d3.BaseType, d3.HierarchyPointNode<unknown>, SVGGElement, unknown>) {
    if (ParaData.shape == Shape.circle) {
        nodeEvent.attr("r", 0.03 * NODE_SCALE);
    } else if (ParaData.shape == Shape.rect) {
        nodeEvent.attr("width", 0.06 * NODE_SCALE);
        nodeEvent.attr("height", 0.06 * NODE_SCALE);
        nodeEvent.attr("x", -0.03 * NODE_SCALE);
        nodeEvent.attr("y", -0.03 * NODE_SCALE);
    } else {
        nodeEvent.attr("width", 0.06 * NODE_SCALE);
        nodeEvent.attr("height", 0.06 * NODE_SCALE);
        nodeEvent.attr("x", -0.03 * NODE_SCALE);
        nodeEvent.attr("y", -0.03 * NODE_SCALE);
        nodeEvent.attr("rx", 0.015 * NODE_SCALE);
        nodeEvent.attr("ry", 0.015 * NODE_SCALE);
    }
}

function dragEvent(enter: d3.Selection<SVGGElement, d3.HierarchyPointNode<unknown>, SVGGElement, unknown>) {
    var drag = d3.drag()
        .on("start", (e, d) => {
            hideContextMenu(ParaData);
            changeCurrent(e, d as HierarchyPointNode<any>)
            const one = (d as HierarchyPointNode<D3Node>);
            // the root node can't be moved
            if (one.parent) {
                DragStart = true
                SelectNodeX = one.x;
                SelectNodeY = one.y;
            }
        })
        .on("drag", (e, d) => {
            if (!DragStart) return
            const one = (d as unknown as HierarchyPointNode<D3Node>);
            const selected = d3.select(`#g${(one).data.id}`);
            // make sure that can put it to target
            selected.attr("transform", () => {
                let x = e.x - one.x + one.y + Offset * NODE_SCALE;
                let y = e.y - one.y + one.x + Offset * NODE_SCALE;
                return `translate(${x},${y})`;
            });
        })
        .on("end", (e, d) => {
            if (!DragStart) return;
            DragStart = false
            // Instance-Mode can not move
            if (!TargetToDrop || ParaData.shape == Shape.rectR) {
                // back transform
                const one = (d as unknown as HierarchyPointNode<D3Node>);
                const selected = d3.select(`#g${(one).data.id}`);
                selected.attr("transform", () => `translate(${SelectNodeY},${SelectNodeX})`);
                return;
            }
            // close tip
            if (ParaData.event?.hideNodeTip) ParaData.event.hideNodeTip()
            if (ParaData.event?.hideLinkTip) ParaData.event.hideLinkTip()
            // do move
            let dragged = d as HierarchyPointNode<D3Node>;
            let target = TargetToDrop
            TargetToDrop = null
            if (ParaData.event && ParaData.event.nodeMoved)
                ParaData.event?.nodeMoved(dragged, target as HierarchyPointNode<D3Node>);
        });

    drag(enter as unknown as d3.Selection<Element, unknown, any, any>);
}

function nodeChanged(updateData: d3.Selection<d3.BaseType, d3.HierarchyPointNode<unknown>, SVGGElement, unknown>): d3.Selection<d3.BaseType, d3.HierarchyPointNode<unknown>, SVGGElement, unknown> | undefined {
    const text = updateData.selectAll(".side");
    setTextPosition(text);

    updateData.attr("transform", (d: Position) => `translate(${d.y},${d.x})`);

    return updateData
}

function appendText<T extends BaseType>(selected: d3.Selection<T, d3.HierarchyPointNode<unknown>, SVGGElement, unknown>) {
    selected.append("text")
        .attr("y", `${0.015 * NODE_SCALE}`)
        // distance from text to node
        .attr("x", (d) => (d.children ? `${-0.04 * NODE_SCALE}` : `${0.04 * NODE_SCALE}`))
        .attr("text-anchor", (d) => (d.children ? "end" : "start"))
        // used to select all text beside node
        .attr("class", "side")
        // set status meta
        .attr("fill", d => (d.data as any as D3Node).getTextColor())
        .attr("opacity", (d) => ((d.data as any as D3Node).isShadow ? 0.4 : 1))
        .text(d => (d.data as D3Node).name)
        .clone(true)
        // stroke no text inner
        .lower()
        .attr("stroke", "white");
}

function setTextPosition(text: d3.Selection<d3.BaseType, unknown, d3.BaseType, d3.HierarchyPointNode<unknown>>) {
    text.attr("x", d => {
        var opened = (d as HierarchyPointNode<D3Node>).data.openedCheck();
        return opened ? -0.04 * NODE_SCALE : 0.04 * NODE_SCALE;
    })
        .attr("text-anchor", d => {
            var opened = (d as HierarchyPointNode<D3Node>).data.openedCheck();
            return opened ? "end" : "start";
        });
}

function addFolderIcon<T extends BaseType>(folder: d3.Selection<T, d3.HierarchyPointNode<unknown>, SVGGElement, unknown>) {
    return folder.append("image")
        .attr("x", `${-0.025 * NODE_SCALE}`)
        .attr("y", `${-0.025 * NODE_SCALE}`)
        .attr("width", `${0.05 * NODE_SCALE}`)
        .attr("height", `${0.05 * NODE_SCALE}`)
        .attr("href", d => {
            const node = d.data as D3Node;
            if (node.hasChild()) {
                if (node.openedCheck()) return require("../assets/caret-right-fill.svg");
                else return require("../assets/caret-down-fill.svg");
            }
            return null;
        })
        .attr("id", d => "i" + (d.data as D3Node).id)
}

function changeCurrent(_e: MouseEvent, d: HierarchyPointNode<unknown>) {
    if (CurrentNode) {
        let old = d3.select("#c" + (CurrentNode.data as D3Node).id)
        old.attr("stroke", "#079702");
    }
    CurrentNode = d
    let nNode = "#c" + (CurrentNode.data as D3Node).id;
    let n = d3.select(nNode)
    n.attr("stroke", "#8f3200");
}

function showNodeContextMenu(e: any, node: d3.HierarchyPointNode<unknown> | unknown) {
    if (!ParaData.event) return;
    if (ParaData.event.hideLayoutMenu)
        ParaData.event.hideLayoutMenu();
    let d = node as HierarchyPointNode<any>
    changeCurrent(e, d);
    if (ParaData.event && ParaData.event.showNodeMenu)
        ParaData.event.showNodeMenu(e, d.data);
    e.stopPropagation();
    e.preventDefault();
}

function showLayerContextMenu(e: MouseEvent) {
    if (!ParaData.event) return;
    if (ParaData.event.hideNodeMenu)
        ParaData.event.hideNodeMenu();
    if (ParaData.event.showLayoutMenu)
        ParaData.event.showLayoutMenu(e);
    e.preventDefault();
}

function update(para: TreePara) {
    let nodes = appendProperty(para);

    // draw line first! otherwise you will see the line goes into the node
    drawLinks(GForLink, nodes);
    drawNode(GForNode, nodes);
}

// Toggle folder.
function toggle(e: MouseEvent, node: HierarchyPointNode<unknown> | unknown) {
    let d = node as HierarchyPointNode<D3Node>
    let data: D3Node = d.data
    let one = d3.select(`#i${data.id}`)
    data.toggle(() => one.attr("href", `${require("../assets/caret-down-fill.svg")}`),
        () => one.attr("href", `${require("../assets/caret-right-fill.svg")}`))
    update(ParaData)
}
