// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datablocks::*;
use common_datavalues::prelude::*;
use common_exception::Result;

#[test]
fn test_data_block_take() -> Result<()> {
    let schema = DataSchemaRefExt::create(vec![
        DataField::new("a", DataType::Int64, false),
        DataField::new("b", DataType::String, false),
    ]);

    let raw = DataBlock::create_by_array(schema, vec![
        Series::new(vec![1i64, 2, 3]),
        Series::new(vec!["b1", "b2", "b3"]),
    ]);

    let take = DataBlock::block_take_by_indices(&raw, &[], &[0, 2])?;
    assert_eq!(raw.schema(), take.schema());

    let expected = vec![
        "+---+----+",
        "| a | b  |",
        "+---+----+",
        "| 1 | b1 |",
        "| 3 | b3 |",
        "+---+----+",
    ];
    common_datablocks::assert_blocks_eq(expected, &[take]);

    Ok(())
}
