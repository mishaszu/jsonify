use std::cell::RefCell;
use std::collections::HashMap;

trait Jsonify {
    fn parse(&mut self, string: String) -> ();
}

enum JsonNode {
    ArrayNode(HashMap<String, Vec<JsonNode>>),
    ObjectNode(HashMap<String, JsonNode>),
    StringNode(HashMap<String, String>),
    NumberNode(HashMap<String, i32>),
    BooleanNode(HashMap<String, bool>),
    Nill,
}

struct JSON {
    lvl: u8,
    children: Box<JsonNode>,
}

impl Jsonify for JSON {
    fn parse(&mut self, string: String) -> () {
        let text =
            string
                .trim()
                .bytes()
                .fold(
                    (
                        true,
                        false,
                        false,
                        vec![] as Vec<(u8, RefCell<Vec<u8>>, RefCell<Vec<u8>>)>,
                        0 as u8,
                        false,
                    ),
                    |acc, x| {
                        let (
                            mut if_left,
                            mut if_value,
                            mut if_arr,
                            mut str_arr,
                            mut lvl,
                            mut next_lvl,
                        ) = acc;
                        let (_, left_vec, right_vec) = match str_arr.last() {
                            Some(x) => x,
                            None => {
                                str_arr.push((lvl, RefCell::new(vec![]), RefCell::new(vec![])));
                                &str_arr[str_arr.len() - 1]
                            }
                        };
                        match x {
                            10 => (),
                            32 => {
                                if (if_value) {
                                    // should throw error
                                    // if (if_left) {
                                    // }
                                    right_vec.borrow_mut().push(x);
                                }
                            }
                            34 => {
                                if_value = !if_value;
                                if next_lvl {
                                    next_lvl = false;
                                    str_arr.push((lvl, RefCell::new(vec![]), RefCell::new(vec![])));
                                }
                            }
                            44 => {
                                if if_value {
                                    right_vec.borrow_mut().push(x);
                                } else {
                                    if_left = true;
                                    next_lvl = true;
                                }
                            }
                            58 => if_left = false,
                            91 | 123 => {
                                if if_value {
                                    right_vec.borrow_mut().push(x);
                                } else {
                                    lvl += 1;
                                    if_left = true;
                                }
                                next_lvl = true;
                            }
                            93 | 125 => {
                                if if_value {
                                    right_vec.borrow_mut().push(x);
                                } else {
                                    lvl -= 1;
                                    if_left = true;
                                }
                                next_lvl = true;
                            }
                            _ => {
                                if if_left {
                                    left_vec.borrow_mut().push(x);
                                } else {
                                    right_vec.borrow_mut().push(x);
                                }
                            }
                        }
                        (if_left, if_value, if_arr, str_arr, lvl, next_lvl)
                    },
                )
                .3;
        let json: Vec<(u8, String, String)> = text
            .into_iter()
            .map(|x| {
                (
                    x.0,
                    std::str::from_utf8(&x.1.into_inner()).unwrap().to_string(),
                    std::str::from_utf8(&x.2.into_inner()).unwrap().to_string(),
                )
            })
            .collect();
        println!("{:?}", json);
        // let lines = lines
        // .bytes()
        // .fold()
    }
}

#[cfg(test)]
mod json_test {
    use crate::json::*;
    #[test]
    fn print_x_value() {
        let text = String::from(
            "{
            \"test1\": \"test\",
             \"test2\": \"test test\"   ,
             \"test3\": \"testtest test\",
             \"test4\": {
                \"test5\": true
             },
             \"test6\": \"test on lvl 1\"
        }",
        );
        let mut j1 = JSON {
            lvl: 0,
            children: Box::new(JsonNode::Nill),
        };
        j1.parse(text);
    }
}
