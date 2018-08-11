use db::*;
use fg_service::*;

pub type DeliveryService = DeliveryServiceImpl<DeliveryDaoImpl>;
pub type DispatchService = DispatchServiceImpl<DeliveryService>;
pub type RouteService = RouteServiceImpl<DeliveryService, OneStepFlowCacheImpl>;
pub type SequentialService = SequentialServiceImpl<DeliveryService, StoreService, InstanceServiceImpl>;
pub type PlanService = PlanServiceImpl<StorePlanDaoImpl>;
pub type ConvertService = ConvertServiceImpl<PlanService, DeliveryService, StoreService>;
pub type StoreService = StoreServiceImpl<DeliveryService, InstanceServiceImpl, InstanceDaoImpl, ThingDefineCacheImpl, DispatchService, RouteService>;
pub type ParallelService = ParallelServiceImpl<DeliveryService, StoreService>;
