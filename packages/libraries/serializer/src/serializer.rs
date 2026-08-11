use crate::{Error, binding_power::GetBindingPower};

pub(crate) struct Serializer {
    inner: String,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    pub fn write(&mut self, string: &str) {
        self.inner.push_str(string);
    }

    pub fn write_spaced(&mut self, string: &str) {
        self.inner.push(' ');
        self.inner.push_str(string);
        self.inner.push(' ');
    }

    pub fn write_spaced_after(&mut self, string: &str) {
        self.inner.push_str(string);
        self.inner.push(' ');
    }

    pub fn write_parenthesized(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.write("(");
        f(self)?;
        self.write(")");
        Ok(())
    }

    pub fn write_parenthesized_if(
        &mut self,
        needs_parenthesis: bool,
        f: impl FnOnce(&mut Self) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if needs_parenthesis {
            self.write_parenthesized(f)
        } else {
            f(self)
        }
    }

    pub fn write_parenthesized_if_outer_binding_power_stronger(
        &mut self,
        parent: &impl GetBindingPower,
        child: &impl GetBindingPower,
        f: impl FnOnce(&mut Self) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let needs_parenthesis =
            match (parent.get_binding_power(), child.get_binding_power()) {
                (Some(parent), Some(child)) => parent > child,
                _ => true,
            };
        self.write_parenthesized_if(needs_parenthesis, f)
    }

    pub fn write_parenthesized_if_outer_binding_power_stronger_or_equal(
        &mut self,
        parent: &impl GetBindingPower,
        child: &impl GetBindingPower,
        f: impl FnOnce(&mut Self) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let needs_parenthesis =
            match (parent.get_binding_power(), child.get_binding_power()) {
                (Some(parent), Some(child)) => parent >= child,
                _ => true,
            };
        self.write_parenthesized_if(needs_parenthesis, f)
    }

    pub fn complete(self) -> String {
        self.inner
    }
}
