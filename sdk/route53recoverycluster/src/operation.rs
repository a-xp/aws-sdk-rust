// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `GetRoutingControlState`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_routing_control_state`](crate::client::Client::get_routing_control_state).
///
/// See [`crate::client::fluent_builders::GetRoutingControlState`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRoutingControlState {
    _private: (),
}
impl GetRoutingControlState {
    /// Creates a new builder-style object to manufacture [`GetRoutingControlStateInput`](crate::input::GetRoutingControlStateInput)
    pub fn builder() -> crate::input::get_routing_control_state_input::Builder {
        crate::input::get_routing_control_state_input::Builder::default()
    }
    /// Creates a new `GetRoutingControlState` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRoutingControlState {
    type Output = std::result::Result<
        crate::output::GetRoutingControlStateOutput,
        crate::error::GetRoutingControlStateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_routing_control_state_error(response)
        } else {
            crate::operation_deser::parse_get_routing_control_state_response(response)
        }
    }
}

/// Operation shape for `UpdateRoutingControlState`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_routing_control_state`](crate::client::Client::update_routing_control_state).
///
/// See [`crate::client::fluent_builders::UpdateRoutingControlState`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRoutingControlState {
    _private: (),
}
impl UpdateRoutingControlState {
    /// Creates a new builder-style object to manufacture [`UpdateRoutingControlStateInput`](crate::input::UpdateRoutingControlStateInput)
    pub fn builder() -> crate::input::update_routing_control_state_input::Builder {
        crate::input::update_routing_control_state_input::Builder::default()
    }
    /// Creates a new `UpdateRoutingControlState` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateRoutingControlState {
    type Output = std::result::Result<
        crate::output::UpdateRoutingControlStateOutput,
        crate::error::UpdateRoutingControlStateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_routing_control_state_error(response)
        } else {
            crate::operation_deser::parse_update_routing_control_state_response(response)
        }
    }
}

/// Operation shape for `UpdateRoutingControlStates`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_routing_control_states`](crate::client::Client::update_routing_control_states).
///
/// See [`crate::client::fluent_builders::UpdateRoutingControlStates`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRoutingControlStates {
    _private: (),
}
impl UpdateRoutingControlStates {
    /// Creates a new builder-style object to manufacture [`UpdateRoutingControlStatesInput`](crate::input::UpdateRoutingControlStatesInput)
    pub fn builder() -> crate::input::update_routing_control_states_input::Builder {
        crate::input::update_routing_control_states_input::Builder::default()
    }
    /// Creates a new `UpdateRoutingControlStates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateRoutingControlStates {
    type Output = std::result::Result<
        crate::output::UpdateRoutingControlStatesOutput,
        crate::error::UpdateRoutingControlStatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_routing_control_states_error(response)
        } else {
            crate::operation_deser::parse_update_routing_control_states_response(response)
        }
    }
}