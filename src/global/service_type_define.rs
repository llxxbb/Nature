use flow::*;
use nature_db::*;

pub type DeliveryService = DeliveryServiceImpl<DeliveryDaoImpl>;
pub type DispatchService = DispatchServiceImpl<DeliveryService>;
pub type RouteService = RouteServiceImpl<DeliveryService, OneStepFlowCacheImpl<OneStepFlowDaoImpl>>;
pub type SequentialService = SequentialServiceImpl<DeliveryService, StoreService, InstanceServiceImpl>;
pub type PlanService = PlanServiceImpl<StorePlanDaoImpl>;
pub type ConvertService = ConvertServiceImpl<PlanService, DeliveryService, StoreService, CallOutImpl<LocalExecutorImpl>, InstanceServiceImpl>;
pub type StoreService = StoreServiceImpl<DeliveryService, InstanceServiceImpl, InstanceDaoImpl, ThingDefineCacheImpl, DispatchService, RouteService>;
pub type ParallelService = ParallelServiceImpl<DeliveryService, StoreService>;
