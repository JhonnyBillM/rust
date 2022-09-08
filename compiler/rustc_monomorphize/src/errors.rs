use std::path::PathBuf;

use rustc_errors::DiagnosticHandler;
use rustc_errors::ErrorGuaranteed;
use rustc_macros::{LintDiagnostic, DiagnosticHandler};
use rustc_span::Span;

#[derive(DiagnosticHandler)]
#[diag(monomorphize::recursion_limit)]
pub struct RecursionLimit {
    #[primary_span]
    pub span: Span,
    pub shrunk: String,
    #[note]
    pub def_span: Span,
    pub def_path_str: String,
    #[note(monomorphize::written_to_path)]
    pub was_written: Option<()>,
    pub path: PathBuf,
}

#[derive(DiagnosticHandler)]
#[diag(monomorphize::type_length_limit)]
#[help(monomorphize::consider_type_length_limit)]
pub struct TypeLengthLimit {
    #[primary_span]
    pub span: Span,
    pub shrunk: String,
    #[note(monomorphize::written_to_path)]
    pub was_written: Option<()>,
    pub path: PathBuf,
    pub type_length: usize,
}

#[derive(DiagnosticHandler)]
#[diag(monomorphize::requires_lang_item)]
pub struct RequiresLangItem {
    pub lang_item: String,
}

pub struct UnusedGenericParams {
    pub span: Span,
    pub param_spans: Vec<Span>,
    pub param_names: Vec<String>,
}

impl DiagnosticHandler<'_> for UnusedGenericParams {
    fn into_diagnostic(
        self,
        handler: &'_ rustc_errors::Handler,
    ) -> rustc_errors::DiagnosticBuilder<'_, ErrorGuaranteed> {
        let mut diag =
            handler.struct_err(rustc_errors::fluent::monomorphize::unused_generic_params);
        diag.set_span(self.span);
        for (span, name) in self.param_spans.into_iter().zip(self.param_names) {
            // FIXME: I can figure out how to do a label with a fluent string with a fixed message,
            // or a label with a dynamic value in a hard-coded string, but I haven't figured out
            // how to combine the two. 😢
            diag.span_label(span, format!("generic parameter `{}` is unused", name));
        }
        diag
    }
}

#[derive(LintDiagnostic)]
#[diag(monomorphize::large_assignments)]
#[note]
pub struct LargeAssignmentsLint {
    #[label]
    pub span: Span,
    pub size: u64,
    pub limit: u64,
}

#[derive(DiagnosticHandler)]
#[diag(monomorphize::unknown_partition_strategy)]
pub struct UnknownPartitionStrategy;

#[derive(DiagnosticHandler)]
#[diag(monomorphize::symbol_already_defined)]
pub struct SymbolAlreadyDefined {
    #[primary_span]
    pub span: Option<Span>,
    pub symbol: String,
}
