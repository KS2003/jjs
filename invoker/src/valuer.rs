use crate::{
    inter_api::{ValuerNotification, ValuerResponse},
    invoke_context::InvokeContext,
};

pub(crate) struct Valuer<'a> {
    ctx: InvokeContext<'a>,
    cnt: u32,
}

impl<'a> Valuer<'a> {
    pub(crate) fn new(ctx: InvokeContext<'a>) -> Valuer {
        Valuer { ctx, cnt: 0 }
    }

    pub(crate) fn initial_test(&mut self) -> ValuerResponse {
        ValuerResponse::Test { test_id: 1 }
    }

    pub(crate) fn notify_test_done(&mut self, notification: ValuerNotification) -> ValuerResponse {
        self.cnt += 1;
        let tid = self.cnt + 1;
        let is_succ = notification.test_status.kind == invoker_api::StatusKind::Accepted;
        if tid as usize <= self.ctx.req.problem.tests.len() && is_succ {
            ValuerResponse::Test { test_id: tid }
        } else {
            ValuerResponse::Finish {
                score: if is_succ { 100 } else { 0 },
            }
        }
    }
}
