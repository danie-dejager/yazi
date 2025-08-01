use anyhow::Result;
use yazi_core::mgr::Yanked;
use yazi_macro::{act, render};
use yazi_parser::mgr::YankOpt;
use yazi_shared::{event::Data, url::CovUrl};

use crate::{Actor, Ctx};

pub struct Yank;

impl Actor for Yank {
	type Options = YankOpt;

	const NAME: &str = "yank";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		act!(mgr:escape_visual, cx)?;

		cx.mgr.yanked =
			Yanked::new(opt.cut, cx.tab().selected_or_hovered().cloned().map(CovUrl).collect());
		render!(cx.mgr.yanked.catchup_revision(true));

		act!(mgr:escape_select, cx)
	}
}
