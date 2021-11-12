// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateScalingPlanOutput {}
impl std::fmt::Debug for UpdateScalingPlanOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateScalingPlanOutput");
        formatter.finish()
    }
}
/// See [`UpdateScalingPlanOutput`](crate::output::UpdateScalingPlanOutput)
pub mod update_scaling_plan_output {
    /// A builder for [`UpdateScalingPlanOutput`](crate::output::UpdateScalingPlanOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateScalingPlanOutput`](crate::output::UpdateScalingPlanOutput)
        pub fn build(self) -> crate::output::UpdateScalingPlanOutput {
            crate::output::UpdateScalingPlanOutput {}
        }
    }
}
impl UpdateScalingPlanOutput {
    /// Creates a new builder-style object to manufacture [`UpdateScalingPlanOutput`](crate::output::UpdateScalingPlanOutput)
    pub fn builder() -> crate::output::update_scaling_plan_output::Builder {
        crate::output::update_scaling_plan_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetScalingPlanResourceForecastDataOutput {
    /// <p>The data points to return.</p>
    pub datapoints: std::option::Option<std::vec::Vec<crate::model::Datapoint>>,
}
impl GetScalingPlanResourceForecastDataOutput {
    /// <p>The data points to return.</p>
    pub fn datapoints(&self) -> std::option::Option<&[crate::model::Datapoint]> {
        self.datapoints.as_deref()
    }
}
impl std::fmt::Debug for GetScalingPlanResourceForecastDataOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetScalingPlanResourceForecastDataOutput");
        formatter.field("datapoints", &self.datapoints);
        formatter.finish()
    }
}
/// See [`GetScalingPlanResourceForecastDataOutput`](crate::output::GetScalingPlanResourceForecastDataOutput)
pub mod get_scaling_plan_resource_forecast_data_output {
    /// A builder for [`GetScalingPlanResourceForecastDataOutput`](crate::output::GetScalingPlanResourceForecastDataOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) datapoints: std::option::Option<std::vec::Vec<crate::model::Datapoint>>,
    }
    impl Builder {
        /// Appends an item to `datapoints`.
        ///
        /// To override the contents of this collection use [`set_datapoints`](Self::set_datapoints).
        ///
        /// <p>The data points to return.</p>
        pub fn datapoints(mut self, input: impl Into<crate::model::Datapoint>) -> Self {
            let mut v = self.datapoints.unwrap_or_default();
            v.push(input.into());
            self.datapoints = Some(v);
            self
        }
        /// <p>The data points to return.</p>
        pub fn set_datapoints(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Datapoint>>,
        ) -> Self {
            self.datapoints = input;
            self
        }
        /// Consumes the builder and constructs a [`GetScalingPlanResourceForecastDataOutput`](crate::output::GetScalingPlanResourceForecastDataOutput)
        pub fn build(self) -> crate::output::GetScalingPlanResourceForecastDataOutput {
            crate::output::GetScalingPlanResourceForecastDataOutput {
                datapoints: self.datapoints,
            }
        }
    }
}
impl GetScalingPlanResourceForecastDataOutput {
    /// Creates a new builder-style object to manufacture [`GetScalingPlanResourceForecastDataOutput`](crate::output::GetScalingPlanResourceForecastDataOutput)
    pub fn builder() -> crate::output::get_scaling_plan_resource_forecast_data_output::Builder {
        crate::output::get_scaling_plan_resource_forecast_data_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeScalingPlansOutput {
    /// <p>Information about the scaling plans.</p>
    pub scaling_plans: std::option::Option<std::vec::Vec<crate::model::ScalingPlan>>,
    /// <p>The token required to get the next set of results. This value is <code>null</code> if
    /// there are no more results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeScalingPlansOutput {
    /// <p>Information about the scaling plans.</p>
    pub fn scaling_plans(&self) -> std::option::Option<&[crate::model::ScalingPlan]> {
        self.scaling_plans.as_deref()
    }
    /// <p>The token required to get the next set of results. This value is <code>null</code> if
    /// there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeScalingPlansOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeScalingPlansOutput");
        formatter.field("scaling_plans", &self.scaling_plans);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeScalingPlansOutput`](crate::output::DescribeScalingPlansOutput)
pub mod describe_scaling_plans_output {
    /// A builder for [`DescribeScalingPlansOutput`](crate::output::DescribeScalingPlansOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) scaling_plans: std::option::Option<std::vec::Vec<crate::model::ScalingPlan>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `scaling_plans`.
        ///
        /// To override the contents of this collection use [`set_scaling_plans`](Self::set_scaling_plans).
        ///
        /// <p>Information about the scaling plans.</p>
        pub fn scaling_plans(mut self, input: impl Into<crate::model::ScalingPlan>) -> Self {
            let mut v = self.scaling_plans.unwrap_or_default();
            v.push(input.into());
            self.scaling_plans = Some(v);
            self
        }
        /// <p>Information about the scaling plans.</p>
        pub fn set_scaling_plans(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ScalingPlan>>,
        ) -> Self {
            self.scaling_plans = input;
            self
        }
        /// <p>The token required to get the next set of results. This value is <code>null</code> if
        /// there are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token required to get the next set of results. This value is <code>null</code> if
        /// there are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeScalingPlansOutput`](crate::output::DescribeScalingPlansOutput)
        pub fn build(self) -> crate::output::DescribeScalingPlansOutput {
            crate::output::DescribeScalingPlansOutput {
                scaling_plans: self.scaling_plans,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeScalingPlansOutput {
    /// Creates a new builder-style object to manufacture [`DescribeScalingPlansOutput`](crate::output::DescribeScalingPlansOutput)
    pub fn builder() -> crate::output::describe_scaling_plans_output::Builder {
        crate::output::describe_scaling_plans_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeScalingPlanResourcesOutput {
    /// <p>Information about the scalable resources.</p>
    pub scaling_plan_resources:
        std::option::Option<std::vec::Vec<crate::model::ScalingPlanResource>>,
    /// <p>The token required to get the next set of results. This value is <code>null</code> if
    /// there are no more results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeScalingPlanResourcesOutput {
    /// <p>Information about the scalable resources.</p>
    pub fn scaling_plan_resources(
        &self,
    ) -> std::option::Option<&[crate::model::ScalingPlanResource]> {
        self.scaling_plan_resources.as_deref()
    }
    /// <p>The token required to get the next set of results. This value is <code>null</code> if
    /// there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeScalingPlanResourcesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeScalingPlanResourcesOutput");
        formatter.field("scaling_plan_resources", &self.scaling_plan_resources);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeScalingPlanResourcesOutput`](crate::output::DescribeScalingPlanResourcesOutput)
pub mod describe_scaling_plan_resources_output {
    /// A builder for [`DescribeScalingPlanResourcesOutput`](crate::output::DescribeScalingPlanResourcesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) scaling_plan_resources:
            std::option::Option<std::vec::Vec<crate::model::ScalingPlanResource>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `scaling_plan_resources`.
        ///
        /// To override the contents of this collection use [`set_scaling_plan_resources`](Self::set_scaling_plan_resources).
        ///
        /// <p>Information about the scalable resources.</p>
        pub fn scaling_plan_resources(
            mut self,
            input: impl Into<crate::model::ScalingPlanResource>,
        ) -> Self {
            let mut v = self.scaling_plan_resources.unwrap_or_default();
            v.push(input.into());
            self.scaling_plan_resources = Some(v);
            self
        }
        /// <p>Information about the scalable resources.</p>
        pub fn set_scaling_plan_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ScalingPlanResource>>,
        ) -> Self {
            self.scaling_plan_resources = input;
            self
        }
        /// <p>The token required to get the next set of results. This value is <code>null</code> if
        /// there are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token required to get the next set of results. This value is <code>null</code> if
        /// there are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeScalingPlanResourcesOutput`](crate::output::DescribeScalingPlanResourcesOutput)
        pub fn build(self) -> crate::output::DescribeScalingPlanResourcesOutput {
            crate::output::DescribeScalingPlanResourcesOutput {
                scaling_plan_resources: self.scaling_plan_resources,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeScalingPlanResourcesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeScalingPlanResourcesOutput`](crate::output::DescribeScalingPlanResourcesOutput)
    pub fn builder() -> crate::output::describe_scaling_plan_resources_output::Builder {
        crate::output::describe_scaling_plan_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteScalingPlanOutput {}
impl std::fmt::Debug for DeleteScalingPlanOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteScalingPlanOutput");
        formatter.finish()
    }
}
/// See [`DeleteScalingPlanOutput`](crate::output::DeleteScalingPlanOutput)
pub mod delete_scaling_plan_output {
    /// A builder for [`DeleteScalingPlanOutput`](crate::output::DeleteScalingPlanOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteScalingPlanOutput`](crate::output::DeleteScalingPlanOutput)
        pub fn build(self) -> crate::output::DeleteScalingPlanOutput {
            crate::output::DeleteScalingPlanOutput {}
        }
    }
}
impl DeleteScalingPlanOutput {
    /// Creates a new builder-style object to manufacture [`DeleteScalingPlanOutput`](crate::output::DeleteScalingPlanOutput)
    pub fn builder() -> crate::output::delete_scaling_plan_output::Builder {
        crate::output::delete_scaling_plan_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateScalingPlanOutput {
    /// <p>The version number of the scaling plan. This value is always <code>1</code>. Currently,
    /// you cannot have multiple scaling plan versions.</p>
    pub scaling_plan_version: std::option::Option<i64>,
}
impl CreateScalingPlanOutput {
    /// <p>The version number of the scaling plan. This value is always <code>1</code>. Currently,
    /// you cannot have multiple scaling plan versions.</p>
    pub fn scaling_plan_version(&self) -> std::option::Option<i64> {
        self.scaling_plan_version
    }
}
impl std::fmt::Debug for CreateScalingPlanOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateScalingPlanOutput");
        formatter.field("scaling_plan_version", &self.scaling_plan_version);
        formatter.finish()
    }
}
/// See [`CreateScalingPlanOutput`](crate::output::CreateScalingPlanOutput)
pub mod create_scaling_plan_output {
    /// A builder for [`CreateScalingPlanOutput`](crate::output::CreateScalingPlanOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) scaling_plan_version: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The version number of the scaling plan. This value is always <code>1</code>. Currently,
        /// you cannot have multiple scaling plan versions.</p>
        pub fn scaling_plan_version(mut self, input: i64) -> Self {
            self.scaling_plan_version = Some(input);
            self
        }
        /// <p>The version number of the scaling plan. This value is always <code>1</code>. Currently,
        /// you cannot have multiple scaling plan versions.</p>
        pub fn set_scaling_plan_version(mut self, input: std::option::Option<i64>) -> Self {
            self.scaling_plan_version = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateScalingPlanOutput`](crate::output::CreateScalingPlanOutput)
        pub fn build(self) -> crate::output::CreateScalingPlanOutput {
            crate::output::CreateScalingPlanOutput {
                scaling_plan_version: self.scaling_plan_version,
            }
        }
    }
}
impl CreateScalingPlanOutput {
    /// Creates a new builder-style object to manufacture [`CreateScalingPlanOutput`](crate::output::CreateScalingPlanOutput)
    pub fn builder() -> crate::output::create_scaling_plan_output::Builder {
        crate::output::create_scaling_plan_output::Builder::default()
    }
}