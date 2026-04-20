use std::{any::Any, collections::HashMap, str::FromStr};

use thiserror::Error;

/// # Variables
///
/// Variables stored in the request.
pub struct Variables {
    // should come from the slice of the request.
    route: HashMap<String, String>,
    // dynamically dispatched from the user, needs to be downcasted.
    trace_line: HashMap<String, Box<dyn Any>>,
}

#[derive(Debug, Error)]
pub enum VariableError {
    #[error("variable does not exist")]
    Missing,
    #[error("could not convert '{0}' to {1}")]
    CannotConvert(String, String),
}

impl Variables {

    pub fn new(route_vars: HashMap<String, String>) -> Self {
        Self {
            trace_line: HashMap::new(),
            route: route_vars
        }
    }

    /// # Get Route Variable
    ///
    /// Attempts to get a variable from the route parameters.
    ///
    /// If the variable does not exist than an error of `Missing` is returned.
    ///
    /// If the variable (`&'req str`) cannot be converted to a `T` then `CannotConvert` is returned.
    pub fn get_route_variable<T>(&self, item: &str) -> Result<T, VariableError>
    where
        T: FromStr,
    {
        let route_var = self.route.get(item).ok_or(VariableError::Missing)?;

        T::from_str(route_var).map_err(|_| {
            VariableError::CannotConvert(item.to_string(), std::any::type_name::<T>().to_string())
        })
    }

    /// # Get Variable
    ///
    /// Attempts to get and downcast the variable to <T> from the given request.
    ///
    ///
    pub fn get_variable<T>(&self, item: &str) -> Result<&T, VariableError>
    where
        T: 'static,
    {
        let var = self.trace_line.get(item).ok_or(VariableError::Missing)?;

        var.downcast_ref::<T>().ok_or(VariableError::CannotConvert(
            item.to_string(),
            std::any::type_name::<T>().to_owned(),
        ))
    }

    /// # Set Variable
    /// 
    /// Sets a variable (does not override route variables).
    pub fn set_variable<T>(&mut self, key: impl Into<String>, val: T) -> Option<Box<dyn Any>>
    where T: 'static {
        self.trace_line.insert(key.into(), Box::new(val))
    }
}
