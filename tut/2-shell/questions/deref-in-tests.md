The Deref trait is used wherever the StackVec is dereferenced using array notation (for example stack_vec[0]).

Tests: index_oob(), index_oob_after_truncate(), indexing()

The DerefMut trait is used whenever the StackVec is dereferenced and its contents are mutated

Tests: mut_indexing()