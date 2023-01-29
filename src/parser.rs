use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Explanation {
    query_block: QueryBlock,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QueryBlock {
    select_id: Option<u32>,
    cost_info: Option<QueryCost>,
    #[serde(flatten)]
    relation_block: Box<RelationBlock>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum AttachedBlock {
    #[serde(rename = "query_block")]
    QueryBlock(QueryBlock),
    #[serde(rename = "table")]
    Table(Table),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum RelationBlock {
    #[serde(rename = "duplicates_removal")]
    DuplicatesRemoval(DuplicatesRemoval),
    #[serde(rename = "grouping_operation")]
    GroupingOperation(GroupingOperation),
    #[serde(rename = "nested_loop")]
    NestedLoop(Vec<TableBlock>),
    #[serde(rename = "ordering_operation")]
    OrderingOperation(OrderingOperation),
    #[serde(rename = "table")]
    Table(Table),
    #[serde(rename = "union_result")]
    UnionResult(UnionResult),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum GroupedRelationBlock {
    #[serde(rename = "buffer_result")]
    BufferResult(BufferResult),
    #[serde(rename = "nested_loop")]
    NestedLoop(Vec<TableBlock>),
    #[serde(rename = "table")]
    Table(Table),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QueryCost {
    query_cost: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SortCost {
    sort_cost: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RelationalCosts {
    read_cost: String,
    eval_cost: String,
    prefix_cost: String,
    data_read_per_join: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TableBlock {
    table: Table,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UnionResult {
    using_temporary_table: Option<bool>,
    table_name: String,
    access_type: String,
    query_specifications: Vec<UnionBlock>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UnionBlock {
    dependent: bool,
    cacheable: bool,
    query_block: QueryBlock,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GroupingOperation {
    using_temporary_table: Option<bool>,
    using_filesort: bool,
    cost_info: Option<SortCost>,
    #[serde(flatten)]
    relation_block: Option<GroupedRelationBlock>,
    table: Option<Table>,
    having_subqueries: Option<Vec<AttachedSubquery>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderingOperation {
    using_temporary_table: Option<bool>,
    using_filesort: bool,
    #[serde(flatten)]
    relation_block: Option<Box<RelationBlock>>,
    optimized_away_subqueries: Option<Vec<AttachedSubquery>>,
    order_by_subqueries: Option<Vec<OrderBySubqueries>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BlockReference {
    select_id: u32,
    message: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderBySubqueries {
    dependent: bool,
    cacheable: bool,
    query_block: BlockReference,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BufferResult {
    using_temporary_table: bool,
    table: Table,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DuplicatesRemoval {
    using_temporary_table: bool,
    using_filesort: bool,
    #[serde(flatten)]
    relation_block: Box<RelationBlock>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Table {
    table_name: String,
    access_type: String,
    possible_keys: Option<Vec<String>>,
    key: Option<String>,
    used_key_parts: Option<Vec<String>>,
    key_length: Option<String>,
    #[serde(rename = "ref")]
    reference: Option<Vec<String>>,
    cost_info: Option<RelationalCosts>,
    rows_examined_per_scan: Option<u32>,
    rows_produced_per_join: Option<u32>,
    filtered: Option<String>,
    first_match: Option<String>,
    using_index_for_group_by: Option<bool>,
    index_condition: Option<String>,
    using_index: Option<bool>,
    used_columns: Option<Vec<String>>,
    attached_condition: Option<String>,
    attached_subqueries: Option<Vec<AttachedSubquery>>,
    materialized_from_subquery: Option<MaterializedFromSubquery>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MaterializedFromSubquery {
    using_temporary_table: bool,
    dependent: Option<bool>,
    cacheable: Option<bool>,
    query_block: QueryBlock,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AttachedSubquery {
    dependent: Option<bool>,
    cacheable: Option<bool>,
    #[serde(flatten)]
    attached_block: AttachedBlock,
}

#[cfg(test)]
mod tests {
    use crate::parser::Explanation;
    use std::fs::File;
    use std::io::{BufReader, Read};

    fn parse_explanation_file(file: &File) -> Explanation {
        let mut reader = BufReader::new(file);

        let mut json = String::new();

        let _ = reader.read_to_string(&mut json);
        let e: Explanation = serde_json::from_str(&json).unwrap();

        e
    }

    #[test]
    fn parses_attached_condition() {
        let _ = parse_explanation_file(&File::open("json/01_attached_condition.json").unwrap());
    }

    #[test]
    fn parses_filtering() {
        let _ = parse_explanation_file(&File::open("json/02_filtering.json").unwrap());
    }

    #[test]
    fn parses_parses_sed_key_parts() {
        let _ = parse_explanation_file(&File::open("json/04_used_key_parts.json").unwrap());
    }

    #[test]
    fn parses_subqueries() {
        let _ = parse_explanation_file(&File::open("json/05_subqueries.json").unwrap());
    }

    #[test]
    fn parses_materialized_from_subquery() {
        let _ =
            parse_explanation_file(&File::open("json/06_materialized_from_subquery.json").unwrap());
    }

    #[test]
    fn parses_subquery_sorting() {
        let _ = parse_explanation_file(&File::open("json/07_subquery_sorting.json").unwrap());
    }

    #[test]
    fn parses_subquery_optimized_away() {
        let _ = parse_explanation_file(&File::open("json/07_suquery_optimized_away.json").unwrap());
    }

    #[test]
    fn parses_grouping() {
        let _ = parse_explanation_file(&File::open("json/08_grouping.json").unwrap());
    }

    #[test]
    fn parses_grouping_duplicate_removal() {
        let _ =
            parse_explanation_file(&File::open("json/08_grouping_duplicate_removal.json").unwrap());
    }

    #[test]
    fn parses_order_by() {
        let _ = parse_explanation_file(&File::open("json/09_order_by.json").unwrap());
    }

    #[test]
    fn parses_having() {
        let _ = parse_explanation_file(&File::open("json/10_having.json").unwrap());
    }

    #[test]
    fn parses_unions() {
        let _ = parse_explanation_file(&File::open("json/11_unions.json").unwrap());
    }

    #[test]
    fn parses_buffering() {
        let _ = parse_explanation_file(&File::open("json/12_buffering.json").unwrap());
    }

    #[test]
    fn parses_indexing() {
        let _ = parse_explanation_file(&File::open("json/13_indexing.json").unwrap());
    }

    #[test]
    fn parses_nested_loops() {
        let _ = parse_explanation_file(&File::open("json/14_nested_loops.json").unwrap());
    }
}
