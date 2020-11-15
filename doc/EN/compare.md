# Relationship with other systems

## Streaming Computing

Nature is also a streaming computing product. The difference from other products is:

| Other streaming computing frameworks                 | Nature                                      |
| ---------------------------------------------------- | ------------------------------------------- |
| Lean toward big data and scientific computing        | Lean toward business processing             |
| Low-level control (requires programmer intervention) | High-level control, such as decision-makers |

## Message system

Nature's processing mode is **data --map-->data --map-->data...**, which has certain similarities with the message system, and both can realize the decoupling of complex systems. But the difference between the two is also more obvious, as follows:

| Message System | Nature |
| ------------------------------------------------------------ | -------------------------------------------- |
| Serving technicians | More serving decision makers |
| Data is generally stored temporarily | Data is stored long-term |
| There is no inevitable correlation between data and data; | There is a close relationship between data and data, and data will form a chain |
| Two functional bodies are connected | Two goals connected |

## workflow

Nature's processing flow formed by concatenating goals is similar to workflow. The difference between the two is mainly reflected in:

| workflow | Nature |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| Tell us **how to do it (functionally driven)**, our goals may be **lost** with more iterations | Tell us **what to do**, focus on the really important things, and change management Get **easy** |
| Complex rule engine | Only one-to-one relationship, simple |
| The adjustment is the process, which is more frequent, and the goal is not easy to converge | The adjustment is the goal, concise and clear. |

## FaaS

The way Nature and Executor collaborate can actually be regarded as a form of FaaS (Function as a Service). The difference between the two is reflected in:

| FaaS | Nature |
| -------------------------- | -------------------------- |
| Function-oriented, partial. | Target-oriented, overall. |
| Use code to achieve functional weaving, complex. | Function knitting with configuration is simple. |

## Database

The essence of Nature is actually a data management product, trying to include all kinds of business data, but it does not have the storage capacity itself, and it needs to use specific database products to store it. Nature draws on the `relationship` technology of relational databases, and uses simple one-to-one to solve one-to-many, many-to-many and other complex relationships. The difference between the two is also very obvious, which is specifically reflected in:

| Database | Nature |
| ---------------------- | ---------------------------------- |
| Technology-oriented | Business-oriented |
| Relationship: powerful and flexible | Relationship: Only support one-to-one relationship with direction |
| Detailed modeling of business objects | Business object is a key-value |
| Mainly used for access and retrieval | Mainly used to drive business operations |

## ERP

The issue of decision integration has already been paid attention to in the industry, such as ERP. ERP was once popular, but it has hardly been interested recently. It's not that this concept is bad, but that ERP is not thorough enough. We do need to build a system around resources, but resource scheduling requires decision-making. However, ERP only puts decision-making in a unified view, but it does not substantially solve the problem of **decision-fixing**. Please refer to [Nature Architecture](help/architecture.md).

