use yazi_fs::Filter;
use yazi_proxy::ManagerProxy;
use yazi_shared::render;

use super::filter::Opt;
use crate::tab::Tab;

impl Tab {
	#[yazi_macro::command]
	pub fn filter_do(&mut self, opt: Opt) {
		let filter = if opt.query.is_empty() {
			None
		} else if let Ok(f) = Filter::new(&opt.query, opt.case) {
			Some(f)
		} else {
			return;
		};

		if opt.done {
			ManagerProxy::update_paged(); // Update for paged files in next loop
		}

		let hovered = self.current.hovered().map(|f| f.urn_owned());
		if !self.current.files.set_filter(filter) {
			return;
		}

		self.current.repos(hovered.as_ref().map(|u| u.as_urn()));
		if self.current.hovered().map(|f| f.urn()) != hovered.as_ref().map(|u| u.as_urn()) {
			ManagerProxy::hover(None, self.idx);
		}

		render!();
	}
}