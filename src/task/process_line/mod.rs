use dao::*;
use serde::Serialize;
use super::*;
use uuid::UuidBytes;

pub struct ProcessLine;

impl ProcessLine {
    /// born an instance which is the beginning of the changes.
    pub fn store(instance: Instance, root: Root) -> Result<UuidBytes> {
        let mut instance = instance;
        let uuid = InstanceImpl::verify(&mut instance, root)?;
        let task = StoreTask(instance);
        let carrier = Carrier::new(task)?;
        let _cid = CarrierDaoService::insert(&carrier)?;
        carrier.take_it_over()?;
        let sender = CHANNEL_ROUTE.sender.lock().unwrap().clone();
        thread::spawn(move || {
            sender.send(carrier).unwrap();
        });
        Ok(uuid)
    }

    fn route(carrier: Carrier<StoreTask>) {
        if let Ok(x) = MappingDaoService::get_relations(&carrier.data.0) {
            if let Some(y) = x {
                if let Ok(new_carrier) = Carrier::new(y) {
                    // insert new first carrier
                    if let Ok(_) = CarrierDaoService::insert(&new_carrier) {
                        // then delete old carrier
                        if let Ok(_) = CarrierDaoService::delete(&carrier.id) {
                            let sender = CHANNEL_DISPATCH.sender.lock().unwrap().clone();
                            thread::spawn(move || {
                                sender.send(new_carrier).unwrap();
                            });
                        };
                    };
                };
            };
        };
    }

    fn dispatch(_carrier: Carrier<RouteInfo>) {
        // TODO
//        let new_carrier = carrier.data.maps.iter().map(|m|{
//            let task = ConverterTask(carrier.instance,*m);
//            Carrier::new(task)
//        }).collect::<Carrier<ConverterTask>>();
//        for _map in carrier.data.maps {
//
//        }
    }
}

pub fn start_receive_threads() {
    start_thread(&CHANNEL_ROUTE.receiver,ProcessLine::route );
    start_thread(&CHANNEL_DISPATCH.receiver,ProcessLine::dispatch );
}

fn start_thread<T, F>(receiver: &'static Mutex<Receiver<Carrier<T>>>, f: F)
    where
        T: Serialize + Send,
        F: 'static + Fn(Carrier<T>) + Send
{
    thread::spawn(move || {
        let receiver = receiver.lock().unwrap();
        let mut iter = receiver.iter();
        while let Some(next) = iter.next() {
            f(next);
        }
    });
}



#[cfg(test)]
mod test_store;
