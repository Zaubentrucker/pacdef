/// Used to implement parts of the `Backend` trait that should not change between the actual
/// backends (boilerplate).
#[macro_export]
macro_rules! impl_backend_constants {
    () => {
        fn get_binary_default(&self) -> Text {
            BINARY
        }

        fn get_section(&self) -> Text {
            SECTION
        }

        fn get_switches_info(&self) -> Switches {
            SWITCHES_INFO
        }

        fn get_switches_install(&self) -> Switches {
            SWITCHES_INSTALL
        }

        fn get_switches_noconfirm(&self) -> Switches {
            SWITCHES_NOCONFIRM
        }

        fn get_switches_remove(&self) -> Switches {
            SWITCHES_REMOVE
        }

        fn get_switches_make_dependency(&self) -> Switches {
            SWITCHES_MAKE_DEPENDENCY
        }

        fn get_managed_packages(&self) -> &HashSet<Package> {
            &self.packages
        }

        fn load(&mut self, groups: &HashSet<Group>) {
            let own_section_name = self.get_section();

            groups
                .iter()
                .flat_map(|g| &g.sections)
                .filter(|section| section.name == own_section_name)
                .flat_map(|section| &section.packages)
                .for_each(|package| {
                    self.packages.insert(package.clone());
                })
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }

        fn supports_as_dependency(&self) -> bool {
            SUPPORTS_AS_DEPENDENCY
        }
    };
}
