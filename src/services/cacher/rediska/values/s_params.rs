use crate::forms::searcher::s_params::SearchParams;

use redis::{RedisWrite, ToRedisArgs};
use serde_derive::Serialize;

#[derive(Serialize)]
pub(crate) struct CacherSearchParams {
    search_params: SearchParams,
}

impl From<&SearchParams> for CacherSearchParams {
    fn from(value: &SearchParams) -> Self {
        CacherSearchParams {
            search_params: value.to_owned(),
        }
    }
}

impl From<CacherSearchParams> for SearchParams {
    fn from(value: CacherSearchParams) -> SearchParams {
        value.search_params
    }
}

impl ToRedisArgs for CacherSearchParams {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let json_str = serde_json::to_string(&self.search_params).unwrap();
        out.write_arg_fmt(json_str)
    }
}
