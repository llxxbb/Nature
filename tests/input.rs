//#[cfg(test)]
//mod test {
//    use mockers::matchers::{ANY, check};
//
//    use crate::test_util::*;
//
//    use super::*;
//
//    #[test]
//    fn input_meta_type_must_be_business() {
//        // prepare
//        let mocks = MyMocks::new();
//        let store_svc = init_store_svc(&mocks);
//        // expect
//        mocks.s.expect(mocks.s_instance.verify_call(check(|x: &&mut Instance| x.meta.get_meta_type() == MetaType::Business))
//            .and_return(Err(NatureError::VerifyError("deliberate".to_string()))));
//        let mut instance = Instance::default();
//        instance.data.meta.set_meta_type(MetaType::Dynamic);
//        // run
//        let _rtn = store_svc.input(instance);
//    }
//
//    #[test]
//    fn meta_must_be_defined() {
//        // prepare
//        let mocks = MyMocks::new();
//        let store_svc = init_store_svc(&mocks);
//        // expect
//        mocks.s.expect(mocks.s_instance.verify_call(ANY)
//            .and_return(Err(NatureError::VerifyError("Meta must be defined".to_string()))));
//        let mut instance = Instance::default();
//        instance.data.meta.set_meta_type(MetaType::Dynamic);
//        // run
//        let rtn = store_svc.input(instance);
//        assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("Meta must be defined".to_string()))
//    }
//
//    #[test]
//    fn generate_store_tasks_for_error() {
//        // prepare
//        let mocks = MyMocks::new();
//        let store_svc = init_store_svc(&mocks);
//        // expect
//        let instance = Instance::new("123").unwrap();
//        mocks.s.expect(mocks.s_instance.verify_call(ANY)
//            .and_return(generate_id(&instance)));
//        mocks.s.expect(mocks.s_route.get_mission_call(check(|x: &&Instance| x.meta.get_full_key() == "/B/123"))
//            .and_return(Err(NatureError::VerifyError("get task error".to_string()))));
//        // run
//        let rtn = store_svc.input(instance);
//        assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("get task error".to_string()))
//    }
//
//    #[test]
//    fn insert_task_error() {
//        // prepare
//        let mocks = MyMocks::new();
//        let store_svc = init_store_svc(&mocks);
//        // expect
//        let instance = Instance::new("123").unwrap();
//        mocks.s.expect(mocks.s_instance.verify_call(ANY)
//            .and_return(generate_id(&instance)));
//        mocks.s.expect(mocks.s_route.get_mission_call(ANY)
//            .and_return(Ok(Some(vec![Mission::default()]))));
//        mocks.s.expect(mocks.d_task.insert_call(check(|x: &&RawTask| x.meta == "/B/123"))
//            .and_return(Err(NatureError::VerifyError("insert task error".to_string()))));
//        // run
//        let rtn = store_svc.input(instance);
//        assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("insert task error".to_string()))
//    }
//
//    #[test]
//    fn insert_instance_error() {
//        // prepare
//        let mocks = MyMocks::new();
//        let store_svc = init_store_svc(&mocks);
//        // expect
//        let instance = Instance::new("123").unwrap();
//        mocks.s.expect(mocks.s_instance.verify_call(ANY)
//            .and_return(generate_id(&instance)));
//        mocks.s.expect(mocks.s_route.get_mission_call(ANY)
//            .and_return(Ok(Some(vec![Mission::default()]))));
//        mocks.s.expect(mocks.d_task.insert_call(ANY)
//            .and_return(Ok(1)));
//        mocks.s.expect(mocks.d_instance.insert_call(check(|x: &&Instance| x.meta.get_full_key() == "/B/123"))
//            .and_return(Err(NatureError::VerifyError("insert instance error".to_string()))));
//        mocks.s.expect(mocks.d_task.raw_to_error_call(ANY, check(|x: &&RawTask| x.meta == "/B/123"))
//            .and_return(Ok(1)));
//        // run
//        let rtn = store_svc.input(instance);
//        assert_eq!(rtn.err().unwrap(), NatureError::VerifyError("insert instance error".to_string()))
//    }
//
//    #[test]
//    fn insert_instance_ok() {
//        // prepare
//        let mocks = MyMocks::new();
//        let store_svc = init_store_svc(&mocks);
//        // expect
//        let instance = Instance::new("123").unwrap();
//        mocks.s.expect(mocks.s_instance.verify_call(ANY)
//            .and_return(generate_id(&instance)));
//        mocks.s.expect(mocks.s_route.get_mission_call(ANY)
//            .and_return(Ok(Some(vec![Mission::default()]))));
//        mocks.s.expect(mocks.d_task.insert_call(ANY)
//            .and_return(Ok(1)));
//        mocks.s.expect(mocks.d_instance.insert_call(check(|x: &&Instance| x.meta.get_full_key() == "/B/123"))
//            .and_return(Ok(1)));
//        // run
//        let rtn = store_svc.input(instance);
//        assert_eq!(rtn.unwrap(), 47214786889964314131542807897190159873)
//    }
//
//    fn init_store_svc(mockers: &MyMocks) -> StoreServiceImpl {
//        StoreServiceImpl {
//            instance_dao: mockers.d_instance.clone(),
//            route: mockers.s_route.clone(),
//            task_svc: mockers.s_task.clone(),
//            task_dao: mockers.d_task.clone(),
//            svc_instance: mockers.s_instance.clone(),
//        }
//    }
//}