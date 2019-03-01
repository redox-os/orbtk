/// Used to define the column span of a widget on the `Grid`.
#[derive(Default, Copy, Clone, PartialEq)]
pub struct ColumnSpan(pub usize);

property!(ColumnSpan, ColumnSpanProperty, column_span, shared_column_span);