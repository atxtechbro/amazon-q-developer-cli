// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CodeGenerationStatus {
    #[allow(missing_docs)] // documentation missing in model
    pub status: crate::types::CodeGenerationWorkflowStatus,
    #[allow(missing_docs)] // documentation missing in model
    pub current_stage: crate::types::CodeGenerationWorkflowStage,
}
impl CodeGenerationStatus {
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> &crate::types::CodeGenerationWorkflowStatus {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn current_stage(&self) -> &crate::types::CodeGenerationWorkflowStage {
        &self.current_stage
    }
}
impl CodeGenerationStatus {
    /// Creates a new builder-style object to manufacture
    /// [`CodeGenerationStatus`](crate::types::CodeGenerationStatus).
    pub fn builder() -> crate::types::builders::CodeGenerationStatusBuilder {
        crate::types::builders::CodeGenerationStatusBuilder::default()
    }
}

/// A builder for [`CodeGenerationStatus`](crate::types::CodeGenerationStatus).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CodeGenerationStatusBuilder {
    pub(crate) status: ::std::option::Option<crate::types::CodeGenerationWorkflowStatus>,
    pub(crate) current_stage: ::std::option::Option<crate::types::CodeGenerationWorkflowStage>,
}
impl CodeGenerationStatusBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn status(mut self, input: crate::types::CodeGenerationWorkflowStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::CodeGenerationWorkflowStatus>) -> Self {
        self.status = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<crate::types::CodeGenerationWorkflowStatus> {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn current_stage(mut self, input: crate::types::CodeGenerationWorkflowStage) -> Self {
        self.current_stage = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_current_stage(
        mut self,
        input: ::std::option::Option<crate::types::CodeGenerationWorkflowStage>,
    ) -> Self {
        self.current_stage = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_current_stage(&self) -> &::std::option::Option<crate::types::CodeGenerationWorkflowStage> {
        &self.current_stage
    }

    /// Consumes the builder and constructs a
    /// [`CodeGenerationStatus`](crate::types::CodeGenerationStatus). This method will fail if
    /// any of the following fields are not set:
    /// - [`status`](crate::types::builders::CodeGenerationStatusBuilder::status)
    /// - [`current_stage`](crate::types::builders::CodeGenerationStatusBuilder::current_stage)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::CodeGenerationStatus, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::types::CodeGenerationStatus {
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building CodeGenerationStatus",
                )
            })?,
            current_stage: self.current_stage.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "current_stage",
                    "current_stage was not specified but it is required when building CodeGenerationStatus",
                )
            })?,
        })
    }
}
