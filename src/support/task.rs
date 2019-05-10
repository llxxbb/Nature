use super::*;

pub trait TaskServiceTrait {
    fn create_and_finish_carrier(&self, old: &RawTask, new: &mut RawTask) -> Result<usize>;

    fn create_batch_and_finish_carrier(&self, news: &[RawTask], old_id: &[u8]) -> Result<()>;
}

pub struct TaskServiceImpl;

impl TaskServiceTrait for TaskServiceImpl {
    /// by performance reason, for one-to-one carry we can reuse the beginning carry to finish all flows.
    /// That way we need not to communicate with DB for create new and delete old carrier.
    /// But for failure we must redo from beginning. but I think it has small chance.
    /// Another disadvantage is the failure information will be attached to the beginning.
    fn create_and_finish_carrier(&self, old: &RawTask, new: &mut RawTask) -> Result<usize> {
        // TODO  当遇到错误时如果要结束的 delivery ID 和新的delivery 不一样 需要结束之前的 delivery 并创建新的 delivery
        new.task_id = old.task_id.clone(); // the id is used for final finished
        Ok(1)
    }

    fn create_batch_and_finish_carrier(&self, news: &[RawTask], old_id: &[u8]) -> Result<()> {
        for v in news {
            TaskDaoImpl::insert(v)?;
        }
        TaskDaoImpl::delete(old_id)?;
        Ok(())
    }
}
