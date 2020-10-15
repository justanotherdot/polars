initSidebarItems({"fn":[["aligned_vec_to_primitive_array","Take an owned Vec that is 64 byte aligned and create a zero copy PrimitiveArray Can also take a null bit buffer into account."],["build_primitive_ca_with_opt",""],["build_with_existing_null_bitmap_and_slice","Take an existing slice and a null bitmap and construct an arrow array."],["get_bitmap","Get the null count and the null bitmap of the arrow array"],["get_large_list_builder",""],["round_upto_multiple_of_64","Returns the nearest number that is `>=` than `num` and is a multiple of 64"]],"mod":[["memory","Defines memory-related functions, such as allocate/deallocate/reallocate memory regions, cache and allocation alignments."]],"struct":[["AlignedVec","A `Vec` wrapper with a memory alignment equal to Arrow's primitive arrays. Can be useful in creating a new ChunkedArray or Arrow Primitive array without copying."],["LargeListPrimitiveChunkedBuilder",""],["LargeListUtf8ChunkedBuilder",""],["PrimitiveChunkedBuilder",""],["Utf8ChunkedBuilder",""]],"trait":[["LargListBuilderTrait",""],["NewChunkedArray",""]],"type":[["BooleanChunkedBuilder",""]]});