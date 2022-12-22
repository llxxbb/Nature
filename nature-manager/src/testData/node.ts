export var data: any = {
    name: "L1",
    children: [
        {
            name: "L2-2",
            children: [],
        },
        {
            name: "L2-1",
            _children: [
                { name: "L3-1", children: [] },
                { name: "L3-2", children: [] },
            ],
        },
        {
            name: "L2-3",
            children: [],
        },
    ],
}
export var data2: any = {
    name: "L1",
    children: [
        {
            name: "L2-2",
            children: [
                { name: "a", children: [] },
            ],
        },
        {
            name: "L2-1",
            _children: [
                { name: "L3-1", children: [] },
                { name: "L3-2", children: [] },
            ],
        },
        {
            name: "L2-3",
            children: [{ name: "a", children: [] },],
        },
    ],
}

export var data3: any = {
    name: "L1",
    children: [
        {
            name: "L2-2",
            children: [],
        },
        {
            name: "L2-1",
            _children: [
                {
                    name: "L3-1",
                    children: [
                        {
                            name: "L4-1",
                            _children: [
                                {
                                    name: "L5-1",
                                    children: [
                                        {
                                            name: "L6-1",
                                            _children: [
                                                { name: "L7-1", children: [] },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
                { name: "L3-2", children: [] },
            ],
        },
        {
            name: "L2-3",
            children: [],
        },
        {
            name: "L2-4",
            children: [],
        },
        {
            name: "L2-5",
            children: [],
        },
        {
            name: "L2-6",
            children: [],
        },
        {
            name: "L2-7",
            children: [],
        },
        {
            name: "L2-8",
            children: [],
        },
        {
            name: "L2-9",
            children: [],
        },
        {
            name: "L2-10",
            children: [],
        },
        {
            name: "L2-11",
            children: [],
        },
        {
            name: "L2-12",
            children: [],
        },
        {
            name: "L2-13",
            children: [],
        },
        {
            name: "L2-14",
            children: [],
        },
    ],
}