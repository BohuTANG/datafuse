// Copyright 2022 Datafuse Labs.
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

use std::io::Write;

use goldenfile::Mint;

use super::run_ast;

#[test]
fn test_geo() {
    let mut mint = Mint::new("tests/it/scalars/testdata");
    let file = &mut mint.new_goldenfile("geo.txt").unwrap();

    test_great_circle_distance(file);
    test_geo_distance(file);
    test_great_circle_angle(file);
    test_point_in_ellipses(file);
}

fn test_great_circle_distance(file: &mut impl Write) {
    run_ast(
        file,
        "great_circle_distance(55.755831, 37.617673, -55.755831, -37.617673)",
        &[],
    );
}

fn test_geo_distance(file: &mut impl Write) {
    run_ast(
        file,
        "geo_distance(55.755831, 37.617673, -55.755831, -37.617673)",
        &[],
    );
}

fn test_great_circle_angle(file: &mut impl Write) {
    run_ast(file, "great_circle_angle(0, 0, 45, 0)", &[]);
}

fn test_point_in_ellipses(file: &mut impl Write) {
    run_ast(
        file,
        "point_in_ellipses(10., 10., 10., 9.1, 1., 0.9999)",
        &[],
    );
}
