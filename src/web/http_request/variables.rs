//! # Variables
//!
//! Request-scoped variable store attached to each [`HttpRequest`]. Holds the
//! route parameters captured by the router and a "trace line" of
//! arbitrary, dynamically-typed values that middleware and handlers can
//! set and read back across the request pipeline.

use std::{any::Any, collections::HashMap, str::FromStr};

use thiserror::Error;

use crate::RequestError;

/// # Variables
///
/// Variables stored in the request.
pub struct Variables {
    // should come from the slice of the request.
    route: HashMap<String, String>,
    // dynamically dispatched from the user, needs to be downcasted.
    trace_line: HashMap<String, Box<dyn Any>>,
}

/// String identifying the destination type name in a failed
/// [`VariableError::CannotConvert`].
pub type ConvertTypeName = String;
/// String holding the source value that could not be converted in a failed
/// [`VariableError::CannotConvert`].
pub type ConvertValue = String;

/// # VariableError
///
/// Errors that can occur when reading a variable from a [`Variables`]
/// store.
#[derive(Debug, Error)]
pub enum VariableError {
    #[error("variable does not exist")]
    Missing,
    #[error("could not convert '{0}' to {1}")]
    CannotConvert(ConvertValue, ConvertTypeName),
}

/// Converts a [`VariableError`] into a [`RequestError`]. `Missing` maps to
/// a 404 and `CannotConvert` maps to a 500; the original error message is
/// preserved.
impl From<VariableError> for RequestError {
    fn from(value: VariableError) -> Self {
        let mut req_e = RequestError::default();

        let code = match &value {
            VariableError::Missing => 404,
            VariableError::CannotConvert(_, _) => 500,
        };

        req_e.set_message(value.to_string());
        req_e.set_status(code);

        req_e
    }
}

impl Variables {
    /// Creates a new [`Variables`] store seeded with the route parameters
    /// captured by the router. The trace-line map starts empty.
    pub fn new(route_vars: HashMap<String, String>) -> Self {
        Self {
            trace_line: HashMap::new(),
            route: route_vars,
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
    where
        T: 'static,
    {
        self.trace_line.insert(key.into(), Box::new(val))
    }
}
