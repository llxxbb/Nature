export enum NatureMode {
    Domain,
    Relation,
    Instance
}

export function getModeTitle(mode: NatureMode) {
    switch (mode) {
        case NatureMode.Domain:
            return "Domain Mode";
        case NatureMode.Relation:
            return "Relation Mode";
        default:
            return "Instance Mode";
    }
}

