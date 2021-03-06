use super::Token;
use crate::api::schema::{ContestId, RunId};
use snafu::Snafu;

/// Access check service
#[derive(Copy, Clone)]
pub(crate) struct RawAccessChecker<'a> {
    pub(crate) token: &'a Token,
    pub(crate) cfg: &'a cfg::Config,
    pub(crate) db: &'a dyn db::DbConn,
}

impl<'a> RawAccessChecker<'a> {
    fn wrap<T>(&self, obj: T) -> AccessChecker<'a, T> {
        AccessChecker { raw: *self, obj }
    }

    pub(crate) fn wrap_contest(
        &self,
        contest_id: ContestId,
    ) -> AccessChecker<'a, ack_subject::Contest> {
        self.wrap(ack_subject::Contest(contest_id))
    }

    pub(crate) fn wrap_run(&self, run_id: RunId) -> AccessChecker<'a, ack_subject::Run> {
        self.wrap(ack_subject::Run(run_id))
    }

    pub(crate) fn is_sudo(&self) -> AccessResult {
        // When namespaces are introduced, this function will account for that
        Ok(self.token.user_info.name == "Global/Root")
    }
}

pub(crate) mod ack_subject {
    use super::*;
    pub(crate) struct Contest(pub(super) ContestId);

    pub(crate) struct Run(pub(super) RunId);
}

pub(crate) struct AccessChecker<'a, T> {
    raw: RawAccessChecker<'a>,
    obj: T,
}

#[derive(Debug, Snafu)]
pub(crate) enum AccessCheckError {
    NotFound,
    Db { source: db::Error },
}

impl From<db::Error> for AccessCheckError {
    fn from(source: db::Error) -> Self {
        Self::Db { source }
    }
}

pub(crate) type AccessResult = Result<bool, AccessCheckError>;

impl AccessChecker<'_, ack_subject::Run> {
    fn for_contest(&self) -> AccessChecker<ack_subject::Contest> {
        self.raw.wrap_contest(
            // TODO: correctly determine contest_id here
            "TODO".to_string(),
        )
    }

    pub(crate) fn can_modify_run(&self) -> AccessResult {
        if self.for_contest().is_contest_sudo()? {
            return Ok(true);
        }
        let run = self.raw.db.run_load(self.obj.0)?;

        Ok(run.user_id == self.raw.token.user_id())
    }
}

impl AccessChecker<'_, ack_subject::Contest> {
    pub(crate) fn can_submit(&self) -> AccessResult {
        let contest = self
            .raw
            .cfg
            .find_contest(&self.obj.0)
            .ok_or(AccessCheckError::NotFound)?;
        if self.is_contest_sudo()? {
            return Ok(true);
        }
        for registered_group in &contest.group {
            if self.raw.token.user_info.groups.contains(registered_group) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn is_contest_sudo(&self) -> AccessResult {
        if self.raw.is_sudo()? {
            return Ok(true);
        }
        let contest = self
            .raw
            .cfg
            .find_contest(&self.obj.0)
            .ok_or(AccessCheckError::NotFound)?;
        for judges_group in &contest.judges {
            if self.raw.token.user_info.groups.contains(judges_group) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(crate) fn select_judge_log_kind(
        &self,
    ) -> Result<invoker_api::valuer_proto::JudgeLogKind, AccessCheckError> {
        use invoker_api::valuer_proto::JudgeLogKind;
        if self.is_contest_sudo()? {
            return Ok(JudgeLogKind::Full);
        }
        Ok(JudgeLogKind::Contestant)
    }
}
