use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::{utils, view, PODMAN};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/marhkb/Symphony/ui/info-panel.ui")]
    pub(crate) struct InfoPanel {
        #[template_child]
        pub(super) header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub(super) content_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub(super) spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub(super) preferences_page: TemplateChild<adw::PreferencesPage>,
        #[template_child]
        pub(super) general_api_version_row: TemplateChild<view::PropertyRow>,
        #[template_child]
        pub(super) general_built_time_row: TemplateChild<view::PropertyRow>,
        #[template_child]
        pub(super) general_git_commit_row: TemplateChild<view::PropertyRow>,
        #[template_child]
        pub(super) general_go_version_row: TemplateChild<view::PropertyRow>,
        #[template_child]
        pub(super) general_os_row: TemplateChild<view::PropertyRow>,
        #[template_child]
        pub(super) general_arch_row: TemplateChild<view::PropertyRow>,
        #[template_child]
        pub(super) general_version_row: TemplateChild<view::PropertyRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InfoPanel {
        const NAME: &'static str = "InfoPanel";
        type Type = super::InfoPanel;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InfoPanel {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.setup();
        }

        fn dispose(&self, _obj: &Self::Type) {
            self.header_bar.unparent();
            self.content_box.unparent();
        }
    }
    impl WidgetImpl for InfoPanel {}
}

glib::wrapper! {
    pub(crate) struct InfoPanel(ObjectSubclass<imp::InfoPanel>)
        @extends gtk::Widget;
}

impl Default for InfoPanel {
    fn default() -> Self {
        glib::Object::new(&[]).expect("Failed to create InfoPanel")
    }
}

impl InfoPanel {
    pub(crate) fn setup(&self) {
        utils::do_async(
            PODMAN.version(),
            clone!(@weak self as obj => move |result| match result {
                Ok(version) => {
                    let imp = obj.imp();

                    imp.spinner.set_visible(false);
                    imp.preferences_page.set_visible(true);

                    imp.general_api_version_row
                        .set_value(&utils::format_option(version.api_version));
                    imp.general_built_time_row
                        .set_value(&utils::format_option(version.build_time));
                    imp.general_git_commit_row.set_value(&utils::format_option(
                        version
                            .git_commit
                            .and_then(|s| if s.is_empty() { None } else { Some(s) }),
                    ));
                    imp.general_go_version_row
                        .set_value(&utils::format_option(version.go_version));
                    imp.general_os_row
                        .set_value(&utils::format_option(version.os));
                    imp.general_arch_row
                        .set_value(&utils::format_option(version.arch));
                    imp.general_version_row
                        .set_value(&utils::format_option(version.version));
                }
                Err(e) => {
                    log::error!("Failed to retrieve host info: {e}");
                }
            }),
        )
    }
}
