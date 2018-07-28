
extern crate seahash;
use errors::RustyTemplateError;
use FilterCallback;
use FilterHashMap;
use parse;
use Rule;
use std::default::Default;
use VarHashMap;

pub struct TemplateParser {
    filters: FilterHashMap
}

pub fn upper(input: String) -> String {
    input.to_uppercase()
}

pub fn dot_to_slash(input: String) -> String {
    input.replace(".", "/")
}

pub fn slash(input: String) -> String {
    format!("{}/", input)
}
 pub fn hash(input: String) -> String {
     seahash::hash(input.as_bytes()).to_string()
}

impl Default for TemplateParser {
    fn default() -> Self {
        let  fhm = FilterHashMap::new();
        let mut s = Self::new(fhm);
        s.add_filter("upper".to_string(), upper);
        s.add_filter("dot_to_slash".to_string(), dot_to_slash);
        s.add_filter("slash".to_string(), slash );
        s.add_filter("hash".to_string(), hash );

        s
    }
}

impl TemplateParser {
    pub fn new(filters: FilterHashMap) -> TemplateParser {
        TemplateParser {
            filters
        }
    }

    pub fn add_filter<I>(&mut self, key: I, value: FilterCallback) where I: Into<String> {
        self.filters.insert(key.into(), value);
    }

    /// Given a str template and a refrence to a hashmap storing the potential keys to look up
    /// from the template, generate a Result.
    pub fn parse(&self, template: &str, map: &VarHashMap)  -> Result<String, RustyTemplateError> {
        let pairs = parse(template)
        .or( Err(RustyTemplateError::PestError("unable to parse template".to_owned())) )?;
        // go through and substitute
        let mut result = String::new();
        for pair in pairs {
            let span = pair.clone().into_span();

            match pair.clone().as_rule() {
                Rule::path => {
                    result.push_str(span.as_str());
                },
                Rule::trans => {
                    println!("rule trans");
                    let mut transformed = String::new();
                    let mut cnt = 0;
                    let mut skip = false;
                    for inner_pair in pair.clone().into_inner() {
                        let inner_span = inner_pair.clone().into_span();
                        match inner_pair.as_rule() {
                            Rule::alpha | Rule::digit | Rule::word => {
                                if cnt == 0 {
                                    if let Some(ref value) = map.get(inner_span.as_str()) {
                                        transformed = value.to_string();
                                    } else {
                                        return Err(RustyTemplateError::PestError(format!("Rule:  unable to retrieve {} from map", inner_span.as_str())));
                                    }
                                } else {
                                    if skip {continue;}
                                    if  self.filters.contains_key(inner_span.as_str()) {
                                        transformed = self.filters[inner_span.as_str()](transformed) ;

                                    } else {
                                        return Err(RustyTemplateError::PestError(format!("unable to retrieve {} from filters", inner_span.as_str())));
                                    }
                                }
                                cnt += 1;
                            },
                            Rule::optword => {
                                let key = inner_span.as_str().trim_right_matches('?');
                                if let Some(ref value) = map.get(key) {
                                    transformed = value.to_string();
                                    cnt +=1;
                                } else {
                                    skip = true;
                                    break;
                                }
                            },
                            Rule::trans => {
                                transformed = self.filters[inner_span.as_str()](transformed) ;
                            },
                            _ => {
                                return Err(RustyTemplateError::PestError(format!("Unexpected rule in trans: {:?}", inner_pair.clone().as_rule() )))
                            }
                        }
                    }
                    if !skip {
                        result.push_str(&transformed);
                    }
                },
                Rule::var => {
                    println!(
                        "Rule::var"
                    );
                    for inner_pair in pair.clone().into_inner() {
                        let inner_span = inner_pair.clone().into_span();
                        match inner_pair.as_rule() {
                            Rule::alpha | Rule::digit | Rule::word => {
                                if let Some(ref val) = map.get(inner_span.as_str()) {
                                    println!("matched {}", val);
                                    result.push_str(val);
                                } else {
                                    return Err(RustyTemplateError::PestError(format!("unable to extract {} from map", inner_span.as_str())));
                                }
                            },
                            Rule::optword => {
                                let key = inner_span.as_str().trim_right_matches('?');
                                if let Some(ref val) = map.get(key) {
                                    println!("matched {}", val);
                                    result.push_str(val);
                                } else {
                                    // since we are an optword, we do nothing
                                }
                            },
                            _ => {
                                return Err(RustyTemplateError::PestError(format!("Unexpected rule in var: {:?}", inner_pair.clone().as_rule() )))
                            }
                        }
                    }
                },
                _ => {
                    return Err(RustyTemplateError::PestError(format!("Unexpected rule: {:?}", pair.clone().as_rule() )))
                }

            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use utils::s;

    fn setup_parser() -> (TemplateParser, VarHashMap) {
        let mut vhm = VarHashMap::new();
        vhm.insert(s("instance.level"), s("chickenking.rd.9999"));
        vhm.insert(s("context.department_code"), s("anim"));
        vhm.insert(s("context.lod_code"), s("hi"));
        vhm.insert(s("asset.name"), s("robot"));
        vhm.insert(s("instance.name"),s("1"));
        (TemplateParser::default(), vhm)
    }

    // Just the basic variable lookup
    #[test]
    fn var() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{asset.name}";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/robot"));
    }


    // Basic variable lookup with space in front of the key
    #[test]
    fn var_spaces_front() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{  asset.name}";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/robot"));
    }

    // Basic variable lookup with spaces after key
    #[test]
    fn var_spaces_rear() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{asset.name }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/robot"));
        let t = "/doo/da/{asset.name    }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/robot"));
    }

    // Basic variable lookup with spaces around key
    #[test]
    fn var_spaces() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{ asset.name }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/robot"));
        let t = "/doo/da/{  asset.name    }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/robot"));
    }


    // variable optword lookup with extant key
    #[test]
    fn var_optional_mit() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{ asset.name? }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/robot"));
    }

    // variable optword lookup with non-extant key
    #[test]
    fn var_optional_miss() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{ asset.namee? }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/"));
    }

    // multiple variables in row
    #[test]
    fn multi_var() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{context.department_code }/bla/{ asset.name }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/anim/bla/robot"));
    }

    // pipeline var lookup to upper function
    #[test]
    fn var_pipeline_upper() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{ asset.name | upper}";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/ROBOT"));
    }

    // pipeline var lookup to dot_2_slash function which should replace all
    // periods with slashes
    #[test]
    fn var_pipeline_dot2slash() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{ instance.level | dot_to_slash }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/chickenking/rd/9999"));
    }

    // Multiple pipelined function applications to the same variable.
    #[test]
    fn var_pipeline_multi() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{ asset.name | upper | slash}";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/ROBOT/"));
    }

    // pipeline with successful optword lookup
    #[test]
    fn var_pipeline_upper_optional() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{ asset.name? | upper}";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/ROBOT"));
    }

    // pipeline with failed optword lookup
    #[test]
    fn var_pipeline_optional_miss() {
        let (parser, map) = setup_parser();
        let t = "/doo/da/{ asset.namee? | upper}";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("/doo/da/"));
    }


    // variable optword lookup with hash
    #[test]
    fn var_pipeline_optional_hash() {
        let (parser, map) = setup_parser();
        let t = "{ asset.name? | hash }";
        let result = parser.parse(t, &map).unwrap();
        assert_eq!(result, s("15616689721111649596"));
    }
}