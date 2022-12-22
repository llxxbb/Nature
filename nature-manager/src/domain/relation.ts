export class Relation {
    id = 0;
    from_meta = "";
    to_meta = "";
    settings = "";
    flag = 0;
    settingObj?: RelationSetting;

    init() {
        if (!this.settings || this.settings.length === 0) return;
        this.settingObj = JSON.parse(this.settings)
    }
}

export class RelationSetting {
    selector?: FlowSelector;
    executor?: Executor;
    convert_before?: Executor[];
    convert_after?: Executor[];
    use_upstream_id = false;
    target?: RelationTarget;
    delay = 0;
    delay_on_para = []
    id_bridge = false;
}

export class FlowSelector {
    state_all?: Set<String>;
    state_any?: Set<String>;
    state_none?: Set<String>;
    last_all?: Set<String>;
    last_any?: Set<String>;
    last_none?: Set<String>;
    context_all?: Set<String>;
    context_any?: Set<String>;
    context_none?: Set<String>;
    sys_context_all?: Set<String>;
    sys_context_any?: Set<String>;
    sys_context_none?: Set<String>;
}

export class Executor {
    protocol?: string
    url?: string
    settings?: string
}

export class RelationTarget {
    state_add?: string[]
    state_remove?: string[]
    append_para?: number[]
    dynamic_para?: string
}
