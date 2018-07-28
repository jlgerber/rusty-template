
use FilterCallback;
use FilterHashMap;
use VarHashMap;
use errors::RustyTemplateError;
use Rule;
use parse;
use std::default::Default;

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
// pub fn hash(input: String) -> String {

// }


impl Default for TemplateParser {
    fn default() -> Self {
        let  fhm = FilterHashMap::new();
        let mut s = Self::new(fhm);
        s.add_filter("upper".to_string(), upper);
        s.add_filter("dot_to_slash".to_string(), dot_to_slash);
        s.add_filter("slash".to_string(), slash );
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